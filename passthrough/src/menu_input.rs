//! Paused-menu input runs exclusively on SKSE's UI task thread.
use std::{cell::RefCell,ffi::c_void,io,mem,ptr,sync::{Mutex,TryLockError,OnceLock,atomic::{AtomicBool,Ordering}},time::Duration};
use crate::{Role,RECORD_SIZE,PROTOCOL_VERSION,position::Sample,windows::{SharedMemory,now_ms},native::write_log};
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,at:usize)->T { unsafe {ptr::read_unaligned(p.add(at).cast())} }

#[derive(Clone,Copy,Default)]
pub struct Status {pub active:bool,pub context:u64,pub ms:u64,pub sky:u64,pub mc:u64,pub world:u64}
impl Status {
    pub fn decode(b:&[u8;RECORD_SIZE])->Self { Self{active:crate::read_u32(b,320)==1,
        context:crate::read_u64(b,328),ms:crate::read_u64(b,336),sky:crate::read_u64(b,344),mc:crate::read_u64(b,352),world:crate::read_u64(b,360)} }
    pub fn encode(self,b:&mut [u8;RECORD_SIZE]) {
        b[320..324].copy_from_slice(&(self.active as u32).to_le_bytes());
        for (at,value) in [(328,self.context),(336,self.ms),(344,self.sky),(352,self.mc),(360,self.world)] {b[at..at+8].copy_from_slice(&value.to_le_bytes());}
    }
}
/// `cursor` is (MenuCursor singleton slot, MouseMoveEvent vtable); absent disables only the pointer.
pub struct Api {pub ui:usize,pub player:usize,pub controls:usize,pub input:usize,pub button:usize,pub string_ctor:usize,pub string_release:usize,
    pub cursor:Option<(usize,usize)>}
#[repr(C)]
struct Tasks {version:u32,add:usize,add_ui:unsafe extern "C" fn(*mut c_void)}
#[repr(C)]
struct Delegate {vtable:&'static Vtable}
#[repr(C)]
struct Vtable {run:unsafe extern "C" fn(*mut Delegate),dispose:unsafe extern "C" fn(*mut Delegate)}
static VTABLE:Vtable=Vtable{run,dispose};
static API:OnceLock<Api>=OnceLock::new();
static PENDING:AtomicBool=AtomicBool::new(false);
static STOP:AtomicBool=AtomicBool::new(false);
pub unsafe fn start(raw:*const c_void,api:Api)->io::Result<()> {
    if raw.is_null() || unsafe {(*(raw as *const Tasks)).version}<2 {return Err(io::Error::other("SKSE UI tasks interface 2 required"));}
    let add=unsafe {(*(raw as *const Tasks)).add_ui};
    API.set(api).map_err(|_|io::Error::other("menu input already started"))?;
    std::thread::Builder::new().name("passthrough-menu".into()).spawn(move|| {
        while !STOP.load(Ordering::Acquire) {
            if !PENDING.swap(true,Ordering::AcqRel) {
                let task=Box::into_raw(Box::new(Delegate{vtable:&VTABLE}));
                unsafe {add(task.cast());}
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    })?;
    write_log(Role::Skyrim,"event=SCENE_MENU_READY scheduler=skse_ui interval_ms=20")
}
unsafe extern "C" fn dispose(task:*mut Delegate) {unsafe {drop(Box::from_raw(task));} PENDING.store(false,Ordering::Release);}
unsafe extern "C" fn run(_: *mut Delegate) {
    let result=std::panic::catch_unwind(|| MEMORY.with(|memory| -> io::Result<()> {
        let mut state=match STATE.try_lock() {
            Ok(state)=>state,
            Err(TryLockError::WouldBlock)=>return Ok(()),
            Err(TryLockError::Poisoned(_))=>return Err(io::Error::other("menu state poisoned")),
        };
        let mut memory=memory.borrow_mut();
        if memory.is_none() { *memory=Some(SharedMemory::open(&std::env::var("PASSTHROUGH_SESSION").unwrap_or_else(|_|"game".into()))?); }
        unsafe {state.tick(API.get().unwrap(),memory.as_ref().unwrap())}
    }));
    let why=match result {Ok(Ok(()))=>return,Ok(Err(e)) if e.kind()==io::ErrorKind::TimedOut=>return,
        Ok(Err(e))=>e.to_string(),Err(_)=>"panic in menu task".into()};
    STOP.store(true,Ordering::Release);
    let _=write_log(Role::Skyrim,format!("event=SCENE_MENU_DISABLED reason={why}"));
}
/// Pointer motion from Minecraft still owed to Skyrim's cursor, in screen pixels.
/// `gain` is the pointer pixels one mouse count produced, learned from read-back.
struct Pointer {key:(u64,u64,u64),seen:[f32;2],owed:[f32;2],gain:[f32;2],moves:u64,logged:u64}
impl Pointer {const NONE:Self=Self{key:(0,0,0),seen:[0.0;2],owed:[0.0;2],gain:[1.0;2],moves:0,logged:0};}
impl Default for Pointer {fn default()->Self {Self::NONE}}
#[derive(Default)]
struct State {fingerprint:u64,context:u64,last:Option<(u64,u64,u64)>,pointer:Pointer,saved:u64,learned:Option<[f32;2]>}
static STATE:Mutex<State>=Mutex::new(State{fingerprint:0,context:0,last:None,pointer:Pointer::NONE,saved:0,learned:None});
thread_local! {static MEMORY:RefCell<Option<SharedMemory>>=const {RefCell::new(None)}; }
impl State {
    unsafe fn fingerprint(api:&Api,s:Sample,sky:u64)->u64 {
        if !crate::skse::menu_ready() {return 0;}
        let ui=unsafe {*(api.ui as *const Object)};
        if ui.is_null() || !unsafe {crate::interaction::menu_blocks(ui)} {return 0;}
        let player=unsafe {*(api.player as *const Object)};
        if player.is_null() {return 0;}
        let cell:Object=unsafe {read(player,0x60)};
        if cell.is_null() {return 0;}
        let world:Object=unsafe {read(cell,0x128)};
        let cell_id=unsafe {read(cell,0x14)};
        let world_id=if world.is_null() {0} else {unsafe {read(world,0x14)}};
        if !crate::save_identity::menu_authorized(s,cell_id,world_id) {return 0;}
        let count:u32=unsafe {read(ui,0x120)};
        let stack:*const Object=unsafe {read(ui,0x110)};
        if count==0 || count>128 || stack.is_null() {return 0;}
        let mut hash=0xcbf29ce484222325_u64;
        let mut mix=|n:u64| {hash=(hash^n).wrapping_mul(0x100000001b3);};
        for n in [sky,s.sender,s.world,crate::save_identity::revision(),count as u64] {mix(n);}
        for i in 0..count as usize {let menu=unsafe {*stack.add(i)};mix(menu as u64);}
        hash.max(1)
    }
    fn change(&mut self,fingerprint:u64)->io::Result<()> {
        if self.fingerprint!=fingerprint {
            self.fingerprint=fingerprint;self.context=self.context.wrapping_add(1).max(1);
            write_log(Role::Skyrim,format!("event=SCENE_MENU_STATE active={} context={} thread={:?}",fingerprint!=0,self.context,std::thread::current().id()))?;
        }
        Ok(())
    }
    unsafe fn tick(&mut self,api:&Api,memory:&SharedMemory)->io::Result<()> {
        let now=now_ms();
        let (sample,sky,valid)=memory.update(|r| {
            let own=r.slots[0];let peer=r.slots[1];let s=r.position;
            let owner=own.pid==std::process::id() && own.protocol==PROTOCOL_VERSION && own.session!=0;
            if owner {r.slots[0].heartbeat_ms=now;r.slots[0].heartbeat_count=r.slots[0].heartbeat_count.wrapping_add(1);}
            let valid=owner && peer.pid!=0 && peer.protocol==PROTOCOL_VERSION
                && peer.acknowledged_session==own.session && own.acknowledged_session==peer.session
                && peer.heartbeat_ms<=now && now-peer.heartbeat_ms<=2000
                && s.sender==peer.session && s.active==1 && s.valid() && s.sent_ms<=now && now-s.sent_ms<=250;
            Ok((s,own.session,valid))
        })?;
        let fingerprint=if valid {unsafe {Self::fingerprint(api,sample,sky)}} else {0};
        self.change(fingerprint)?;
        let key=(sample.sender,sample.world,sample.menu_seq);
        if sample.menu_seq!=0 && self.last!=Some(key) {
            self.last=Some(key);
            let age=now.saturating_sub(sample.menu_ms);
            let stale=sample.menu_ms==0 || sample.menu_ms>now || age>250;
            let cooling=now.saturating_sub(self.saved)<1000;
            let mut reason=verdict(sample.menu_action,valid,fingerprint!=0,sample.menu_context,self.context,stale,cooling);
            // Gameplay commands passed the pure rules; the live game state decides the rest.
            if reason=="gameplay" {reason=unsafe {Self::gameplay(api,sample)};}
            if reason=="dispatch" && GAMEPLAY_COMMANDS.contains(&sample.menu_action) {self.saved=now;}
            write_log(Role::Skyrim,format!("event=SCENE_MENU_RECV sequence={} action={} context={} state={} age_ms={} thread={:?}",sample.menu_seq,sample.menu_action,sample.menu_context,reason,age,std::thread::current().id()))?;
            if reason=="dispatch" {
                let dispatched=unsafe {dispatch(api,sample.menu_action)};
                write_log(Role::Skyrim,format!("event=SCENE_MENU_RESULT sequence={} state={} outcome=game_owned thread={:?}",sample.menu_seq,if dispatched {"returned"} else {"busy_or_missing_controls"},std::thread::current().id()))?;
                self.change(unsafe {Self::fingerprint(api,sample,sky)})?;
            }
        }
        unsafe {self.point(api,sample,now)}?;
        let status=Status{active:self.fingerprint!=0,context:self.context,ms:now_ms(),sky,mc:sample.sender,world:sample.world};
        memory.update(|r| {
            if r.slots[0].pid==std::process::id() && r.slots[0].session==sky {r.menu=status;}
            Ok(())
        })
    }
    /// Quicksave/quickload need a loaded game, the paired save and no Skyrim menu in the way.
    unsafe fn gameplay(api:&Api,s:Sample)->&'static str {
        if !crate::skse::menu_ready() {return "loading";}
        let ui=unsafe {*(api.ui as *const Object)};
        if ui.is_null() || unsafe {crate::interaction::menu_blocks(ui)} {return "menu_open";}
        let loaded=crate::save_identity::published_id();
        if loaded==[0;16] || loaded!=s.save_id {return "unpaired_save";}
        "dispatch"
    }
    /// Turn Minecraft's running mouse total into ordinary Skyrim mouse-move events.
    unsafe fn point(&mut self,api:&Api,s:Sample,now:u64)->io::Result<()> {
        if self.fingerprint==0 || s.cursor_context==0 || s.cursor_context!=self.context {self.pointer.key=(0,0,0);return Ok(());}
        let Some((slot,vtable))=api.cursor else {return Ok(());};
        let cursor=unsafe {*(slot as *const Object)};
        if cursor.is_null() {return Ok(());}
        // CommonLib MenuCursor.h: position 0x4/0x8, screen size 0x14/0x18, sensitivity 0x1C.
        let place=|| unsafe {[read::<f32>(cursor,0x4),read::<f32>(cursor,0x8)]};
        let size:[f32;2]=unsafe {[read(cursor,0x14),read(cursor,0x18)]};
        let before=place();
        let sane=before.iter().all(|v|v.is_finite()) && size.iter().all(|v|(64.0..=16384.0).contains(v));
        let key=(s.sender,s.world,s.cursor_context);
        let learned=self.learned;
        let p=&mut self.pointer;
        if p.key!=key {
            // The first total for a menu is only a baseline: the pointer must not jump.
            let speed:f32=unsafe {read(cursor,0x1C)};
            let gain=starting_gain(learned,speed);
            *p=Pointer{key,seen:s.cursor,gain,..Pointer::NONE};
            return write_log(Role::Skyrim,format!("event=SCENE_CURSOR state=baseline context={} x={} y={} screen_w={} screen_h={} speed={} gain_x={} gain_y={} remembered={} sane={} thread={:?}",s.cursor_context,before[0],before[1],size[0],size[1],speed,gain[0],gain[1],learned.is_some(),sane,std::thread::current().id()));
        }
        let step=[0,1].map(|i| s.cursor[i]-p.seen[i]);
        p.seen=s.cursor;
        if !sane {return Ok(());}
        for i in 0..2 {p.owed[i]=(p.owed[i]+step[i]*size[i]).clamp(-size[i],size[i]);}
        let request=[0,1].map(|i| (p.owed[i]/p.gain[i]).round().clamp(-4096.0,4096.0) as i32);
        if request==[0,0] || !unsafe {glide(api,vtable,request)} {return Ok(());}
        let after=place();
        let used=p.gain;
        for i in 0..2 {
            // Whatever Skyrim did not apply is dropped, as with a real mouse at a screen edge.
            p.owed[i]-=request[i] as f32*p.gain[i];
            p.gain[i]=measured_gain(p.gain[i],request[i],before[i],after[i],size[i]);
        }
        // Keep a measured speed for the rest of the session, so the next menu starts with it.
        let remember=(p.gain!=used).then_some(p.gain);
        p.moves+=1;
        if p.moves<=5 || now.saturating_sub(p.logged)>=250 {
            p.logged=now;
            write_log(Role::Skyrim,format!("event=SCENE_CURSOR state=move context={} moves={} request_x={} request_y={} before_x={} before_y={} after_x={} after_y={} used_gain_x={} used_gain_y={} gain_x={} gain_y={} thread={:?}",s.cursor_context,p.moves,request[0],request[1],before[0],before[1],after[0],after[1],used[0],used[1],p.gain[0],p.gain[1],std::thread::current().id()))?;
        }
        if remember.is_some() {self.learned=remember;}
        Ok(())
    }
}
/// Pointer pixels per mouse count at the start of a menu: the value measured earlier this
/// session if there is one, else Skyrim's stored sensitivity when plausible, else 1.
fn starting_gain(learned:Option<[f32;2]>,speed:f32)->[f32;2] {
    learned.unwrap_or([if speed.is_finite() && (0.05..=20.0).contains(&speed) {speed} else {1.0};2])
}
/// The pixels-per-count this move showed, or `old` when the move cannot be trusted: too
/// small to measure, ended near a screen edge (where Skyrim clips it), or implausible.
fn measured_gain(old:f32,request:i32,before:f32,after:f32,size:f32)->f32 {
    let ratio=(after-before)/request as f32;
    let inside=after>size*0.05 && after<size*0.95;
    if request.abs()>=8 && inside && ratio.is_finite() && (0.05..=20.0).contains(&ratio.abs()) {ratio} else {old}
}
/// Commands that need an open Skyrim menu and its current context number.
pub const MENU_COMMANDS:[u32;8]=[1,2,3,4,5,6,9,10];
/// Commands for normal play: context 0 and no Skyrim menu open.
pub const GAMEPLAY_COMMANDS:[u32;2]=[7,8];
/// Skyrim user event (NUL-terminated), input device (0 keyboard, 1 mouse) and key code for
/// each command number. A number missing here is rejected before anything is sent.
fn event(action:u32)->Option<(&'static [u8],u32,u32)> {
    Some(match action {
        1=>(b"Up\0",0,0xC8),2=>(b"Down\0",0,0xD0),3=>(b"Left\0",0,0xCB),4=>(b"Right\0",0,0xCD),
        5=>(b"Accept\0",0,0x1C),6=>(b"Cancel\0",0,0x0F),
        // Skyrim's own quicksave/quickload user events with their default F5/F9 scan codes.
        7=>(b"Quicksave\0",0,0x3F),8=>(b"Quickload\0",0,0x43),
        // Skyrim's R key in item menus (take all, drop), and a true left mouse click.
        9=>(b"XButton\0",0,0x13),10=>(b"Click\0",1,0),
        _=>return None,
    })
}
/// The rules that need no game state. "gameplay" means the live checks still have to pass.
fn verdict(action:u32,valid:bool,menu_open:bool,context:u64,current:u64,stale:bool,cooling:bool)->&'static str {
    if GAMEPLAY_COMMANDS.contains(&action) {
        if !valid {"inactive"} else if context!=0 {"context_changed"} else if stale {"expired"}
        else if cooling {"cooldown"} else {"gameplay"}
    } else if !menu_open {"inactive"} else if context!=current {"context_changed"} else if stale {"expired"}
    else if !MENU_COMMANDS.contains(&action) {"bad_action"} else {"dispatch"}
}
/// CommonLib MouseMoveEvent: an IDEvent followed by two signed mouse counts.
#[repr(C)]
struct Motion {vtable:usize,device:u32,kind:u32,next:usize,user:usize,id:u32,pad:u32,x:i32,y:i32}
/// One mouse-move event through MenuControls, so Skyrim's own cursor handler moves the pointer.
unsafe fn glide(api:&Api,vtable:usize,by:[i32;2])->bool {
    let controls=unsafe {*(api.controls as *const Object)};
    let source=unsafe {*(api.input as *const Object)};
    if controls.is_null() || source.is_null() || unsafe {read::<u8>(controls,0x80)!=0 || read::<u8>(controls,0x82)!=0} {return false;}
    let ctor:unsafe extern "C" fn(*mut usize,*const u8)->*mut usize=unsafe {mem::transmute(api.string_ctor)};
    let release:unsafe extern "C" fn(*mut usize)=unsafe {mem::transmute(api.string_release)};
    // Device 1 is the mouse, event type 1 a mouse move; 0xA is the mouse-move code in Skyrim's control map.
    let mut motion=Motion{vtable,device:1,kind:1,next:0,user:0,id:0xA,pad:0,x:by[0],y:by[1]};
    unsafe {ctor(&mut motion.user,b"Cursor\0".as_ptr());}
    let table:*const usize=unsafe {read(controls,0)};
    let process:unsafe extern "C" fn(Object,*const *mut Motion,Object)->u32=unsafe {mem::transmute(*table.add(1))};
    let event=&mut motion as *mut Motion;
    unsafe {process(controls,&event,source);release(&mut motion.user);}
    true
}
#[repr(C)]
struct Button {vtable:usize,device:u32,kind:u32,next:*mut Button,user:usize,id:u32,pad:u32,value:f32,held:f32}
unsafe fn dispatch(api:&Api,action:u32)->bool {
    let controls=unsafe {*(api.controls as *const Object)};
    let source=unsafe {*(api.input as *const Object)};
    if controls.is_null() || source.is_null() || unsafe {read::<u8>(controls,0x80)!=0 || read::<u8>(controls,0x82)!=0} {return false;}
    let Some((name,device,id))=event(action) else {return false;};
    let ctor:unsafe extern "C" fn(*mut usize,*const u8)->*mut usize=unsafe {mem::transmute(api.string_ctor)};
    let release:unsafe extern "C" fn(*mut usize)=unsafe {mem::transmute(api.string_release)};
    let mut button=Button{vtable:api.button,device,kind:0,next:ptr::null_mut(),user:0,id,pad:0,value:1.0,held:0.0};
    unsafe {ctor(&mut button.user,name.as_ptr());}
    let vtable:*const usize=unsafe {read(controls,0)};
    let process:unsafe extern "C" fn(Object,*const *mut Button,Object)->u32=unsafe {mem::transmute(*vtable.add(1))};
    let event=&mut button as *mut Button;
    unsafe {process(controls,&event,source);}
    button.value=0.0;button.held=0.05;
    unsafe {process(controls,&event,source);release(&mut button.user);}
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    /// A command number added in one place but not the other would be silently dropped
    /// (or sent without a rule), so the two lists and the event table must agree.
    #[test]
    fn every_command_number_has_exactly_one_rule_and_one_event() {
        for action in 0..=64 {
            let listed=MENU_COMMANDS.contains(&action) || GAMEPLAY_COMMANDS.contains(&action);
            assert_eq!(event(action).is_some(),listed,"command {action}");
            assert!(!(MENU_COMMANDS.contains(&action) && GAMEPLAY_COMMANDS.contains(&action)),"command {action}");
        }
        for action in MENU_COMMANDS.iter().chain(&GAMEPLAY_COMMANDS) {
            let (name,device,_)=event(*action).unwrap();
            // Skyrim's string constructor reads up to the terminator.
            assert_eq!(name.iter().filter(|byte|**byte==0).count(),1);
            assert_eq!(name.last(),Some(&0));
            assert!(device<=1);
        }
        // Only the true click is a mouse event; everything else stays on the proven key path.
        assert_eq!(MENU_COMMANDS.iter().chain(&GAMEPLAY_COMMANDS).filter(|a|event(**a).unwrap().1==1).count(),1);
        assert_eq!(event(10).unwrap().1,1);
    }
    #[test]
    fn menu_commands_need_the_open_menu_they_were_aimed_at() {
        for action in MENU_COMMANDS {
            assert_eq!(verdict(action,true,true,4,4,false,false),"dispatch");
            assert_eq!(verdict(action,true,false,4,4,false,false),"inactive");
            assert_eq!(verdict(action,true,true,3,4,false,false),"context_changed");
            assert_eq!(verdict(action,true,true,4,4,true,false),"expired");
            // The save cooldown never delays menu navigation.
            assert_eq!(verdict(action,true,true,4,4,false,true),"dispatch");
        }
        assert_eq!(verdict(0,true,true,4,4,false,false),"bad_action");
        assert_eq!(verdict(11,true,true,4,4,false,false),"bad_action");
    }
    /// Numbers from report 20261011-001910: Skyrim moved the pointer 2 pixels per count.
    #[test]
    fn pointer_speed_is_measured_from_trustworthy_moves_only() {
        assert_eq!(measured_gain(1.0,21,2156.0,2198.0,2560.0),2.0);
        assert_eq!(measured_gain(1.0,-39,1578.0,1500.0,2560.0),2.0);
        // Too few counts to measure, a clipped move at the edge, and no movement at all.
        assert_eq!(measured_gain(1.0,6,2132.0,2144.0,2560.0),1.0);
        assert_eq!(measured_gain(1.0,30,2470.0,2500.0,2560.0),1.0);
        assert_eq!(measured_gain(1.5,30,900.0,900.0,1440.0),1.5);
        // A reversed axis is learned with its sign, so requests are then sent reversed.
        assert_eq!(measured_gain(1.0,10,500.0,480.0,1440.0),-2.0);
    }
    #[test]
    fn a_new_menu_starts_with_the_speed_measured_earlier() {
        assert_eq!(starting_gain(None,1.0),[1.0;2]);
        assert_eq!(starting_gain(None,f32::NAN),[1.0;2]);
        assert_eq!(starting_gain(None,500.0),[1.0;2]);
        assert_eq!(starting_gain(Some([2.0,2.0]),1.0),[2.0,2.0]);
    }
    #[test]
    fn quicksave_and_quickload_need_normal_play() {
        for action in GAMEPLAY_COMMANDS {
            assert_eq!(verdict(action,true,false,0,9,false,false),"gameplay");
            assert_eq!(verdict(action,false,false,0,9,false,false),"inactive");
            assert_eq!(verdict(action,true,false,9,9,false,false),"context_changed");
            assert_eq!(verdict(action,true,false,0,9,true,false),"expired");
            assert_eq!(verdict(action,true,false,0,9,false,true),"cooldown");
        }
    }
}
