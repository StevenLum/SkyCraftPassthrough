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
    data_version: 1, plugin_version: 0x000A_003B,
    name: text(b"PassthroughPractice"), author: text(b"Passthrough practice project"),
    email: [0;252], independence_ex: 2, independence: 4,
    versions: [RUNTIME,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], minimum_skse: 0,
};

#[link(name="kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
    fn VirtualProtect(address: *mut c_void, size: usize, new_protect: u32, old: *mut u32) -> i32;
}

struct Addresses { vtable: usize, ui: usize, render_window: Option<usize>, hud_vtable:Option<usize>, world: Option<crate::world_export::Api>, look:Option<crate::look::Api>, interaction:Option<crate::interaction::Api>, stand_up:Option<usize> }
/// Chair activation is only exposed when exiting it is possible.
pub(crate) fn sitting_available()->bool {
    crate::switches::on(Role::Skyrim,crate::switches::SITTING) && ADDRESSES.get().is_some_and(|a|a.stand_up.is_some())
}
static ADDRESSES: OnceLock<Addresses> = OnceLock::new();
static ORIGINAL: AtomicUsize = AtomicUsize::new(0);
static ORIGINAL_DAMAGE:AtomicUsize=AtomicUsize::new(0);
static DAMAGE_SEQUENCE:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
static READY: AtomicBool = AtomicBool::new(false);
static RESET: AtomicBool = AtomicBool::new(false);
pub(crate) fn menu_ready()->bool {READY.load(Ordering::Acquire) && !RESET.load(Ordering::Acquire)}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SKSEPlugin_Load(api: *const SkseInterface) -> bool {
    let result = std::panic::catch_unwind(|| -> io::Result<()> {
        if api.is_null() { return Err(io::Error::other("null SKSE interface")); }
        let api = unsafe { &*api };
        if api.is_editor != 0 || api.runtime_version != RUNTIME {
            return Err(io::Error::other("only Skyrim runtime 1.7.104 is supported"));
        }
        write_log(Role::Skyrim,"event=LOAD_BEGIN runtime=1.7.104")?;
        // Read switches.txt now, so the log says from the start which steps are off.
        let _=crate::switches::mask(Role::Skyrim);
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
        if let Ok(modifier)=resolve(38469) {crate::damage::configure(modifier);}
        // ValueModifierEffect::ModifyActorValue slot0x20. Ordinary health potions
        // call Actor::ModActorValue directly and bypass ActorValueOwner slot6.
        match resolve(206143) {
            Ok(table)=>crate::damage::configure_effect(table),
            Err(_)=>{let _=write_log(Role::Skyrim,"event=SCENE_HEAL_EFFECT_HOOK state=unavailable reason=missing_address");}
        }
        if let (Ok(crosshair),Ok(lookup),Ok(damage))=(resolve(401585),resolve(12332),resolve(37335)) {
            // Actor::IsHostileToActor; without it an enemy is reported as an ordinary character.
            let hostile=resolve(37537).ok();
            if hostile.is_none() {let _=write_log(Role::Skyrim,"event=SCENE_CROSSHAIR state=hostility_unavailable reason=missing_address");}
            crate::attack::configure(crosshair,lookup,damage,hostile);
        }
        match resolve(37335) {
            Ok(damage)=>crate::death::configure(damage),
            Err(_)=>{let _=write_log(Role::Skyrim,"event=SCENE_DEATH state=disabled reason=missing_address");}
        }
        let look=crate::look::Api{singleton:resolve(400802)?,first:resolve(50790)?,third:resolve(50796)?};
        match (resolve(411393),resolve(36609)) {
            (Ok(renderer),Ok(camera))=>crate::scene_draw::configure(renderer,camera),
            _=> { let _=write_log(Role::Skyrim,"event=SCENE_DISABLED message=missing_renderer_or_camera_address"); }
        }
        // CommonLib Actor::StopInteractingQuick(bool), RELOCATION_ID(37752, 38697).
        // Optional: missing support must not stop normal bridge movement.
        let stand_up=resolve(38697).ok();
        if stand_up.is_none() {write_log(Role::Skyrim,"event=SCENE_STAND_UP state=disabled reason=missing_address")?;}
        let address = Addresses { vtable: base+offsets[0], ui: base+offsets[1], render_window:resolve(411349).ok(), hud_vtable:resolve(215362).ok(), world:Some(world),look:Some(look),stand_up,
            interaction:match (resolve(40548),resolve(401585),resolve(12332)) {
                // TESObjectREFR::GetLock, CommonLib RELOCATION_ID(19818, 20223): read for the log only.
                (Ok(activate),Ok(crosshair),Ok(lookup))=>Some(crate::interaction::Api{activate,crosshair,lookup,lock:resolve(20223).ok(),midair:resolve(37243).ok()}),
                _=>{let _=write_log(Role::Skyrim,"event=SCENE_INTERACT_DISABLED reason=missing_address");None}
            } };
        ADDRESSES.set(address).map_err(|_| io::Error::other("plugin already loaded"))?;
        let messaging = unsafe { (api.query)(5) };
        if messaging.is_null() || unsafe { (*messaging).version } < 2 {
            return Err(io::Error::other("SKSE messaging interface 2 required"));
        }
        if !unsafe { ((*messaging).register)((api.handle)(), c"SKSE".as_ptr().cast(), message) } {
            return Err(io::Error::other("SKSE listener registration failed"));
        }
        // Once a listener is registered, never return false and unload its code.
        let menus=(|| -> io::Result<()> {
            let menu=crate::menu_input::Api{ui:base+offsets[1],player:resolve(403521)?,controls:resolve(401263)?,
                input:resolve(402776)?,button:resolve(208708)?,string_ctor:resolve(69161)?,string_release:resolve(69192)?,
                cursor:resolve(403551).ok().zip(resolve(236703).ok()),
                // CommonLib PlayerControls.cpp: singleton RELOCATION_ID(514706, 400864).
                player_controls:resolve(400864).ok(),
                // Pinned CommonLib InventoryMenu/FavoritesMenu primary vtable IDs.
                item_click:resolve(215494).ok().zip(resolve(215307).ok()).zip(resolve(215061).ok()).map(|((i,f),c)|[i,f,c])};
            if menu.cursor.is_none() {let _=write_log(Role::Skyrim,"event=SCENE_CURSOR state=disabled reason=missing_address");}
            if menu.item_click.is_none() {let _=write_log(Role::Skyrim,"event=SCENE_MENU state=item_click_unavailable reason=missing_address");}
            if menu.player_controls.is_none() {let _=write_log(Role::Skyrim,"event=SCENE_MENU state=shout_unavailable reason=missing_address");}
            unsafe {crate::menu_input::start((api.query)(4).cast(),menu)}
        })();
        if let Err(error)=menus {let _=write_log(Role::Skyrim,format!("event=SCENE_MENU_DISABLED reason={error}"));}
        if !unsafe {crate::save_identity::register((api.query)(3).cast(),(api.handle)())} {
            let _=write_log(Role::Skyrim,"event=SCENE_SAVE_ID state=interface_unavailable id=none");
        }
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
        let need = if i == 0 { (0x104+1)*8 } else { 8 };
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
                    if let Some(window)=addresses.render_window {
                        if let Err(error)=unsafe { crate::overlay_draw::install(window) } {
                            let _=write_log(Role::Skyrim,format!("event=OVERLAY_DISABLED message={error}"));
                        }
                    } else { let _=write_log(Role::Skyrim,"event=OVERLAY_DISABLED message=missing_render_window_address"); }
                    if let Some(table)=addresses.hud_vtable {
                        if let Err(error)=unsafe { crate::overlay_draw::install_hud(table) } {
                            let _=write_log(Role::Skyrim,format!("event=SCENE_STAGE_DISABLED message={error}"));
                        }
                    } else { let _=write_log(Role::Skyrim,"event=SCENE_STAGE_DISABLED message=missing_hud_vtable"); }
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
                    let damage_slot=(addresses.vtable+0x104*8) as *mut usize;
                    let mut old=0;
                    if unsafe {VirtualProtect(damage_slot.cast(),8,0x04,&mut old)}!=0 {
                        ORIGINAL_DAMAGE.store(unsafe {*damage_slot},Ordering::Release);
                        unsafe {ptr::write(damage_slot,damage_probe as *const () as usize);}
                        let mut unused=0;
                        let restored=unsafe {VirtualProtect(damage_slot.cast(),8,old,&mut unused)};
                        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_HOOK slot=260 mode=observe protection_restored={}",restored!=0));
                    } else {let _=write_log(Role::Skyrim,"event=SCENE_DAMAGE_DISABLED reason=protect_failed");}
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
    suspended:Option<(Anchor,[f32;3])>,
    admission:Option<(bool,[u8;16],[u8;16])>,
    origin_block:Option<u32>,
    interaction:crate::interaction::Interaction,
    attack:crate::attack::Attack,
    menu_blocked:bool,
    recovery:crate::recovery::Recovery,
    recovery_exporter:crate::world_export::Exporter,
    recovery_scan:Option<(u64,u64,[u8;16],[u8;16],u64,u32,u32)>,
    recovery_blocked:bool,
    death:crate::death::Death,
    /// The cell changed and has not been judged yet (see `stays_steered`).
    crossed:bool,
    /// Outdoor world (form ID, 0 for an interior) of the place being steered in.
    steered_world:u32,
    /// How often the character was let go; only read by the tests.
    released:u64,
    /// Skyrim's sit state of the character when it was last moved (seat.rs).
    seat:Option<u32>,
    seat_watch:crate::seat::Watch,
    sitting:crate::seat::Intent,
    seat_published_ms:u64,chair_scope:Option<(usize,u64)>,chair_held:bool,chair_hold_failed:bool,chair_clearance:Option<(u64,Vec<[f64;6]>)>,
}
/// How far Skyrim's character may be from where Minecraft says it is and still count as
/// "in the same spot" when its cell changes. The same tolerance a session start uses.
const CROSSING_TOLERANCE:f64=140.0;
/// Whether a change of cell leaves the character steered. Skyrim cuts an outdoor world into
/// cells; walking into the next one is still the same place. An interior (world 0) is a
/// place of its own, and a door, a load or fast travel moves the character away from where
/// Minecraft says it is, which fails the distance test.
fn stays_steered(old_world:u32,new_world:u32,distance:Option<f64>)->bool {
    old_world!=0 && old_world==new_world && distance.is_some_and(|d|d.is_finite() && d<=CROSSING_TOLERANCE)
}
thread_local! { static PUPPET: RefCell<Puppet> = RefCell::new(Puppet::default()); }
thread_local! { static COST: RefCell<crate::cost::Cost> = const { RefCell::new(crate::cost::Cost::new()) }; }

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
    unsafe fn suspend(&mut self, player:Object) {
        if self.chair_held || self.chair_hold_failed {
            self.recovery.clear();self.recovery_scan=None;self.recovery_exporter.reset();self.chair_clearance=None;
        }
        if let Some(anchor)=self.anchor {
            let position:[f32;3]=unsafe { read_at(player,0x54) };
            self.suspended=Some((anchor,position));
            let _=write_log(Role::Skyrim,format!("event=SCENE_ALIGNMENT state=suspended sky={position:?}"));
        }
        unsafe { self.release(player); }
    }
    unsafe fn release(&mut self, player: Object) {
        crate::damage::suspend();
        self.interaction.block();self.attack.block();
        crate::overlay::allow(None);
        crate::scene::align(None);
        if let Some(api)=ADDRESSES.get().and_then(|a|a.look.as_ref()) { unsafe { self.look.release(api); } }
        unsafe {self.restore_gravity(player);}
        self.crossed=false;
        self.seat_watch.reset();
        if self.anchor.is_some() {
            let _ = write_log(Role::Skyrim,"event=RELEASED");
            self.released+=1;
            self.exporter.reset();
            if let Some(link)=self.link.as_mut() {
                if let Ok(snapshot)=link.endpoint.send_world(self.exporter.empty()) {
                    let _=snapshot.log(Role::Skyrim,"SEND");
                }
            }
        }
        self.gravity = None; self.anchor = None; self.last = None;self.chair_held=false;self.chair_hold_failed=false;
    }
    unsafe fn restore_gravity(&mut self,player:Object) {
        if let Some((saved,gravity))=self.gravity.take() {
            if !player.is_null() {
                let ctrl=unsafe {controller(player)};
                if !ctrl.is_null() && ctrl as usize==saved {
                    unsafe {ptr::write_unaligned(ctrl.add(0x248).cast::<f32>(),gravity);}
                }
            }
        }
    }
    /// Existing normal steering writes, also used after a chair is fully standing.
    unsafe fn place(&mut self,player:Object,target:[f32;3])->bool {
        let ctrl=unsafe {controller(player)};
        if ctrl.is_null() {return false;}
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
        true
    }
    /// Hold only a standing actor at the pre-seat point. Native furniture still
    /// owns every nonzero sit state. This prevents free exit physics during scan/ack.
    unsafe fn hold_chair(&mut self,player:Object,sample:Sample,entry:[f64;3])->io::Result<Option<[f32;3]>> {
        if unsafe {crate::seat::of(player)}!=0 {return Ok(None);}
        let Some(target)=self.anchor.and_then(|a|a.target(Sample{mc:entry,..sample})) else {return Ok(None);};
        let before:[f32;3]=unsafe {read_at(player,0x54)};
        let ctrl=unsafe {controller(player)};
        if ctrl.is_null() {return Ok(None);}
        let controller_before:[f32;3]=unsafe {[read_at(ctrl,0x240),read_at(ctrl,0x244),read_at(ctrl,0x248)]};
        if !unsafe {self.place(player,target)} {return Ok(None);}
        let actual:[f32;3]=unsafe {read_at(player,0x54)};
        let controller_after:[f32;3]=unsafe {[read_at(ctrl,0x240),read_at(ctrl,0x244),read_at(ctrl,0x248)]};
        let placed=(0..3).all(|i|(actual[i]-target[i]).abs()<=0.5)
            && (controller_after[0]-target[2]).abs()<=0.5 && controller_after[1]==0.0 && controller_after[2]==0.0;
        if !placed {
            if !self.chair_hold_failed {write_log(Role::Skyrim,format!("event=SCENE_CHAIR_RETURN state=not_placed entry={entry:?} native_before={before:?} requested={target:?} native_actual={actual:?} controller_after={controller_after:?}"))?;}
            self.chair_hold_failed=true;self.chair_held=false;return Ok(None);
        }
        if !self.chair_held {
            self.chair_held=true;self.chair_hold_failed=false;
            write_log(Role::Skyrim,format!("event=SCENE_CHAIR_RETURN state=held entry={entry:?} native_before={before:?} requested={target:?} native_actual={actual:?} controller_before={controller_before:?} controller_after={controller_after:?}"))?;
        }
        crate::scene::align(Some((sample,self.anchor.unwrap())));
        Ok(Some(target))
    }
    /// No position/controller writes: Skyrim owns its furniture until standing/aligned.
    unsafe fn publish_seat(&mut self,player:Object,sample:Sample,addresses:&Addresses,now:u64)->io::Result<()> {
        unsafe {self.restore_gravity(player);}
        self.seat_watch.reset();self.interaction.consume(sample);self.attack.block();
        crate::overlay::allow(Some(sample));crate::scene::align(Some((sample,self.anchor.unwrap())));
        let key=(sample.sender,sample.world,sample.frame);
        if let Some(api)=addresses.look.as_ref() {unsafe {self.look.apply(api,player,sample,self.last!=Some(key))?;}}
        self.last=Some(key);
        unsafe {crate::damage::arm(player,sample)?;}
        // Fresh controls/crosshair/health every 125 ms without running collision scans while held.
        if now.saturating_sub(self.seat_published_ms)<125 {return Ok(());}
        self.seat_published_ms=now;
        let cell=unsafe {read_at::<Object>(player,0x60)};
        let space=if cell.is_null() {ptr::null_mut()} else {unsafe {read_at::<Object>(cell,0x128)}};
        let mut reply=self.exporter.empty();
        reply.active=true;reply.admitted=true;reply.radius=4.0;reply.center=sample.mc;
        reply.recipient=sample.sender;reply.world=sample.world;reply.save_id=sample.save_id;
        reply.seated=true;reply.player_health=unsafe {probe_health(player)};
        reply.cell_id=if cell.is_null() {0} else {unsafe {read_at(cell,0x14)}};
        reply.worldspace_id=if space.is_null() {0} else {unsafe {read_at(space,0x14)}};
        crate::damage::snapshot(&mut reply);unsafe {crate::attack::snapshot(&mut reply,player);}
        self.link.as_mut().unwrap().endpoint.send_world(reply)?;
        Ok(())
    }
    /// Scan the arrival before issuing a teleport. Held replies never permit movement.
    unsafe fn recover(&mut self,player:Object,sample:Sample,actual:[f32;3],raw:[f64;3],cell:u32,space:u32,addresses:&Addresses,chair:bool)->io::Result<crate::recovery::Reply> {
        use crate::recovery::Reply;
        let key=(sample.sender,sample.world,sample.save_id,sample.mc_id,crate::save_identity::revision(),cell,space);
        if self.recovery_scan.is_some_and(|old|old!=key) {self.recovery.clear();}
        if self.recovery.pending() {
            if chair {
                let target=self.recovery.target().unwrap();
                let mut current=crate::world::Snapshot::default();
                if let Some(api)=addresses.world.as_ref() {
                    let native_cell=unsafe {read_at::<Object>(player,0x60)};
                    unsafe {self.recovery_exporter.npcs(api,player,native_cell,self.anchor.unwrap(),Sample{mc:target,..sample},&mut current)?;}
                }
                let now=crate::windows::now_ms();
                let valid=self.chair_clearance.as_ref().is_some_and(|(at,boxes)|now>=*at && now-*at<=500
                    && crate::recovery::chair_destination_clear(target,boxes,&current.npcs));
                if !valid {
                    write_log(Role::Skyrim,format!("event=SCENE_CHAIR_RETURN state=invalidated target={target:?} npc={:?} reason=occupied_or_scan_expired",crate::recovery::blocking_npc(target,&current.npcs)))?;
                    self.recovery.clear();self.recovery_scan=None;self.recovery_exporter.reset();self.chair_clearance=None;
                    return Ok(Reply::Waiting(0,raw));
                }
            }
            let result=self.recovery.poll(sample,actual,raw,cell,space);
            if matches!(result,Reply::Complete) {self.recovery_scan=None;self.recovery_exporter.reset();self.chair_clearance=None;}
            return Ok(result);
        }
        let mut target=raw;
        if let Some(api)=addresses.world.as_ref().filter(|_|chair || crate::switches::on(Role::Skyrim,crate::switches::SAFE_ARRIVAL)) {
            if self.recovery_scan!=Some(key) {
                self.recovery_scan=Some(key);self.recovery_exporter.reset();self.recovery_blocked=false;
            }
            let local=Sample{mc:raw,..sample};
            let anchor=if chair {self.anchor.unwrap()} else {Anchor::new(local,actual)};
            let Some(scan)= (unsafe {self.recovery_exporter.frame(api,player,anchor,local)?}) else {return Ok(Reply::Waiting(0,raw));};
            if !scan.active {return Ok(Reply::Waiting(0,raw));}
            let clear=if chair {crate::recovery::chair_return(raw,&scan.boxes,&scan.npcs)} else {crate::recovery::clear_feet(raw,&scan.boxes)};
            let Some(clear)=clear else {
                if !self.recovery_blocked {
                    write_log(Role::Skyrim,format!("event=SCENE_RECOVERY state=terrain_blocked target={raw:?} boxes={}",scan.boxes.len()))?;
                    if chair {
                        write_log(Role::Skyrim,format!("event=SCENE_CHAIR_RETURN state=blocked entry={raw:?} native_held={actual:?} terrain={:?} npc={:?} boxes={} npcs={}",crate::recovery::blocking_terrain(raw,&scan.boxes),crate::recovery::blocking_npc(raw,&scan.npcs),scan.boxes.len(),scan.npcs.len()))?;
                    }
                    self.recovery_blocked=true;
                }
                if chair {self.recovery_exporter.reset();self.chair_clearance=None;}
                return Ok(Reply::Failed);
            };
            target=clear;
            if chair {self.chair_clearance=Some((crate::windows::now_ms(),scan.boxes.clone()));write_log(Role::Skyrim,format!("event=SCENE_CHAIR_RETURN state=clear entry={raw:?} target={target:?} npcs={}",scan.npcs.len()))?;}
            write_log(Role::Skyrim,format!("event=SCENE_RECOVERY state=terrain_checked raw={raw:?} target={target:?} boxes={}",scan.boxes.len()))?;
        }
        Ok(self.recovery.poll(sample,actual,target,cell,space))
    }
    /// True means this frame has been held; false permits the ordinary steering path.
    unsafe fn seating(&mut self,player:Object,sample:Sample,addresses:&Addresses,ui:Object,cell:usize,now:u64,state:u32)->io::Result<bool> {
        use crate::seat::SitDecision;
        let action=self.sitting.poll(sample,state,now);
        match action {
            SitDecision::Idle=>return Ok(false),
            SitDecision::Failed=>{
                write_log(Role::Skyrim,format!("event=SCENE_SITTING state=activation_failed sit={state}"))?;
                self.seat_watch.reset();return Ok(false);
            }
            SitDecision::Align=>{
                self.interaction.consume(sample);self.attack.block();self.seat_watch.reset();
                let entry=if addresses.world.is_some() && crate::switches::on(Role::Skyrim,crate::switches::CHAIR_RETURN)
                    && self.chair_scope==Some((cell,crate::save_identity::revision())) {self.sitting.entry(sample)} else {None};
                let (actual,target)=if let Some(entry)=entry {
                    let Some(held)= (unsafe {self.hold_chair(player,sample,entry)?}) else {return Ok(true);};
                    (held,entry)
                } else {
                    unsafe {self.restore_gravity(player);}
                    let actual:[f32;3]=unsafe {read_at(player,0x54)};
                    (actual,self.anchor.unwrap().minecraft(actual.map(f64::from)))
                };
                let cell_ptr=cell as Object;
                let cell_id=unsafe {read_at(cell_ptr,0x14)};
                let space=unsafe {read_at::<Object>(cell_ptr,0x128)};
                let space_id=if space.is_null() {0} else {unsafe {read_at(space,0x14)}};
                match unsafe {self.recover(player,sample,actual,target,cell_id,space_id,addresses,entry.is_some())?} {
                    crate::recovery::Reply::Complete=>{
                        self.sitting.reset();self.exporter.reset();self.seat_published_ms=0;self.chair_scope=None;self.chair_held=false;self.chair_hold_failed=false;self.chair_clearance=None;
                        write_log(Role::Skyrim,format!("event=SCENE_SITTING state=standing_aligned mc={:?} sky={actual:?}",sample.mc))?;
                        return Ok(false);
                    }
                    pending=>{
                        let mut reply=self.exporter.empty();
                        reply.active=true;reply.radius=4.0;reply.recipient=sample.sender;
                        reply.world=sample.world;reply.save_id=sample.save_id;reply.origin_block=5;
                        if let crate::recovery::Reply::Waiting(token,target)=pending {
                            reply.recovery_token=token;reply.recovery_target=target;
                        }
                        self.link.as_mut().unwrap().endpoint.send_world(reply)?;
                        return Ok(true);
                    }
                }
            }
            SitDecision::Stand(attempt)=>{
                unsafe {self.restore_gravity(player);}
                if let Some(address)=addresses.stand_up {
                    write_log(Role::Skyrim,format!("event=SCENE_SITTING state=stand_before attempt={attempt} sit={state} controls={} sequence={}",sample.controls,sample.interact_seq))?;
                    let stop:unsafe extern "C" fn(Object,bool)=unsafe {mem::transmute(address)};
                    unsafe {stop(player,true);}
                    write_log(Role::Skyrim,format!("event=SCENE_SITTING state=stand_after attempt={attempt} sit={}",unsafe {crate::seat::of(player)}))?;
                    if RESET.load(Ordering::Acquire) || !READY.load(Ordering::Acquire)
                        || unsafe {crate::interaction::menu_blocks(ui)}
                        || unsafe {read_at::<Object>(player,0x60)} as usize!=cell {
                        unsafe {self.suspend(player);}return Ok(true);
                    }
                }
            }
            SitDecision::GaveUp=>write_log(Role::Skyrim,format!("event=SCENE_SITTING state=gave_up attempts=5 sit={state}"))?,
            SitDecision::Held=>{},
        }
        unsafe {self.publish_seat(player,sample,addresses,now)?;}
        Ok(true)
    }
    unsafe fn frame(&mut self, player: Object) -> io::Result<()> {
        if player.is_null() { return Ok(()); }
        let cell=unsafe { read_at::<Object>(player,0x60) } as usize;
        // A new cell is judged further down, once this frame's Minecraft position is known:
        // the next outdoor cell of the same world keeps the character steered, anything else
        // lets go. Every early return below lets go by itself.
        if self.cell!=cell { self.crossed=true; self.cell=cell; }
        let now = crate::windows::now_ms();
        if RESET.swap(false,Ordering::AcqRel) {
            self.recovery.clear();self.recovery_scan=None;self.recovery_exporter.reset();crate::damage::reset();self.death.loaded();self.sitting.reset();
            self.suspended=None;
            unsafe { self.release(player); }
        } else if now.saturating_sub(self.last_update_ms) > crate::position::MAX_SAMPLE_AGE_MS {
            unsafe { self.suspend(player); }
        }
        self.last_update_ms = now;
        let addresses = ADDRESSES.get().ok_or_else(|| io::Error::other("missing game addresses"))?;
        let ui = unsafe { *(addresses.ui as *const Object) };
        let paused = unsafe {crate::interaction::menu_blocks(ui)};
        if self.menu_blocked!=paused {
            self.menu_blocked=paused;
            write_log(Role::Skyrim,format!("event=SCENE_INTERACT_MENU blocked={paused}"))?;
        }
        if self.failed || !READY.load(Ordering::Acquire) {
            self.suspended=None;
            unsafe { self.release(player); } return Ok(());
        }
        if paused { unsafe { self.suspend(player); } return Ok(()); }
        if self.link.is_none() { self.link = Some(GameLink::open(Role::Skyrim)?); }
        let link = self.link.as_mut().unwrap(); link.poll()?;
        let Some(sample) = link.endpoint.receive_position()? else {
            // A paused Minecraft keeps its damage state and protects the player; a missing
            // one forgets it, and Skyrim's own damage applies.
            let idle=link.endpoint.idle_position()?;
            self.recovery.clear();
            match idle {
                Some((sender,world))=>crate::damage::pause(sender,world),
                None=>crate::damage::reset(),
            }
            unsafe { self.suspend(player); } return Ok(());
        };
        let loaded=crate::save_identity::published_id();
        let admitted=loaded!=[0;16] && loaded==sample.save_id;
        let decision=(admitted,sample.save_id,loaded);
        if self.admission!=Some(decision) {
            write_log(Role::Skyrim,format!("event=SCENE_ADMISSION allowed={} expected={:02X?} loaded={:02X?}",admitted,sample.save_id,loaded))?;
            self.admission=Some(decision);
        }
        if !admitted {
            self.sitting.reset();
            self.recovery.clear();crate::damage::reset();
            self.suspended=None;
            unsafe {self.release(player);}
            let mut reply=self.exporter.empty();
            reply.active=true;reply.radius=4.0;reply.recipient=sample.sender;
            reply.world=sample.world;reply.save_id=loaded;
            self.link.as_mut().unwrap().endpoint.send_world(reply)?;
            return Ok(());
        }
        // A dead or dying player is never steered; a fresh Minecraft death kills it first.
        let verdict=unsafe {self.death.check(player,sample)}?;
        if !matches!(verdict,crate::death::Verdict::Play) {
            self.sitting.reset();
            self.recovery.clear();crate::damage::reset();
            self.suspended=None;
            unsafe {self.release(player);}
            if matches!(verdict,crate::death::Verdict::Kill) {unsafe {self.death.kill(player,sample)}?;}
            let mut reply=self.exporter.empty();
            reply.active=true;reply.radius=4.0;reply.recipient=sample.sender;
            reply.world=sample.world;reply.save_id=loaded;reply.origin_block=crate::world::HOLD_DEAD;
            self.link.as_mut().unwrap().endpoint.send_world(reply)?;
            return Ok(());
        }
        let current: [f32;3] = unsafe { read_at(player,0x54) };
        if mem::take(&mut self.crossed) {
            if let Some(anchor)=self.anchor {
                let cell_ptr=cell as Object;
                let (cell_id,world_id):(u32,u32)=if cell_ptr.is_null() {(0,0)} else {
                    let world:Object=unsafe {read_at(cell_ptr,0x128)};
                    (unsafe {read_at(cell_ptr,0x14)},if world.is_null() {0} else {unsafe {read_at(world,0x14)}})
                };
                let distance=anchor.target(sample).map(|t|(0..3).map(|i|(t[i] as f64-current[i] as f64).powi(2)).sum::<f64>().sqrt());
                if stays_steered(self.steered_world,world_id,distance) {
                    write_log(Role::Skyrim,format!("event=SCENE_ALIGNMENT state=cell_crossed cell_id={cell_id:08X} worldspace_id={world_id:08X} distance={:.1}",distance.unwrap_or(f64::NAN)))?;
                } else {
                    self.suspended=None;
                    unsafe { self.release(player); }
                }
            }
        }
        if self.anchor.is_none_or(|a| !a.matches(sample)) {
            self.suspended=None;
            unsafe { self.release(player); }
            let cell_ptr=cell as Object;
            if cell_ptr.is_null() {return Ok(());}
            let cell_id:u32=unsafe {read_at(cell_ptr,0x14)};
            let worldspace:Object=unsafe {read_at(cell_ptr,0x128)};
            let worldspace_id=if worldspace.is_null() {0} else {unsafe {read_at(worldspace,0x14)}};
            let mut initial=crate::save_identity::origin(sample,current,cell_id,worldspace_id);
            if matches!(initial,Err(5)) || self.recovery.pending() {
                if let Some(target)=crate::save_identity::recovery_target(sample,current,cell_id,worldspace_id) {
                    match unsafe {self.recover(player,sample,current,target,cell_id,worldspace_id,addresses,false)?} {
                        crate::recovery::Reply::Complete=>{initial=crate::save_identity::origin(sample,current,cell_id,worldspace_id);},
                        pending=>{
                            let mut reply=self.exporter.empty();
                            reply.active=true;reply.radius=4.0;reply.recipient=sample.sender;
                            reply.world=sample.world;reply.save_id=loaded;reply.origin_block=5;
                            if let crate::recovery::Reply::Waiting(token,target)=pending {
                                reply.recovery_token=token;reply.recovery_target=target;
                            }
                            self.link.as_mut().unwrap().endpoint.send_world(reply)?;
                            return Ok(());
                        }
                    }
                } else {self.recovery.clear();crate::damage::reset();}
            }
            match initial {
                Ok(anchor)=>{
                    self.anchor=Some(anchor);self.origin_block=Some(0);self.steered_world=worldspace_id;
                    let (mc_origin,sky_origin)=anchor.coordinates();
                    write_log(Role::Skyrim,format!("event=SCENE_ALIGNMENT state=origin_ready sender={} world={} mc_origin={:?} sky_origin={:?}",
                        sample.sender,sample.world,mc_origin,sky_origin))?;
                }
                Err(reason)=>{
                    if self.origin_block!=Some(reason) {
                        write_log(Role::Skyrim,format!("event=SCENE_ORIGIN state=blocked reason={} mc_id={:02X?} cell_id={:08X} worldspace_id={:08X} mc={:?} sky={:?}",
                            reason,sample.mc_id,cell_id,worldspace_id,sample.mc,current))?;
                        self.origin_block=Some(reason);
                    }
                    let mut reply=self.exporter.empty();
                    reply.active=true;reply.radius=4.0;reply.recipient=sample.sender;
                    reply.world=sample.world;reply.save_id=loaded;reply.origin_block=reason;
                    self.link.as_mut().unwrap().endpoint.send_world(reply)?;
                    return Ok(());
                }
            }
        }
        let target = self.anchor.unwrap().target(sample).ok_or_else(|| io::Error::other("invalid puppet target"))?;
        let mut ctrl = unsafe { controller(player) };
        if ctrl.is_null() { unsafe { self.release(player); } return Ok(()); }
        // Log the sit state before choosing furniture ownership or ordinary movement:
        // anything but 0 means Skyrim has it using furniture or a mount (seat.rs).
        let seat=unsafe {crate::seat::of(player)};
        if self.seat!=Some(seat) {
            write_log(Role::Skyrim,format!("event=SCENE_SEAT state={seat} name={} was={}",crate::seat::label(seat),
                self.seat.map_or_else(||"unseen".to_owned(),|old|old.to_string())))?;
            self.seat=Some(seat);
        }
        if let Some(api)=addresses.interaction.as_ref() {
            if self.interaction.grounding() {
                unsafe {self.restore_gravity(player);}
                if unsafe {self.interaction.poll_ground(api,player,sample)?} {
                    unsafe {self.publish_seat(player,sample,addresses,now)?;}return Ok(());
                }
                if unsafe {crate::interaction::menu_blocks(ui)} || !READY.load(Ordering::Acquire)
                    || RESET.load(Ordering::Acquire) || unsafe {read_at::<Object>(player,0x60)} as usize!=cell {
                    unsafe {self.suspend(player);}return Ok(());
                }
            }
        }
        if unsafe {self.seating(player,sample,addresses,ui,cell,now,crate::seat::of(player))?} {return Ok(());}
        if crate::switches::on(Role::Skyrim,crate::switches::STAND_UP) {
            if let Some(address)=addresses.stand_up {
                match self.seat_watch.look(seat,now) {
                    Some(crate::seat::Decision::Stand(attempt))=>{
                        // Write before entering native code: a crash must leave this trace.
                        write_log(Role::Skyrim,format!("event=SCENE_STAND_UP state=before attempt={attempt} sit={seat} name={}",crate::seat::label(seat)))?;
                        let stop:unsafe extern "C" fn(Object,bool)=unsafe {mem::transmute(address)};
                        unsafe {stop(player,true);}
                        let after=unsafe {crate::seat::of(player)};
                        write_log(Role::Skyrim,format!("event=SCENE_STAND_UP state=after attempt={attempt} sit={after} name={}",crate::seat::label(after)))?;
                        // The interaction-ending call can change the controller or game state.
                        if RESET.load(Ordering::Acquire) || !READY.load(Ordering::Acquire)
                            || unsafe {crate::interaction::menu_blocks(ui)}
                            || unsafe {read_at::<Object>(player,0x60)} as usize!=cell {
                            unsafe {self.suspend(player);}
                            return Ok(());
                        }
                        ctrl=unsafe {controller(player)};
                        if ctrl.is_null() {unsafe {self.release(player);} return Ok(());}
                    }
                    Some(crate::seat::Decision::GaveUp)=>{
                        write_log(Role::Skyrim,format!("event=SCENE_STAND_UP state=gave_up attempts=5 sit={seat} name={}",crate::seat::label(seat)))?;
                    }
                    None=>{}
                }
            }
        } else {self.seat_watch.reset();}
        if !unsafe {self.place(player,target)} {return Ok(());}
        crate::overlay::allow(Some(sample));
        crate::scene::align(Some((sample,self.anchor.unwrap())));
        let key = (sample.sender,sample.world,sample.frame);
        if let Some(api)=addresses.look.as_ref() {
            unsafe { self.look.apply(api,player,sample,self.last!=Some(key))?; }
        }
        if let Some(api)=addresses.interaction.as_ref() {
            unsafe {self.interaction.apply(api,player,sample)?;}
            if self.interaction.grounding() {
                self.sitting.arm_native(sample,now);self.seat_published_ms=0;self.chair_scope=None;self.chair_held=false;self.chair_hold_failed=false;self.chair_clearance=None;
                unsafe {self.restore_gravity(player);}
            }
            if self.interaction.take_chair() {
                self.sitting.arm(sample,now);self.seat_published_ms=0;self.chair_scope=Some((cell,crate::save_identity::revision()));self.chair_held=false;self.chair_hold_failed=false;self.chair_clearance=None;
                unsafe {self.restore_gravity(player);}
                self.seat_watch.reset();
                write_log(Role::Skyrim,format!("event=SCENE_SITTING state=requested sequence={} mc={:?} sky={:?}",sample.interact_seq,sample.mc,unsafe {read_at::<[f32;3]>(player,0x54)}))?;
            }
            // Activation can open UI or load a cell synchronously. Stop this frame.
            if RESET.load(Ordering::Acquire) || !READY.load(Ordering::Acquire)
                || unsafe {crate::interaction::menu_blocks(ui)}
                || unsafe {read_at::<Object>(player,0x60)} as usize!=cell {
                unsafe {self.suspend(player);}
                return Ok(());
            }
        }
        if self.sitting.active() {
            unsafe {self.publish_seat(player,sample,addresses,now)?;}
            return Ok(());
        }
        if self.last != Some(key) {
            let actual: [f32;3] = unsafe { read_at(player,0x54) };
            log_received(sample,target,actual)?;
            self.last = Some(key);
        }
        unsafe {self.attack.apply(player,sample,addresses.world.as_ref())?;}
        if RESET.load(Ordering::Acquire) || !READY.load(Ordering::Acquire)
            || unsafe {crate::interaction::menu_blocks(ui)}
            || unsafe {read_at::<Object>(player,0x60)} as usize!=cell {
            unsafe {self.suspend(player);}
            return Ok(());
        }
        unsafe {crate::damage::arm(player,sample)?;}
        if let Some(api)=addresses.world.as_ref() {
            if let Some(mut snapshot)=unsafe { self.exporter.frame(api,player,self.anchor.unwrap(),sample)? } {
                crate::damage::snapshot(&mut snapshot);
                unsafe {crate::attack::snapshot(&mut snapshot,player); }
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
unsafe fn probe_health(player:Object)->f32 {
    let owner=unsafe {player.add(0xB8)};
    let table:*const usize=unsafe {read_at(owner,0)};
    let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
    unsafe {get(owner,24)}
}
unsafe extern "C" fn damage_probe(player:Object,attacker:Object,amount:f32) {
    let original=ORIGINAL_DAMAGE.load(Ordering::Acquire);
    if original==0 {return;}
    let call:unsafe extern "C" fn(Object,Object,f32)=unsafe {mem::transmute(original)};
    let seq=DAMAGE_SEQUENCE.fetch_add(1,Ordering::Relaxed);
    let before=if player.is_null() {f32::NAN} else {unsafe {probe_health(player)}};
    let attacker_id=if attacker.is_null() {0} else {unsafe {read_at::<u32>(attacker,0x14)}};
    unsafe {call(player,attacker,amount);}
    let after=if player.is_null() {f32::NAN} else {unsafe {probe_health(player)}};
    let _=std::panic::catch_unwind(|| {
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_PROBE sequence={seq} supplied={amount} before={before} after={after} measured_loss={} attacker={attacker_id:08X} revision={} thread={:?}",before-after,crate::save_identity::revision(),std::thread::current().id()));
    });
}
unsafe extern "C" fn update(player: Object, delta: f32) {
    let original = ORIGINAL.load(Ordering::Acquire);
    if original == 0 { return; }
    let original: Update = unsafe { mem::transmute(original) };
    unsafe { original(player,delta); }
    // Time the bridge's own work in this frame (not Skyrim's update above it).
    let began=std::time::Instant::now();
    let result = std::panic::catch_unwind(|| PUPPET.with(|state| unsafe { state.borrow_mut().frame(player) }));
    let micros=began.elapsed().as_micros() as u64;
    COST.with(|cost| {
        if let Some(line)=cost.borrow_mut().add(micros,crate::windows::now_ms()) {
            let _=write_log(Role::Skyrim,format!("event=SCENE_COST side=skyrim {line}"));
        }
    });
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
    fn chair_hold_places_only_standing_actor_and_restores_original_gravity() {
        static IGNORE:AtomicBool=AtomicBool::new(false);
        unsafe extern "C" fn position(player:Object,at:*const [f32;3],_:bool) {
            if IGNORE.load(Ordering::Acquire) {return;}
            unsafe {ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),*at);}
        }
        unsafe extern "C" fn velocity(ctrl:Object,at:*const Vector4) {
            unsafe {ptr::write_unaligned(ctrl.add(0x100).cast::<[f32;4]>(),(*at).0);}
        }
        let mut actor=[0usize;128];let player=actor.as_mut_ptr().cast::<u8>();
        let mut process=[0usize;4];let mut middle=[0usize;80];let mut data=[0usize;100];
        let ctrl=data.as_mut_ptr().cast::<u8>();
        let mut player_table=[0usize;0xAE];player_table[0xA9]=position as *const () as usize;
        let ctrl_table=[0usize,0,0,0,0,0,0,velocity as *const () as usize];
        unsafe {
            ptr::write(player.cast::<usize>(),player_table.as_ptr() as usize);
            ptr::write_unaligned(player.add(0xF8).cast::<usize>(),process.as_mut_ptr() as usize);
            ptr::write(process.as_mut_ptr().add(1),middle.as_mut_ptr() as usize);
            ptr::write(middle.as_mut_ptr().add(0x250/8),ctrl as usize);
            ptr::write(ctrl.cast::<usize>(),ctrl_table.as_ptr() as usize);
            ptr::write_unaligned(ctrl.add(0x248).cast::<f32>(),9.8);
            ptr::write_unaligned(ctrl.add(0x100).cast::<[f32;4]>(),[4.0;4]);
            ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),[200.0,-40.0,-70.0]);
        }
        let sample=Sample{sender:7,world:2,frame:1,mc:[10.0,128.125,20.0],active:1,..Default::default()};
        let anchor=Anchor::new(sample,[100.0,200.0,300.0]);
        let mut puppet=Puppet{anchor:Some(anchor),..Default::default()};
        unsafe {ptr::write_unaligned(player.add(0xC8).cast::<u32>(),3<<14);}
        assert_eq!(unsafe {puppet.hold_chair(player,sample,sample.mc)}.unwrap(),None);
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[200.0,-40.0,-70.0]);
        assert_eq!(unsafe {read_at::<f32>(ctrl,0x248)},9.8);
        unsafe {ptr::write_unaligned(player.add(0xC8).cast::<u32>(),0);}
        IGNORE.store(true,Ordering::Release);
        assert_eq!(unsafe {puppet.hold_chair(player,sample,sample.mc)}.unwrap(),None);
        assert!(puppet.chair_hold_failed);assert!(!puppet.chair_held);
        IGNORE.store(false,Ordering::Release);
        assert_eq!(unsafe {puppet.hold_chair(player,sample,sample.mc)}.unwrap(),Some([100.0,200.0,300.0]));
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[100.0,200.0,300.0]);
        assert_eq!(unsafe {read_at::<[f32;4]>(ctrl,0x100)},[0.0;4]);
        assert_eq!(unsafe {read_at::<f32>(ctrl,0x240)},300.0);
        assert_eq!(unsafe {read_at::<f32>(ctrl,0x244)},0.0);
        assert_eq!(unsafe {read_at::<f32>(ctrl,0x248)},0.0);
        // Repeated pin must not replace the saved gravity with its temporary zero.
        unsafe {puppet.hold_chair(player,sample,sample.mc)}.unwrap();
        unsafe {puppet.restore_gravity(player);}
        assert_eq!(unsafe {read_at::<f32>(ctrl,0x248)},9.8);
        let foreign=Sample{world:3,..sample};
        assert_eq!(unsafe {puppet.hold_chair(player,foreign,sample.mc)}.unwrap(),None);
        puppet.chair_clearance=Some((crate::windows::now_ms(),vec![]));
        puppet.recovery_scan=Some((7,2,[0;16],[0;16],0,123,0));
        unsafe {puppet.suspend(player);}
        assert!(puppet.chair_clearance.is_none());assert!(puppet.recovery_scan.is_none());
        assert!(!puppet.recovery.pending());assert!(puppet.anchor.is_none());
        crate::scene::align(None);
    }
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
        static STAND_QUICK:AtomicBool=AtomicBool::new(false);
        unsafe extern "C" fn stand(player:Object,quick:bool) {
            STAND_QUICK.store(quick,Ordering::Release);
            let word:u32=unsafe {read_at(player,0xC8)};
            unsafe {ptr::write_unaligned(player.add(0xC8).cast::<u32>(),word&!(0xF<<14));}
        }
        // A loaded chair reference with the same can-sit marker as the meadery stool.
        static CHAIR:AtomicUsize=AtomicUsize::new(0);
        unsafe extern "C" fn lookup(_handle:*const u32,out:*mut Object)->bool {
            let target=CHAIR.load(Ordering::Acquire) as Object;
            unsafe {(&*target.add(0x28).cast::<std::sync::atomic::AtomicU32>()).fetch_add(1,Ordering::SeqCst);*out=target;}
            true
        }
        static MIDAIR:AtomicBool=AtomicBool::new(false);
        static ACTIVATIONS:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(0);
        unsafe extern "C" fn midair(_player:Object)->bool {MIDAIR.load(Ordering::Acquire)}
        unsafe extern "C" fn activate(player:Object) {
            ACTIVATIONS.fetch_add(1,Ordering::AcqRel);
            if unsafe {crate::seat::furniture(CHAIR.load(Ordering::Acquire) as Object)}.2&(1<<31)!=0 {return;}
            unsafe {
                ptr::write_unaligned(player.add(0xC8).cast::<u32>(),3<<14);
                ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),[180.0,-60.0,335.0]);
            }
        }
        let mut furniture=[0_usize;32];let record=furniture.as_mut_ptr().cast::<u8>();
        let mut chair=[0_usize;16];let chair_ptr=chair.as_mut_ptr().cast::<u8>();
        let mut pick=[0_usize;2];let pick_slot=Box::new(pick.as_mut_ptr() as usize);
        unsafe {
            *record.add(0x1A)=0x28;
            ptr::write_unaligned(record.add(0xE4).cast::<u32>(),0x40000001);
            ptr::write_unaligned(chair_ptr.add(0x40).cast::<Object>(),record);
            ptr::write_unaligned(chair_ptr.add(0x28).cast::<u32>(),1);
            ptr::write_unaligned(pick.as_mut_ptr().cast::<u8>().add(4).cast::<u32>(),123);
        }
        CHAIR.store(chair_ptr as usize,Ordering::Release);
        let mut actor=[0_usize;128]; let player=actor.as_mut_ptr().cast::<u8>();
        let mut process=[0_usize;4]; let mut middle=[0_usize;80];
        let mut controller_data=[0_usize;100]; let ctrl=controller_data.as_mut_ptr().cast::<u8>();
        let mut actor_vtable=[0_usize;0xAE]; actor_vtable[0xA9]=set_position as *const () as usize;
        let mut ctrl_vtable=[0_usize;8]; ctrl_vtable[7]=set_velocity as *const () as usize;
        // Since 0.8: a paired save lineage, a cell to key the origin, and (0.10.6) readable health.
        static HEALTH:std::sync::atomic::AtomicU32=std::sync::atomic::AtomicU32::new(0);
        HEALTH.store(100.0_f32.to_bits(),Ordering::Release);
        unsafe extern "C" fn health(_owner:Object,_value:u32)->f32 { f32::from_bits(HEALTH.load(Ordering::Acquire)) }
        unsafe extern "C" fn lethal(_player:Object,amount:f32,_source:Object,_plain:bool)->bool {
            HEALTH.store((100.0-amount).to_bits(),Ordering::Release); false
        }
        let owner_vtable=[0_usize,health as *const () as usize,0,0];
        let mut cell_data=[0_usize;64];
        unsafe { ptr::write_unaligned(cell_data.as_mut_ptr().cast::<u8>().add(0x14).cast::<u32>(),0x1234); }
        crate::save_identity::publish_for_test([7;16]);
        let mut ui_data=[0_usize;64]; let ui=ui_data.as_mut_ptr().cast::<u8>();
        let ui_slot=Box::new(ui as usize);
        ADDRESSES.set(Addresses{vtable:0,ui:(&*ui_slot) as *const usize as usize,render_window:None,hud_vtable:None,world:None,look:None,interaction:Some(crate::interaction::Api{activate:activate as *const () as usize,crosshair:(&*pick_slot) as *const usize as usize,lookup:lookup as *const () as usize,lock:None,midair:Some(midair as *const () as usize)}),stand_up:Some(stand as *const () as usize)}).ok().unwrap();
        unsafe {
            ptr::write(player.cast::<usize>(),actor_vtable.as_ptr() as usize);
            ptr::write_unaligned(player.add(0xF8).cast::<usize>(),process.as_mut_ptr() as usize);
            ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),[100.0,-200.0,300.0]);
            ptr::write_unaligned(player.add(0x60).cast::<usize>(),cell_data.as_ptr() as usize);
            ptr::write_unaligned(player.add(0xB8).cast::<usize>(),owner_vtable.as_ptr() as usize);
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
        let sample=Sample{world:1,frame:1,active:1,partial:0.5,mc:[10.0,64.0,20.0],save_id:[7;16],mc_id:[9;16],..Sample::default()};
        sender.send_position(sample).unwrap();
        READY.store(true,Ordering::Release);
        let mut puppet=Puppet{link:Some(receiver),..Puppet::default()};
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<[f32;3]>(player,0x54) },[100.0,-200.0,300.0]);
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x248) },0.0);
        // A loaded seated character stays movable, but after four continuous seconds
        // the real adapter calls the stand-up entry with true and observes standing.
        let now=crate::windows::now_ms();
        for elapsed in (0..4000).step_by(400) {
            assert_eq!(puppet.seat_watch.look(3,now-4000+elapsed),None);
        }
        unsafe {ptr::write_unaligned(player.add(0xC8).cast::<u32>(),3<<14);}
        sender.send_position(Sample{frame:2,mc:[11.0,64.5,18.0],..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert!(STAND_QUICK.load(Ordering::Acquire));
        assert_eq!(unsafe {crate::seat::of(player)},0);
        assert_eq!(unsafe { read_at::<[f32;3]>(player,0x54) },[170.0,-60.0,335.0]);
        assert_eq!(unsafe { read_at::<[f32;4]>(ctrl,0x100) },[0.0;4]);
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x240) },335.0);
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x244) },0.0);
        // Actual E dispatch on a classified chair arms the mode and restores gravity.
        STAND_QUICK.store(false,Ordering::Release);
        let sit=Sample{frame:3,mc:[11.0,64.5,18.0],interact_seq:7,interact_ms:crate::windows::now_ms(),..sample};
        sender.send_position(sit).unwrap();unsafe {puppet.frame(player).unwrap();}
        assert!(puppet.sitting.active());assert!(!STAND_QUICK.load(Ordering::Acquire));
        assert_eq!(unsafe {read_at::<f32>(ctrl,0x248)},9.8);
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[180.0,-60.0,335.0]);
        let held=sender.receive_world().unwrap().unwrap();assert!(held.admitted && held.seated);
        // Still seated on a later frame: zero movement writes, no four-second auto recovery.
        sender.send_position(Sample{frame:4,..sit}).unwrap();unsafe {puppet.frame(player).unwrap();}
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[180.0,-60.0,335.0]);
        assert!(!STAND_QUICK.load(Ordering::Acquire));
        // WASD requests native standing, but the actor is not moved to Minecraft yet.
        sender.send_position(Sample{frame:5,controls:1,..sit}).unwrap();unsafe {puppet.frame(player).unwrap();}
        assert!(STAND_QUICK.load(Ordering::Acquire));assert_eq!(unsafe {crate::seat::of(player)},0);
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[180.0,-60.0,335.0]);
        sender.send_position(Sample{frame:6,..sit}).unwrap();unsafe {puppet.frame(player).unwrap();}
        let exit=sender.receive_world().unwrap().unwrap();
        assert_eq!(exit.origin_block,5);assert!(!exit.admitted && !exit.seated);assert_ne!(exit.recovery_token,0);
        assert!((exit.recovery_target[0]-(11.0+10.0/70.0)).abs()<1e-6);
        sender.send_position(Sample{frame:7,mc:exit.recovery_target,recovery_ack:exit.recovery_token,..sit}).unwrap();
        unsafe {puppet.frame(player).unwrap();}
        assert!(!puppet.sitting.active());
        assert!((unsafe {read_at::<[f32;3]>(player,0x54)}[0]-180.0).abs()<0.01);
        // Repeated entering E never activates again after the handshake.
        assert_eq!(unsafe {crate::seat::of(player)},0);
        sender.send_position(Sample{frame:8,mc:[11.0,64.5,18.0],..sample}).unwrap();
        unsafe {puppet.frame(player).unwrap();}
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[170.0,-60.0,335.0]);
        // Owned bed: native gravity remains on until native grounded test clears;
        // dispatch happens once, then coordinates are aligned before steering resumes.
        unsafe {ptr::write_unaligned(record.add(0xE4).cast::<u32>(),0x88000001);}
        std::thread::sleep(std::time::Duration::from_millis(260));
        let sample=Sample{sent_ms:crate::windows::now_ms(),mc:[11.0,64.5,18.0],..sample};
        sender.send_position(Sample{frame:9,..sample}).unwrap();unsafe {puppet.frame(player).unwrap();}
        MIDAIR.store(true,Ordering::Release);let before=ACTIVATIONS.load(Ordering::Acquire);
        let bed=Sample{frame:10,mc:[11.0,64.5,18.0],interact_seq:50,interact_ms:crate::windows::now_ms(),..sample};
        sender.send_position(bed).unwrap();unsafe {puppet.frame(player).unwrap();}
        assert!(puppet.interaction.grounding());assert_eq!(unsafe {read_at::<f32>(ctrl,0x248)},9.8);
        unsafe {ptr::write_unaligned(player.add(0x54).cast::<[f32;3]>(),[170.0,-60.0,326.0]);}
        sender.send_position(Sample{frame:11,..bed}).unwrap();unsafe {puppet.frame(player).unwrap();}
        assert_eq!(ACTIVATIONS.load(Ordering::Acquire),before);
        assert_eq!(unsafe {read_at::<[f32;3]>(player,0x54)},[170.0,-60.0,326.0]);
        MIDAIR.store(false,Ordering::Release);
        sender.send_position(Sample{frame:12,..bed}).unwrap();unsafe {puppet.frame(player).unwrap();}
        assert_eq!(ACTIVATIONS.load(Ordering::Acquire),before+1);assert!(!puppet.interaction.grounding());
        let arrival=sender.receive_world().unwrap().unwrap();assert_ne!(arrival.recovery_token,0);
        sender.send_position(Sample{frame:13,mc:arrival.recovery_target,recovery_ack:arrival.recovery_token,..bed}).unwrap();
        unsafe {puppet.frame(player).unwrap();}
        assert!(!puppet.sitting.active());assert_eq!(ACTIVATIONS.load(Ordering::Acquire),before+1);
        // Resume ordinary steering to the earlier test position before its release checks.
        sender.send_position(Sample{frame:14,..sample}).unwrap();unsafe {puppet.frame(player).unwrap();}
        sender.send_position(Sample{frame:15,active:0,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x248) },9.8);
        assert!(puppet.anchor.is_none());
        // A far-away Minecraft position against the saved origin must not teleport Skyrim:
        // the adapter holds and asks Minecraft to recover instead (hold reason 5).
        sender.send_position(Sample{world:2,frame:16,mc:[9000.0,72.0,-4000.0],..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<[f32;3]>(player,0x54) },[170.0,-60.0,335.0]);
        std::thread::sleep(std::time::Duration::from_millis(300));
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(unsafe { read_at::<f32>(ctrl,0x248) },9.8);
        assert!(puppet.anchor.is_none());
        // A Minecraft death kills the player exactly once, and every following frame must
        // still succeed: the "player dead" hold reply has to pass the sender's own check.
        crate::death::configure(lethal as *const () as usize);
        let died=Sample{frame:17,death_seq:1,death_ms:crate::windows::now_ms(),..sample};
        sender.send_position(died).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(f32::from_bits(HEALTH.load(Ordering::Acquire)),-200.0);
        assert!(puppet.anchor.is_none());
        sender.send_position(Sample{frame:18,..died}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(f32::from_bits(HEALTH.load(Ordering::Acquire)),-200.0);
        // Skyrim reloads: health is back and the same death number must not kill again.
        HEALTH.store(100.0_f32.to_bits(),Ordering::Release);
        RESET.store(true,Ordering::Release);
        sender.send_position(Sample{frame:19,..died}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(f32::from_bits(HEALTH.load(Ordering::Acquire)),100.0);
        // A loading door: Skyrim's player is now in another interior cell with unrelated
        // coordinates. The bridge must hold, ask Minecraft to move into that cell's own
        // region, and resume there without moving Skyrim's player.
        let mut inn_data=[0_usize;64];
        unsafe {
            ptr::write_unaligned(inn_data.as_mut_ptr().cast::<u8>().add(0x14).cast::<u32>(),0x5678);
            ptr::write_unaligned(player.add(0x60).cast::<usize>(),inn_data.as_ptr() as usize);
        }
        sender.send_position(Sample{frame:20,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert!(puppet.anchor.is_none());
        let request=sender.receive_world().unwrap().expect("a hold reply carrying a recovery request");
        assert_eq!(request.origin_block,5);
        assert_ne!(request.recovery_token,0);
        let expected=[((0x678-2048)*2048) as f64+170.0/70.0,128.0+335.0/70.0,((0x5-2048)*2048) as f64+60.0/70.0];
        for axis in 0..3 { assert!((request.recovery_target[axis]-expected[axis]).abs()<1e-6); }
        sender.send_position(Sample{frame:21,mc:request.recovery_target,recovery_ack:request.recovery_token,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert!(puppet.anchor.is_some());
        let stayed=unsafe { read_at::<[f32;3]>(player,0x54) };
        for axis in 0..3 { assert!((stayed[axis]-[170.0,-60.0,335.0][axis]).abs()<0.01); }
        // Walking on inside the building moves Skyrim's player by 70 units per block.
        let mut step=request.recovery_target; step[0]+=1.0;
        sender.send_position(Sample{frame:22,mc:step,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        let moved=unsafe { read_at::<[f32;3]>(player,0x54) };
        assert!((moved[0]-240.0).abs()<0.01 && (moved[1]+60.0).abs()<0.01 && (moved[2]-335.0).abs()<0.01);
        // Outdoors: two neighbouring cells of world 3C and one cell of another world.
        let mut world_3c=[0_usize;8]; let mut world_3d=[0_usize;8];
        let mut outdoor_a=[0_usize;64]; let mut outdoor_b=[0_usize;64]; let mut elsewhere=[0_usize;64];
        unsafe {
            ptr::write_unaligned(world_3c.as_mut_ptr().cast::<u8>().add(0x14).cast::<u32>(),0x3C);
            ptr::write_unaligned(world_3d.as_mut_ptr().cast::<u8>().add(0x14).cast::<u32>(),0x3D);
            for (cell,id,world) in [(&mut outdoor_a,0x9656_u32,&world_3c),(&mut outdoor_b,0x9657,&world_3c),(&mut elsewhere,0x9999,&world_3d)] {
                let p=cell.as_mut_ptr().cast::<u8>();
                ptr::write_unaligned(p.add(0x14).cast::<u32>(),id);
                ptr::write_unaligned(p.add(0x128).cast::<usize>(),world.as_ptr() as usize);
            }
            // The inn's door leads outdoors: another place, so Minecraft is asked to move there.
            ptr::write_unaligned(player.add(0x60).cast::<usize>(),outdoor_a.as_ptr() as usize);
        }
        sender.send_position(Sample{frame:23,mc:step,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        let request=sender.receive_world().unwrap().expect("a recovery request for the outdoor world");
        assert_eq!(request.origin_block,5);
        let mut outdoors=request.recovery_target;
        sender.send_position(Sample{frame:24,mc:outdoors,recovery_ack:request.recovery_token,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert!(puppet.anchor.is_some());
        // Walking across the border into the next outdoor cell of the same world: the
        // character stays steered with the same origin, and still moves 70 units per block.
        let (released,origin)=(puppet.released,puppet.anchor.unwrap().coordinates());
        unsafe { ptr::write_unaligned(player.add(0x60).cast::<usize>(),outdoor_b.as_ptr() as usize); }
        outdoors[0]+=1.0;
        sender.send_position(Sample{frame:25,mc:outdoors,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(puppet.released,released);
        assert_eq!(puppet.anchor.unwrap().coordinates(),origin);
        let walked=unsafe { read_at::<[f32;3]>(player,0x54) };
        assert!((walked[0]-310.0).abs()<0.01 && (walked[1]+60.0).abs()<0.01,"walked to {walked:?}");
        // A door into a cell of another world lets go, even though the position did not jump.
        unsafe { ptr::write_unaligned(player.add(0x60).cast::<usize>(),elsewhere.as_ptr() as usize); }
        sender.send_position(Sample{frame:26,mc:outdoors,..sample}).unwrap();
        unsafe { puppet.frame(player).unwrap(); }
        assert_eq!(puppet.released,released+1);
        assert!(puppet.anchor.is_none());
        READY.store(false,Ordering::Release);
    }
    #[test]
    fn only_the_next_outdoor_cell_of_the_same_world_keeps_the_character_steered() {
        // Walking over a cell border outdoors (report 20261011-155446 held 0.1 s on each).
        assert!(stays_steered(0x3C,0x3C,Some(3.0)));
        assert!(stays_steered(0x3C,0x3C,Some(140.0)));
        // Fast travel inside the same world: the character is far from where Minecraft is.
        assert!(!stays_steered(0x3C,0x3C,Some(140.1)));
        assert!(!stays_steered(0x3C,0x3C,Some(250_000.0)));
        // A door to another world, into a building, out of one, or between two buildings.
        assert!(!stays_steered(0x3C,0x3D,Some(3.0)));
        assert!(!stays_steered(0x3C,0,Some(3.0)));
        assert!(!stays_steered(0,0x3C,Some(3.0)));
        assert!(!stays_steered(0,0,Some(3.0)));
        // A position that cannot be worked out never counts as "the same spot".
        assert!(!stays_steered(0x3C,0x3C,None));
        assert!(!stays_steered(0x3C,0x3C,Some(f64::NAN)));
    }
}







