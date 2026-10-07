//! Stable C entry points used by Fabric's Java foreign-function API.
use crate::{Endpoint, Role, State, PROTOCOL_VERSION, position::Sample};
use std::{cell::RefCell, fs::{self, File}, io::{self, Write}, path::PathBuf,
    sync::{Mutex, OnceLock}};

static MC_LOG: OnceLock<Mutex<File>> = OnceLock::new();
static SKY_LOG: OnceLock<Mutex<File>> = OnceLock::new();

pub(crate) fn write_log(role: Role, message: impl AsRef<str>) -> io::Result<()> {
    let slot = match role { Role::Minecraft => &MC_LOG, Role::Skyrim => &SKY_LOG };
    if slot.get().is_none() {
        let fixture = cfg!(test) || std::env::var("PASSTHROUGH_TEST_MODE").as_deref() == Ok("1");
        let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(if fixture { "../logs/step2-fixtures" } else { "../logs/step2" });
        fs::create_dir_all(&folder)?;
        let name = match role { Role::Minecraft => "minecraft", Role::Skyrim => "skyrim" };
        let file = File::create(folder.join(format!("{name}-{}-{}.log", std::process::id(), crate::windows::now_ms())))?;
        let _ = slot.set(Mutex::new(file));
    }
    let mut file = slot.get().unwrap().lock().map_err(|_| io::Error::other("log lock poisoned"))?;
    let now=crate::windows::now_ms();
    for line in message.as_ref().lines() {
        writeln!(file, "time_ms={now} pid={} {line}", std::process::id())?;
    }
    file.flush()
}

pub(crate) struct GameLink { pub endpoint: Endpoint, role: Role, last_state: Option<State> }
impl GameLink {
    pub fn open(role: Role) -> io::Result<Self> {
        let session = std::env::var("PASSTHROUGH_SESSION").unwrap_or_else(|_| "game".into());
        Self::open_named(role, &session)
    }
    pub fn open_named(role: Role, session: &str) -> io::Result<Self> {
        let endpoint = Endpoint::open(session, role, PROTOCOL_VERSION)?;
        write_log(role, format!("event=NATIVE_READY protocol={PROTOCOL_VERSION}"))?;
        Ok(Self { endpoint, role, last_state: None })
    }
    pub fn poll(&mut self) -> io::Result<()> {
        let state = self.endpoint.poll()?;
        if self.last_state != Some(state.state) {
            write_log(self.role, format!("event=LINK state={:?} peer_pid={}", state.state, state.peer_pid))?;
            self.last_state = Some(state.state);
        }
        Ok(())
    }
}

thread_local! { static MINECRAFT: RefCell<Option<GameLink>> = const { RefCell::new(None) }; }
thread_local! { static LAST_WORLD: RefCell<Option<(u64,u64,u64)>> = const { RefCell::new(None) }; }

/// Called only on the render thread; Java publishes an immutable copy to physics threads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pt_mc_world(destination:*mut u8,capacity:u32)->i32 {
    mc_guard(|| MINECRAFT.with(|state| {
        if destination.is_null() || capacity as usize!=crate::world::SIZE { return Err(io::Error::other("invalid world destination")); }
        let mut state=state.borrow_mut();
        let link=state.as_mut().ok_or_else(||io::Error::other("world receiver not initialized"))?;
        let Some(snapshot)=link.endpoint.receive_world()? else { return Ok(0); };
        let key=(snapshot.sender,snapshot.epoch,snapshot.sequence);
        LAST_WORLD.with(|last| -> io::Result<()> {
            if *last.borrow()!=Some(key) { snapshot.log(Role::Minecraft,"RECV")?; *last.borrow_mut()=Some(key); }
            Ok(())
        })?;
        let mut bytes=snapshot.encode()?;
        let remaining=crate::world::MAX_AGE_MS.saturating_sub(crate::windows::now_ms().saturating_sub(snapshot.sent_ms));
        bytes[72..80].copy_from_slice(&remaining.to_le_bytes());
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(),destination,bytes.len()); }
        Ok(1)
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn pt_mc_init() -> i32 {
    mc_guard(|| MINECRAFT.with(|state| {
        let mut state = state.borrow_mut();
        if state.is_none() { *state = Some(GameLink::open(Role::Minecraft)?); }
        Ok(1)
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn pt_mc_frame4(world: u64, frame: u64, partial: f64,
    x: f64, y: f64, z: f64, yaw:f64, pitch:f64, active: u32) -> i32
{
    mc_guard(|| MINECRAFT.with(|state| {
        let mut state = state.borrow_mut();
        let link = state.as_mut().ok_or_else(|| io::Error::other("Minecraft bridge not initialized on this thread"))?;
        link.poll()?;
        let sample = link.endpoint.send_position(Sample { world, frame, partial, mc: [x,y,z], yaw, pitch, active, ..Sample::default() })?;
        write_log(Role::Minecraft, format!("event=SEND active={active} {}", sample.log_fields()))?;
        Ok(1)
    }))
}

fn mc_guard(action: impl FnOnce() -> io::Result<i32>) -> i32 {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(action)) {
        Ok(Ok(value)) => value,
        Ok(Err(error)) if error.kind() == io::ErrorKind::TimedOut => 0, // Skip a contended frame.
        other => {
            let message = match other { Ok(Err(e)) => e.to_string(), _ => "native bridge panic".into() };
            let _ = write_log(Role::Minecraft, format!("event=ERROR message={message}"));
            MINECRAFT.with(|state| { *state.borrow_mut() = None; });
            -1
        }
    }
}
