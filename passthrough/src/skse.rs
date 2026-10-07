//! Narrow SKSE/Skyrim 1.7.104 ABI adapter; definitions are documented in docs/ABI.md.
//! All game reads/writes below occur on Skyrim's player-update thread.
use crate::{Role, native::{GameLink, write_log}, position::{Anchor, Sample}};
use std::{cell::RefCell, ffi::c_void, fs, io, mem, ptr,
    sync::{OnceLock, atomic::{AtomicBool, AtomicUsize, Ordering}}};

const RUNTIME: u32 = (1 << 24) | (7 << 16) | (104 << 4);
type Object = *mut u8;
type Update = unsafe extern "C" fn(Object, f32);

#[repr(C)]
pub struct SkseInterface {
    skse_version: u32, runtime_version: u32, editor_version: u32, is_editor: u32,
    query: unsafe extern "C" fn(u32) -> *const Messaging,
    handle: unsafe extern "C" fn() -> u32,
    release_index: usize, plugin_info: usize,
}
#[repr(C)]
pub struct Messaging {
    version: u32,
    register: unsafe extern "C" fn(u32, *const u8, unsafe extern "C" fn(*const Message)) -> bool,
    dispatch: usize, dispatcher: usize,
}
#[repr(C)]
pub struct Message { sender: *const u8, kind: u32, len: u32, data: *const u8 }
#[repr(C)]
pub struct VersionData {
    data_version: u32, plugin_version: u32, name: [u8;256], author: [u8;256],
    email: [u8;252], independence_ex: u32, independence: u32,
    versions: [u32;16], minimum_skse: u32,
}
const fn text<const N: usize>(value: &[u8]) -> [u8;N] {
    let mut out = [0;N]; let mut i = 0;
    while i < value.len() { out[i] = value[i]; i += 1; } out
}
#[unsafe(no_mangle)]
pub static SKSEPlugin_Version: VersionData = VersionData {
    data_version: 1, plugin_version: 0x0004_0000,
    name: text(b"PassthroughPractice"), author: text(b"Passthrough practice project"),
    email: [0;252], independence_ex: 2, independence: 4,
    versions: [RUNTIME,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], minimum_skse: 0,
};

#[link(name="kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
    fn VirtualProtect(address: *mut c_void, size: usize, new_protect: u32, old: *mut u32) -> i32;
}

struct Addresses { vtable: usize, ui: usize, world: Option<crate::world_export::Api>, look:Option<crate::look::Api> }
static ADDRESSES: OnceLock<Addresses> = OnceLock::new();
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static RESET: AtomicBool = AtomicBool::new(false);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SKSEPlugin_Load(api: *const SkseInterface) -> bool {
    let result = std::panic::catch_unwind(|| -> io::Result<()> {
        if api.is_null() { return Err(io::Error::other("null SKSE interface")); }
        let api = unsafe { &*api };
        if api.is_editor != 0 || api.runtime_version != RUNTIME {
            return Err(io::Error::other("only Skyrim runtime 1.7.104 is supported"));
        }
        write_log(Role::Skyrim,"event=LOAD_BEGIN runtime=1.7.104")?;
        let exe = std::env::current_exe()?;
        let root = exe.parent().ok_or_else(|| io::Error::other("missing Skyrim directory"))?;
        let db_path = root.join("Data/SKSE/Plugins/versionlib-1-7-104-0.bin");
        let db = fs::read(&db_path).map_err(|error| io::Error::new(error.kind(), format!(
            "cannot read Address Library '{}': {error}; install the Address Library for Skyrim runtime 1.7.104.0",
            db_path.display()
        )))?;
        // Read the running executable's PE header to bound every address-library RVA.
        let base = unsafe { GetModuleHandleW(ptr::null()) } as usize;
        if base == 0 { return Err(io::Error::last_os_error()); }
        let pe = unsafe { ptr::read_unaligned((base + 0x3c) as *const u32) } as usize;
        let image_size = unsafe { ptr::read_unaligned((base + pe + 24 + 56) as *const u32) } as usize;
        let offsets = address_offsets(&db, image_size)?;
        let resolve=|id:usize| -> io::Result<usize> {
            let at=96+id*4;
            let value=db.get(at..at+4).ok_or_else(||io::Error::other("missing world export Address Library ID"))?;
            let offset=u32::from_le_bytes(value.try_into().unwrap()) as usize;
            if offset==0 || offset+8>image_size { return Err(io::Error::other("world export address outside executable")); }
            Ok(base+offset)
        };
        let world=crate::world_export::Api{cell_world:resolve(18995)?,cast:resolve(61399)?,
            scale:resolve(188105)?,processes:resolve(400315)?,lookup:resolve(12332)?,
            lock:resolve(68233)?,unlock:resolve(68239)?};
        let look=crate::look::Api{singleton:resolve(400802)?,first:resolve(50790)?,third:resolve(50796)?};
        let address = Addresses { vtable: base+offsets[0], ui: base+offsets[1], world:Some(world),look:Some(look) };
        ADDRESSES.set(address).map_err(|_| io::Error::other("plugin already loaded"))?;
        let messaging = unsafe { (api.query)(5) };
        if messaging.is_null() || unsafe { (*messaging).version } < 2 {
            return Err(io::Error::other("SKSE messaging interface 2 required"));
        }
        if !unsafe { ((*messaging).register)((api.handle)(), c"SKSE".as_ptr().cast(), message) } {
            return Err(io::Error::other("SKSE listener registration failed"));
        }
        // Once a listener is registered, never return false and unload its code.
        let _ = write_log(Role::Skyrim, "plugin loaded runtime=1.7.104 event=PLUGIN_LOADED");
        Ok(())
    });
    match result {
        Ok(Ok(())) => true,
        failure => {
            let why = match failure { Ok(Err(e)) => e.to_string(), _ => "panic during plugin load".into() };
            let _ = write_log(Role::Skyrim, format!("event=LOAD_FAILED message={why}"));
            false
        }
    }
}

// Format 5 is a documented 96-byte header followed by a dense u32 RVA table.
// IDs refer to published CommonLib AE APIs, not offsets guessed from game data.
fn address_offsets(bytes: &[u8], image_size: usize) -> io::Result<[usize;2]> {
    let bad = || io::Error::other("invalid or incompatible Address Library (need format 5 for 1.7.104.0)");
    let word = |at: usize| -> io::Result<u32> {
        Ok(u32::from_le_bytes(bytes.get(at..at+4).ok_or_else(bad)?.try_into().unwrap()))
    };
    if word(0)? != 5 || [word(4)?,word(8)?,word(12)?,word(16)?] != [1,7,104,0]
        || word(84)? != 8 || word(88)? != 0 { return Err(bad()); }
    let count = word(92)? as usize;
    if count > 10_000_000 || bytes.len() != 96 + count*4 { return Err(bad()); }
    let mut out = [0;2];
    for (i,id) in [208040,400327].into_iter().enumerate() {
        if id >= count { return Err(bad()); }
        out[i] = word(96+id*4)? as usize;
        let need = if i == 0 { (0xAD+1)*8 } else { 8 };
        if out[i] == 0 || out[i].checked_add(need).is_none_or(|v| v > image_size) { return Err(bad()); }
    }
    Ok(out)
}

unsafe extern "C" fn message(raw: *const Message) {
    let _ = std::panic::catch_unwind(|| {
        if raw.is_null() { return; }
        let msg = unsafe { &*raw };
        match msg.kind {
            8 => { // Data loaded: install the player update hook once.
                if ORIGINAL.load(Ordering::Acquire) != 0 { return; }
                if let Some(addresses) = ADDRESSES.get() {
                    let slot = (addresses.vtable + 0xAD*8) as *mut usize;
                    let mut protection = 0;
                    if unsafe { VirtualProtect(slot.cast(), 8, 0x04, &mut protection) } == 0 {
                        let _ = write_log(Role::Skyrim, "event=ERROR cannot protect player vtable"); return;
                    }
                    ORIGINAL.store(unsafe { ptr::read(slot) }, Ordering::Release);
                    unsafe { ptr::write(slot, update as *const () as usize); }
                    let mut unused = 0;
                    let restored = unsafe { VirtualProtect(slot.cast(), 8, protection, &mut unused) };
                    let _ = write_log(Role::Skyrim, format!("event=HOOK_INSTALLED protection_restored={}", restored != 0));
                }
            }
            2 => { READY.store(false,Ordering::Release); RESET.store(true,Ordering::Release); }
            3 => { // SKSE encodes the bool as (void*)result, not a pointer to a bool.
                let success = load_succeeded(msg.data);
                READY.store(success,Ordering::Release); RESET.store(true,Ordering::Release);
            }
            7 => { READY.store(true,Ordering::Release); RESET.store(true,Ordering::Release); }
            _ => {}
        }
    });
}

fn load_succeeded(data: *const u8) -> bool { !data.is_null() }

#[derive(Default)]
struct Puppet {
    link: Option<GameLink>, anchor: Option<Anchor>, last: Option<(u64,u64,u64)>,
    gravity: Option<(usize,f32)>, failed: bool, last_update_ms: u64,
    exporter: crate::world_export::Exporter, cell: usize,
    look:crate::look::Look,
}
thread_local! { static PUPPET: RefCell<Puppet> = RefCell::new(Puppet::default()); }

unsafe fn read_at<T: Copy>(object: Object, offset: usize) -> T {
    unsafe { ptr::read_unaligned(object.add(offset).cast::<T>()) }
}
unsafe fn controller(player: Object) -> Object {
    let process: Object = unsafe { read_at(player,0xF8) };
    if process.is_null() { return ptr::null_mut(); }
    let middle: Object = unsafe { read_at(process,0x08) };
    if middle.is_null() { return ptr::null_mut(); }
    unsafe { read_at(middle,0x250) }
}
#[repr(C,align(16))]
struct Vector4([f32;4]);

impl Puppet {
    unsafe fn release(&mut self, player: Object) {
        if let Some(api)=ADDRESSES.get().and_then(|a|a.look.as_ref()) { unsafe { self.look.release(api); } }
        if !player.is_null() {
            let ctrl = unsafe { controller(player) };
            if let Some((saved,gravity)) = self.gravity {
                if !ctrl.is_null() && ctrl as usize == saved {
                    unsafe { ptr::write_unaligned(ctrl.add(0x248).cast::<f32>(),gravity); }
                }
            }
        }
        if self.anchor.is_some() {
            let _ = write_log(Role::Skyrim,"event=RELEASED");
            self.exporter.reset();
            if let Some(link)=self.link.as_mut() {
                if let Ok(snapshot)=link.endpoint.send_world(self.exporter.empty()) {
                    let _=snapshot.log(Role::Skyrim,"SEND");
                }
            }
        }
        self.gravity = None; self.anchor = None; self.last = None;
    }
    unsafe fn frame(&mut self, player: Object) -> io::Result<()> {
        if player.is_null() { return Ok(()); }
        let cell=unsafe { read_at::<Object>(player,0x60) } as usize;
        if self.cell!=cell { unsafe { self.release(player); } self.cell=cell; }
        let now = crate::windows::now_ms();
        if RESET.swap(false,Ordering::AcqRel) || now.saturating_sub(self.last_update_ms) > crate::position::MAX_SAMPLE_AGE_MS {
            unsafe { self.release(player); }
        }
        self.last_update_ms = now;
        let addresses = ADDRESSES.get().ok_or_else(|| io::Error::other("missing game addresses"))?;
        let ui = unsafe { *(addresses.ui as *const Object) };
        let paused = ui.is_null() || unsafe { read_at::<u32>(ui,0x160) } != 0;
        if self.failed || !READY.load(Ordering::Acquire) || paused {
            unsafe { self.release(player); } return Ok(());
        }
        if self.link.is_none() { self.link = Some(GameLink::open(Role::Skyrim)?); }
        let link = self.link.as_mut().unwrap(); link.poll()?;
        let Some(sample) = link.endpoint.receive_position()? else {
            unsafe { self.release(player); } return Ok(());
        };
        let current: [f32;3] = unsafe { read_at(player,0x54) };
        if self.anchor.is_none_or(|a| !a.matches(sample)) {
            unsafe { self.release(player); }
            self.anchor = Some(Anchor::new(sample,current));
            write_log(Role::Skyrim,format!("event=ANCHOR {} sky_x={} sky_y={} sky_z={}",
                sample.log_fields(),current[0],current[1],current[2]))?;
        }
        let target = self.anchor.unwrap().target(sample).ok_or_else(|| io::Error::other("invalid puppet target"))?;
        let ctrl = unsafe { controller(player) };
        if ctrl.is_null() { unsafe { self.release(player); } return Ok(()); }
        if self.gravity.is_none_or(|(old,_)| old != ctrl as usize) {
            self.gravity = Some((ctrl as usize,unsafe { read_at(ctrl,0x248) }));
        }
        unsafe { ptr::write_unaligned(ctrl.add(0x248).cast::<f32>(),0.0); }
        let vtable: *const usize = unsafe { read_at(player,0) };
        let set_position: unsafe extern "C" fn(Object,*const [f32;3],bool) = unsafe { mem::transmute(*vtable.add(0xA9)) };
        unsafe { set_position(player,&target,true); }
        let ctrl_vtable: *const usize = unsafe { read_at(ctrl,0) };
        let velocity: unsafe extern "C" fn(Object,*const Vector4) = unsafe { mem::transmute(*ctrl_vtable.add(7)) };
        unsafe {
            velocity(ctrl,&Vector4([0.0;4]));
            ptr::write_unaligned(ctrl.add(0x240).cast::<f32>(),target[2]);
            ptr::write_unaligned(ctrl.add(0x244).cast::<f32>(),0.0);
        }
        let key = (sample.sender,sample.world,sample.frame);
        if let Some(api)=addresses.look.as_ref() {
            unsafe { self.look.apply(api,player,sample,self.last!=Some(key))?; }
        }
        if self.last != Some(key) {
            let actual: [f32;3] = unsafe { read_at(player,0x54) };
            log_received(sample,target,actual)?;
            self.last = Some(key);
        }
        if let Some(api)=addresses.world.as_ref() {
            if let Some(snapshot)=unsafe { self.exporter.frame(api,player,self.anchor.unwrap(),sample)? } {
                let snapshot=self.link.as_mut().unwrap().endpoint.send_world(snapshot)?;
                snapshot.log(Role::Skyrim,"SEND")?;
            }
        }
        Ok(())
    }
}
fn log_received(sample: Sample, target: [f32;3], actual: [f32;3]) -> io::Result<()> {
    write_log(Role::Skyrim,format!("event=RECV {} target_x={:.9} target_y={:.9} target_z={:.9} actual_x={:.9} actual_y={:.9} actual_z={:.9}",
        sample.log_fields(),target[0],target[1],target[2],actual[0],actual[1],actual[2]))
}
unsafe extern "C" fn update(player: Object, delta: f32) {
    let original = ORIGINAL.load(Ordering::Acquire);
    if original == 0 { return; }
    let original: Update = unsafe { mem::transmute(original) };
    unsafe { original(player,delta); }
    let result = std::panic::catch_unwind(|| PUPPET.with(|state| unsafe { state.borrow_mut().frame(player) }));
    if matches!(&result,Ok(Err(error)) if error.kind()==io::ErrorKind::TimedOut) {
        PUPPET.with(|state| unsafe { state.borrow_mut().release(player); });
        return;
    }
    if !matches!(result,Ok(Ok(()))) {
        let message = match result { Ok(Err(e)) => e.to_string(), _ => "panic in player update".into() };
        let _ = write_log(Role::Skyrim,format!("event=ERROR message={message}"));
        PUPPET.with(|state| {
            let mut state = state.borrow_mut(); unsafe { state.release(player); }
            state.link = None; state.failed = true;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn skse_metadata_layout_and_runtime_are_exact() {
        assert_eq!(mem::size_of::<VersionData>(),0x350);
        assert_eq!(mem::offset_of!(VersionData,versions),0x30c);
        assert_eq!(RUNTIME,0x0107_0680);
        assert_eq!(mem::size_of::<SkseInterface>(),48);
        assert!(!load_succeeded(ptr::null()));
        assert!(load_succeeded(1_usize as *const u8));
    }
    #[test]
    fn address_library_checks_version_length_and_offsets() {
        let mut bytes = vec![0_u8;96+400864*4];
        for (offset,value) in [(0,5_u32),(4,1),(8,7),(12,104),(84,8),(92,400864)] {
            bytes[offset..offset+4].copy_from_slice(&value.to_le_bytes());
        }
        for id in [208040,400863,68545,68548,400327] {
            bytes[96+id*4..100+id*4].copy_from_slice(&0x1000_u32.to_le_bytes());
        }
        assert!(address_offsets(&bytes,0x10000).is_ok());
        assert!(address_offsets(&bytes,0x1001).is_err());
        assert!(address_offsets(&bytes[..bytes.len()-1],0x10000).is_err());
        bytes[12]=103;
        assert!(address_offsets(&bytes,0x10000).is_err());
    }

    // Simulated objects, not Skyrim: exercise the same pointer offsets, virtual
    // calls, coordinate application and gravity restoration used by the adapter.
    #[test]
    fn puppet_updates_actor_and_controller_then_restores_gravity() {
        use crate::{Endpoint,PROTOCOL_VERSION};
        unsafe extern "C" fn set_position(player:Object,position:*const [f32;3],update_controller:bool) {
            if update_controller { unsafe { ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),*position); } }
        }
        unsafe extern "C" fn set_velocity(ctrl:Object,velocity:*const Vector4) {
            unsafe { ptr::write_unaligned(ctrl.add(0x100).cast::<[f32;4]>(),(*velocity).0); }
        }
        let mut actor=[0_usize;128]; let player=actor.as_mut_ptr().cast::<u8>();
        let mut process=[0_usize;4]; let mut middle=[0_usize;80];
        let mut controller_data=[0_usize;100]; let ctrl=controller_data.as_mut_ptr().cast::<u8>();
        let mut actor_vtable=[0_usize;0xAE]; actor_vtable[0xA9]=set_position as *const () as usize;
        let mut ctrl_vtable=[0_usize;8]; ctrl_vtable[7]=set_velocity as *const () as usize;
        let mut ui_data=[0_usize;64]; let ui=ui_data.as_mut_ptr().cast::<u8>();
        let ui_slot=Box::new(ui as usize);
        ADDRESSES.set(Addresses{vtable:0,ui:(&*ui_slot) as *const usize as usize,world:None,look:None}).ok().unwrap();
        unsafe {
            ptr::write(player.cast::<usize>(),actor_vtable.as_ptr() as usize);
            ptr::write_unaligned(player.add(0xF8).cast::<usize>(),process.as_mut_ptr() as usize);
            ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),[100.0,-200.0,300.0]);
            ptr::write(process.as_mut_ptr().add(1),middle.as_mut_ptr() as usize);
            ptr::write(middle.as_mut_ptr().add(0x250/8),ctrl as usize);
            ptr::write(ctrl.cast::<usize>(),ctrl_vtable.as_ptr() as usize);
            ptr::write_unaligned(ctrl.add(0x248).cast::<f32>(),9.8);
            ptr::write_unaligned(ctrl.add(0x100).cast::<[f32;4]>(),[1.0;4]);
        }
        let session=format!("puppet_test_{}",std::process::id());
        let mut sender=Endpoint::open(&session,Role::Minecraft,PROTOCOL_VERSION).unwrap();
        let mut receiver=GameLink::open_named(Role::Skyrim,&session).unwrap();
        sender.poll().unwrap(); receiver.poll().unwrap(); sender.poll().unwrap();
        let sample=Sample{world:1,frame:1,active:1,partial:0.5,mc:[10.0,64.0,20.0],..Sample::default()};
        sender.send_position(sample).unwrap();
        READY.store(true,Ordering::Release);
        let mut puppet=Puppet{link:Some(receiver),..Puppet::default()};
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<[f32;3]>(player,0x54) },[100.0,-200.0,300.0]);
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x248) },0.0);
        sender.send_position(Sample{frame:2,mc:[11.0,64.5,18.0],..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<[f32;3]>(player,0x54) },[170.0,-60.0,335.0]);
        assert_eq!(unsafe { read_at::<[f32;4]>(ctrl,0x100) },[0.0;4]);
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x240) },335.0);
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x244) },0.0);
        sender.send_position(Sample{frame:3,active:0,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x248) },9.8);
        assert!(puppet.anchor.is_none());
        // Another world starts at the current Skyrim position, without teleporting.
        sender.send_position(Sample{world:2,frame:4,mc:[9000.0,72.0,-4000.0],..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<[f32;3]>(player,0x54) },[170.0,-60.0,335.0]);
        std::thread::sleep(std::time::Duration::from_millis(300));
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x248) },9.8);
        assert!(puppet.anchor.is_none());
        READY.store(false,Ordering::Release);
    }
}
