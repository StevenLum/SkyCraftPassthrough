//! Small Win32 boundary. All mapped-memory access is under the same named mutex.
use crate::{RECORD_SIZE, Record};
use std::{ffi::c_void, io, ptr};

type RawHandle = *mut c_void;
const WAIT_OBJECT_0: u32 = 0;
const WAIT_ABANDONED: u32 = 0x80;
const WAIT_TIMEOUT: u32 = 0x102;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateMutexW(attributes: *const c_void, owner: i32, name: *const u16) -> RawHandle;
    fn ReleaseMutex(handle: RawHandle) -> i32;
    fn WaitForSingleObject(handle: RawHandle, milliseconds: u32) -> u32;
    fn CreateFileMappingW(file: RawHandle, attributes: *const c_void, protection: u32,
        size_high: u32, size_low: u32, name: *const u16) -> RawHandle;
    fn MapViewOfFile(mapping: RawHandle, access: u32, offset_high: u32,
        offset_low: u32, bytes: usize) -> *mut c_void;
    fn UnmapViewOfFile(address: *const c_void) -> i32;
    fn CloseHandle(handle: RawHandle) -> i32;
    fn OpenProcess(access: u32, inherit: i32, pid: u32) -> RawHandle;
    fn GetTickCount64() -> u64;
}

struct Handle(RawHandle);
impl Handle {
    fn checked(raw: RawHandle) -> io::Result<Self> {
        if raw.is_null() { Err(io::Error::last_os_error()) } else { Ok(Self(raw)) }
    }
}
impl Drop for Handle {
    fn drop(&mut self) { unsafe { CloseHandle(self.0); } }
}

struct View(*mut c_void);
impl Drop for View {
    fn drop(&mut self) { unsafe { UnmapViewOfFile(self.0); } }
}

struct Lock<'a>(&'a Handle);
impl Drop for Lock<'_> {
    fn drop(&mut self) { unsafe { ReleaseMutex(self.0.0); } }
}

fn wide(text: &str) -> Vec<u16> { text.encode_utf16().chain(Some(0)).collect() }

pub(crate) fn now_ms() -> u64 {
    // Windows boot-relative time is shared by both processes and not wall-clock time.
    unsafe { GetTickCount64() }
}

pub(crate) fn process_alive(pid: u32) -> io::Result<bool> {
    let raw = unsafe { OpenProcess(0x0010_0000 /* SYNCHRONIZE */, 0, pid) };
    if raw.is_null() {
        let error = io::Error::last_os_error();
        return match error.raw_os_error() {
            Some(87) => Ok(false), // PID no longer exists.
            Some(5) => Ok(true),   // Access denied: conservatively retain ownership.
            _ => Err(error),
        };
    }
    let process = Handle(raw);
    match unsafe { WaitForSingleObject(process.0, 0) } {
        WAIT_OBJECT_0 => Ok(false),
        WAIT_TIMEOUT => Ok(true),
        _ => Err(io::Error::last_os_error()),
    }
}

pub(crate) struct SharedMemory {
    // Drop order: unmap the view, close mapping, then close the mutex.
    view: View,
    _mapping: Handle,
    mutex: Handle,
}

impl SharedMemory {
    pub(crate) fn open(session: &str) -> io::Result<Self> {
        Self::open_sized(session, RECORD_SIZE)
    }

    pub(crate) fn open_sized(session: &str, size: usize) -> io::Result<Self> {
        let prefix = format!("Local\\PassthroughPractice_link4_{session}");
        let mutex_name = wide(&format!("{prefix}_lock"));
        let map_name = wide(&format!("{prefix}_memory"));
        // Local namespace: same Windows login session, no administrator privileges.
        let mutex = Handle::checked(unsafe {
            CreateMutexW(ptr::null(), 0, mutex_name.as_ptr())
        })?;
        let mapping = Handle::checked(unsafe {
            CreateFileMappingW(-1_isize as RawHandle, ptr::null(), 0x04 /* PAGE_READWRITE */,
                0, size as u32, map_name.as_ptr())
        })?;
        let address = unsafe {
            MapViewOfFile(mapping.0, 0x0006 /* FILE_MAP_READ | FILE_MAP_WRITE */,
                0, 0, size)
        };
        if address.is_null() { return Err(io::Error::last_os_error()); }
        Ok(Self { view: View(address), _mapping: mapping, mutex })
    }

    // Snapshot buffers are copied while locked, then decoded off shared memory.
    pub(crate) fn blob(&self, bytes: &mut [u8], write: bool) -> io::Result<()> {
        let wait = unsafe { WaitForSingleObject(self.mutex.0, 5) };
        if wait == WAIT_TIMEOUT { return Err(io::Error::new(io::ErrorKind::TimedOut,"world snapshot lock timed out")); }
        if wait != WAIT_OBJECT_0 && wait != WAIT_ABANDONED { return Err(io::Error::last_os_error()); }
        let _lock = Lock(&self.mutex);
        if wait == WAIT_ABANDONED {
            unsafe { ptr::write_bytes(self.view.0.cast::<u8>(),0,bytes.len()); }
            return Err(io::Error::other("world snapshot writer died; restart both games"));
        }
        unsafe {
            if write { ptr::copy_nonoverlapping(bytes.as_ptr(), self.view.0.cast(),bytes.len()); }
            else { ptr::copy_nonoverlapping(self.view.0.cast(),bytes.as_mut_ptr(),bytes.len()); }
        }
        Ok(())
    }

    pub(crate) fn update<T>(&self, action: impl FnOnce(&mut Record) -> io::Result<T>) -> io::Result<T> {
        let wait = unsafe { WaitForSingleObject(self.mutex.0, 5) };
        if wait == WAIT_TIMEOUT {
            return Err(io::Error::new(io::ErrorKind::TimedOut, "shared-memory lock timed out"));
        }
        if wait != WAIT_OBJECT_0 && wait != WAIT_ABANDONED {
            return Err(io::Error::last_os_error());
        }
        let _lock = Lock(&self.mutex);
        let address = self.view.0.cast::<[u8; RECORD_SIZE]>();
        if wait == WAIT_ABANDONED {
            // A peer died while writing: poison the header so every reader fails
            // closed, including the next one after this thread releases the lock.
            let mut poisoned = [0; RECORD_SIZE];
            poisoned[..8].copy_from_slice(b"BROKEN!!");
            unsafe { ptr::write_volatile(address, poisoned); }
            return Err(io::Error::other("peer died while holding the lock; stop both peers and restart"));
        }
        // No Rust reference points into cross-process memory. Decode a local copy,
        // then commit it while still owning the mutex; errors do not commit edits.
        let bytes = unsafe { ptr::read_volatile(address) };
        let mut record = if bytes == [0; RECORD_SIZE] {
            Record::default() // Windows zero-initializes a newly created mapping.
        } else {
            Record::decode(&bytes)?
        };
        let result = action(&mut record)?;
        unsafe { ptr::write_volatile(address, record.encode()); }
        Ok(result)
    }
}
