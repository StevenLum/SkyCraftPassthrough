//! Session-local incoming damage queue. Game callbacks can arrive on different threads.
use std::{collections::VecDeque,io,mem,ptr,sync::{Mutex,atomic::{AtomicUsize,AtomicU64,Ordering}}};
use crate::{native::write_log,position::Sample,Role};
type Object=*mut u8;
type Clamp=unsafe extern "C" fn(Object,u32,f32)->f32;
static ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static MODIFIER:AtomicUsize=AtomicUsize::new(0);
static NEXT:AtomicU64=AtomicU64::new(1);
static INSTALL:Mutex<()>=Mutex::new(());
#[derive(Clone,Copy,PartialEq)]
struct Key {sender:u64,world:u64,revision:u64,save:[u8;16],mc:[u8;16]}
struct State {key:Option<Key>,epoch:u64,sequence:u64,player:usize,until:u64,maximum:f32,queue:VecDeque<(u64,f32,f32,u64)>}
static STATE:Mutex<State>=Mutex::new(State{key:None,epoch:0,sequence:0,player:0,until:0,maximum:0.0,queue:VecDeque::new()});
#[link(name="kernel32")]
unsafe extern "system" {fn VirtualProtect(p:*mut std::ffi::c_void,n:usize,protect:u32,old:*mut u32)->i32;}
pub fn configure(modifier:usize) {MODIFIER.store(modifier,Ordering::Release);}
pub fn suspend() {if let Ok(mut s)=STATE.lock() {s.until=0;}}
static LETHAL:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
/// While set, the bridge's own lethal hit on the player must reach Skyrim unchanged.
pub fn lethal(on:bool) {LETHAL.store(on,Ordering::Release);}
pub fn reset() {
    if let Ok(mut s)=STATE.lock() {
        if !s.queue.is_empty() {let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_DROP epoch={} count={} reason=reset",s.epoch,s.queue.len()));}
        s.key=None;s.until=0;s.queue.clear();
    }
}
pub unsafe fn arm(player:Object,sample:Sample)->io::Result<()> {
    let modifier=MODIFIER.load(Ordering::Acquire);
    if modifier==0 {return Ok(());}
    let owner=unsafe {player.add(0xB8)};
    let table=unsafe {ptr::read_unaligned(owner.cast::<*mut usize>())};
    let installation=INSTALL.lock().map_err(|_|io::Error::other("damage install lock poisoned"))?;
    if ORIGINAL.load(Ordering::Acquire)==0 {
        let primary=unsafe {ptr::read_unaligned(player.cast::<*mut usize>())};
        let slot=unsafe {primary.add(0x127)};let mut protection=0;
        if unsafe {VirtualProtect(slot.cast(),8,0x04,&mut protection)}==0 {return Err(io::Error::last_os_error());}
        ORIGINAL.store(unsafe {*slot},Ordering::Release);
        unsafe {ptr::write(slot,intercept as *const () as usize);}
        let mut ignored=0;let restored=unsafe {VirtualProtect(slot.cast(),8,protection,&mut ignored)};
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_CLAMP_HOOK slot=295 protection_restored={}",restored!=0));
    }
    drop(installation);
    let permanent:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(2))};
    let temporary:unsafe extern "C" fn(Object,u32,u32)->f32=unsafe {mem::transmute(modifier)};
    let maximum=unsafe {permanent(owner,24)+temporary(player,1,24)};
    let key=Key{sender:sample.sender,world:sample.world,revision:crate::save_identity::revision(),save:sample.save_id,mc:sample.mc_id};
    let mut s=STATE.lock().map_err(|_|io::Error::other("damage state poisoned"))?;
    if s.key!=Some(key) {
        if !s.queue.is_empty() {let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_DROP epoch={} count={} reason=generation_changed",s.epoch,s.queue.len()));}
        s.key=Some(key);s.epoch=NEXT.fetch_add(1,Ordering::Relaxed);s.sequence=0;s.queue.clear();
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_GENERATION epoch={} world={} revision={}",s.epoch,sample.world,key.revision));
    }
    if sample.damage_epoch==s.epoch {
        while s.queue.front().is_some_and(|e|e.0<=sample.damage_ack) {
            let e=s.queue.pop_front().unwrap();
            let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_ACK epoch={} sequence={}",s.epoch,e.0));
        }
    }
    let now=crate::windows::now_ms();
    while s.queue.front().is_some_and(|e|now.saturating_sub(e.3)>5000) {
        let e=s.queue.pop_front().unwrap();
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_DROP epoch={} sequence={} reason=expired",s.epoch,e.0));
    }
    s.player=player as usize;s.maximum=maximum;
    let ready=sample.damage_epoch==s.epoch && sample.damage_ready==1 && maximum.is_finite() && maximum>0.0;
    if (s.until>now)!=ready {
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_READY epoch={} ready={} skyrim_max={maximum}",s.epoch,ready));
    }
    s.until=if ready {now+250} else {0};
    Ok(())
}
pub fn snapshot(snapshot:&mut crate::world::Snapshot) {
    if let Ok(s)=STATE.lock() {
        if s.key.is_some_and(|k|k.sender==snapshot.recipient && k.world==snapshot.world && k.revision==crate::save_identity::revision()) {
            snapshot.damage_epoch=s.epoch;
            snapshot.damage_events=s.queue.iter().map(|e|(e.0,e.1,e.2)).collect();
        }
    }
}
unsafe extern "C" fn intercept(player:Object,value:u32,requested:f32)->f32 {
    let original=ORIGINAL.load(Ordering::Acquire);if original==0 {return requested;}
    let call:Clamp=unsafe {mem::transmute(original)};
    let amount=unsafe {call(player,value,requested)};
    let captured=std::panic::catch_unwind(|| {
        if value!=24 {return false;}
        if requested!=0.0 || amount!=0.0 {
            let owner=unsafe {player.add(0xB8)};
            let table=unsafe {ptr::read_unaligned(owner.cast::<*const usize>())};
            let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
            let health=unsafe {get(owner,24)};
            let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_CLAMP requested={requested} returned={amount} health={health}"));
        }
        if !amount.is_finite() || amount>=0.0 {return false;}
        if LETHAL.load(Ordering::Acquire) {
            let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_PASSTHROUGH amount={amount} reason=bridge_death"));
            return false;
        }
        let Ok(mut s)=STATE.lock() else {return false;};
        let now=crate::windows::now_ms();
        let ready=s.until>now && s.player==player as usize
            && crate::skse::menu_ready() && s.key.is_some_and(|k|k.revision==crate::save_identity::revision());
        if !ready || s.queue.len()>=32 {
            let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_PASSTHROUGH amount={} reason={}",amount,if ready {"queue_full"} else {"not_ready"}));
            return false;
        }
        let fraction=-amount/s.maximum;
        if !fraction.is_finite() || fraction<=0.0 {return false;}
        s.sequence+=1;let seq=s.sequence;
        s.queue.push_back((seq,fraction,-amount,now));
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_CAPTURE epoch={} sequence={} skyrim_damage={} skyrim_max={} fraction={} suppressed=true",s.epoch,seq,-amount,s.maximum,fraction));
        true
    }).unwrap_or(false);
    if captured {0.0} else {amount}
}
