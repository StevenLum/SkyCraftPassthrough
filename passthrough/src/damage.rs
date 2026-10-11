//! Session-local incoming damage queue. Game callbacks can arrive on different threads.
use std::{collections::VecDeque,io,mem,ptr,sync::{Mutex,atomic::{AtomicUsize,AtomicU64,Ordering}}};
use crate::{native::write_log,position::Sample,Role};
type Object=*mut u8;
type Clamp=unsafe extern "C" fn(Object,u32,f32)->f32;
static ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static RESTORE_ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static MODIFIER:AtomicUsize=AtomicUsize::new(0);
static NEXT:AtomicU64=AtomicU64::new(1);
static INSTALL:Mutex<()>=Mutex::new(());
#[derive(Clone,Copy,PartialEq)]
struct Key {sender:u64,world:u64,revision:u64,save:[u8;16],mc:[u8;16]}
/// `shield` is the time until which a paused Minecraft protects the player; `shielded`
/// counts the hits cancelled since it came on.
/// Queue entries are (sequence, share of full health, Skyrim amount, time). A negative
/// Skyrim amount means health restored instead of taken.
struct State {key:Option<Key>,epoch:u64,sequence:u64,player:usize,until:u64,restore_until:u64,maximum:f32,queue:VecDeque<(u64,f32,f32,u64)>,shield:u64,shielded:u64,restore:Restore}
static STATE:Mutex<State>=Mutex::new(State{key:None,epoch:0,sequence:0,player:0,until:0,restore_until:0,maximum:0.0,queue:VecDeque::new(),shield:0,shielded:0,restore:Restore{start:0,sum:0.0}});
/// Small positive health changes, added up from `start` until they can be judged.
#[derive(Clone,Copy)]
struct Restore {start:u64,sum:f32}
/// A single restore of at least this share of full health is forwarded at once (potions, food).
const RESTORE_AT_ONCE:f32=0.005;
/// Smaller ones are added up for this long ...
const RESTORE_WINDOW_MS:u64=250;
/// ... and forwarded only when they come faster than this share of full health per second.
/// Skyrim's own natural recovery (about 0.007 per second) stays below it and is left out,
/// because Minecraft has its own; healing spells are well above it.
const RESTORE_RATE:f32=0.025;
/// How much restored health to forward to Minecraft for one positive change, if any.
fn restore(bucket:&mut Restore,requested:f32,maximum:f32,now:u64)->Option<f32> {
    if !(requested.is_finite() && requested>0.0 && maximum.is_finite() && maximum>0.0) {return None;}
    if requested/maximum>=RESTORE_AT_ONCE {return Some(requested);}
    let age=now.saturating_sub(bucket.start);
    // An empty or abandoned bucket starts again with this change.
    if bucket.sum<=0.0 || age>4*RESTORE_WINDOW_MS {*bucket=Restore{start:now,sum:requested};return None;}
    bucket.sum+=requested;
    if age<RESTORE_WINDOW_MS {return None;}
    let (sum,rate)=(bucket.sum,bucket.sum/maximum*1000.0/age as f32);
    *bucket=Restore{start:now,sum:0.0};
    (rate>=RESTORE_RATE).then_some(sum)
}
/// Skyrim restored some of the player's health. Queue it for Minecraft when Minecraft is
/// taking hits and it is more than natural recovery. Skyrim's own result is never changed.
fn forward_restore(player:Object,requested:f32) {
    if !crate::switches::on(Role::Skyrim,crate::switches::HEALING) {return;}
    let Ok(mut s)=STATE.lock() else {return;};
    let now=crate::windows::now_ms();
    let ready=s.restore_until>now && s.player==player as usize
        && crate::skse::menu_ready() && s.key.is_some_and(|k|k.revision==crate::save_identity::revision());
    if !ready {return;}
    let maximum=s.maximum;
    let Some(amount)=restore(&mut s.restore,requested,maximum,now) else {return;};
    if s.queue.len()>=32 {
        let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_DROP epoch={} skyrim_heal={amount} reason=queue_full",s.epoch));
        return;
    }
    let fraction=(amount/maximum).min(1.0);
    s.sequence+=1;let seq=s.sequence;
    s.queue.push_back((seq,fraction,-amount,now));
    let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_CAPTURE epoch={} sequence={seq} skyrim_heal={amount} skyrim_max={maximum} fraction={fraction}",s.epoch));
}
fn same_restore_pair(key:Option<Key>,sample:Sample,revision:u64)->bool {
    key==Some(Key{sender:sample.sender,world:sample.world,revision,save:sample.save_id,mc:sample.mc_id})
}
/// Called only after menu_input verifies live peer/sample, save/place and open menu.
pub fn menu_lease(player:Object,sample:Sample) {
    let Ok(mut s)=STATE.lock() else {return;};
    if s.player!=player as usize || !same_restore_pair(s.key,sample,crate::save_identity::revision()) {return;}
    let now=crate::windows::now_ms();s.restore_until=now+250;
    // A potion used in a menu still counts when the user closes that menu later.
    // Damage expiry is unchanged; disconnected menus cannot refresh this lease.
    for e in &mut s.queue {if e.2<0.0 {e.3=now;}}
}
fn restoration(modifier:u32,value:u32,requested:f32)->bool {
    modifier==2 && value==24 && requested.is_finite() && requested>0.0
}
/// ActorValueOwner::ModActorValue, slot 6; this pointer is Actor+0xB8.
unsafe extern "C" fn intercept_restore(owner:Object,modifier:u32,value:u32,requested:f32) {
    let original=RESTORE_ORIGINAL.load(Ordering::Acquire);if original==0 {return;}
    let call:unsafe extern "C" fn(Object,u32,u32,f32)=unsafe {mem::transmute(original)};
    let _=std::panic::catch_unwind(|| unsafe {dispatch_restore(owner,modifier,value,requested,call,|player,amount| {
        let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_REQUEST requested={amount} modifier={modifier} value={value}"));
        forward_restore(player,amount);
    })});
}
unsafe fn dispatch_restore(owner:Object,modifier:u32,value:u32,requested:f32,
    call:unsafe extern "C" fn(Object,u32,u32,f32),capture:impl FnOnce(Object,f32)) {
    unsafe {call(owner,modifier,value,requested);}
    if restoration(modifier,value,requested) {capture(unsafe {owner.sub(0xB8)},requested);}
}
/// How long one "not active" Minecraft frame keeps the protection on.
const SHIELD_MS:u64=500;
/// What happens to one Skyrim hit on the player.
#[derive(Clone,Copy,Debug,PartialEq)]
enum Route {
    /// Cancelled in Skyrim and queued for Minecraft, which owns the health.
    Forward,
    /// Cancelled and not queued: Minecraft is paused, as its own pause protects its player.
    Shield,
    /// Left to Skyrim, with the reason for the log.
    Native(&'static str),
}
/// `ready`: Minecraft is taking hits now. `paused`: the Minecraft that was steering is alive
/// but not playing. `queued`: hits still waiting for Minecraft.
fn route(ready:bool,paused:bool,queued:usize)->Route {
    if ready {if queued>=32 {Route::Native("queue_full")} else {Route::Forward}}
    else if paused {Route::Shield}
    else {Route::Native("not_ready")}
}
/// Whether a "not active" Minecraft frame comes from the session, world and loaded save the
/// damage state belongs to. Anything else must not inherit the protection.
fn same_minecraft(key:Option<Key>,sender:u64,world:u64,revision:u64)->bool {
    key.is_some_and(|k|k.sender==sender && k.world==world && k.revision==revision)
}
fn shield_off(s:&mut State,reason:&str) {
    if s.shield!=0 {
        let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_SHIELD state=off epoch={} reason={reason} cancelled={}",s.epoch,s.shielded));
    }
    s.shield=0;s.shielded=0;
}
/// Minecraft is alive but paused or not in play. If it is the one that was steering, keep its
/// waiting hits and protect the player a moment longer; otherwise forget the damage state.
pub fn pause(sender:u64,world:u64) {
    // Switched off in switches.txt: behave as before 0.10.22.
    if !crate::switches::on(Role::Skyrim,crate::switches::PAUSE_SHIELD) {return reset();}
    let matches=STATE.lock().is_ok_and(|mut s| {
        if !same_minecraft(s.key,sender,world,crate::save_identity::revision()) {return false;}
        let now=crate::windows::now_ms();
        if s.shield<=now {
            let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_SHIELD state=on epoch={} reason=minecraft_not_active waiting={}",s.epoch,s.queue.len()));
            s.shielded=0;
        }
        s.shield=now+SHIELD_MS;s.until=0;s.restore_until=0;
        true
    });
    if !matches {reset();}
}
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
        shield_off(&mut s,"reset");
        s.key=None;s.until=0;s.restore_until=0;s.restore=Restore{start:0,sum:0.0};s.queue.clear();
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
    if RESTORE_ORIGINAL.load(Ordering::Acquire)==0 {
        let slot=unsafe {table.add(6)};let mut protection=0;
        if unsafe {VirtualProtect(slot.cast(),8,0x04,&mut protection)}==0 {return Err(io::Error::last_os_error());}
        RESTORE_ORIGINAL.store(unsafe {*slot},Ordering::Release);
        unsafe {ptr::write(slot,intercept_restore as *const () as usize);}
        let mut ignored=0;let restored=unsafe {VirtualProtect(slot.cast(),8,protection,&mut ignored)};
        let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_HOOK owner_offset=184 slot=6 protection_restored={}",restored!=0));
    }
    drop(installation);
    let permanent:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(2))};
    let temporary:unsafe extern "C" fn(Object,u32,u32)->f32=unsafe {mem::transmute(modifier)};
    let maximum=unsafe {permanent(owner,24)+temporary(player,1,24)};
    let key=Key{sender:sample.sender,world:sample.world,revision:crate::save_identity::revision(),save:sample.save_id,mc:sample.mc_id};
    let mut s=STATE.lock().map_err(|_|io::Error::other("damage state poisoned"))?;
    // Minecraft is playing again: its own health takes the hits from here.
    shield_off(&mut s,"resumed");
    if s.key!=Some(key) {
        if !s.queue.is_empty() {let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_DROP epoch={} count={} reason=generation_changed",s.epoch,s.queue.len()));}
        s.key=Some(key);s.epoch=NEXT.fetch_add(1,Ordering::Relaxed);s.sequence=0;s.queue.clear();s.restore=Restore{start:0,sum:0.0};
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
    s.player=player as usize;s.maximum=maximum;s.restore_until=now+250;
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
        // Positive restore requests are captured at ActorValueOwner slot 6 before
        // full-health clamping. Never capture again from a nested clamp call.
        if requested.is_finite() && requested>0.0 {return false;}
        if !amount.is_finite() || amount>=0.0 {return false;}
        if LETHAL.load(Ordering::Acquire) {
            let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_PASSTHROUGH amount={amount} reason=bridge_death"));
            return false;
        }
        let Ok(mut s)=STATE.lock() else {return false;};
        let now=crate::windows::now_ms();
        let same=s.player==player as usize
            && crate::skse::menu_ready() && s.key.is_some_and(|k|k.revision==crate::save_identity::revision());
        match route(s.until>now && same,s.shield>now && same,s.queue.len()) {
            Route::Forward=>{}
            Route::Shield=>{
                s.shielded+=1;
                let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_SHIELD state=cancelled epoch={} skyrim_damage={} count={}",s.epoch,-amount,s.shielded));
                return true;
            }
            Route::Native(reason)=>{
                let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_PASSTHROUGH amount={amount} reason={reason}"));
                return false;
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_restore_callback_keeps_full_health_potion_request_and_calls_original_once() {
        static CALLS:AtomicUsize=AtomicUsize::new(0);
        unsafe extern "C" fn native(owner:Object,modifier:u32,value:u32,amount:f32) {
            assert_eq!((modifier,value),(2,24));CALLS.fetch_add(1,Ordering::SeqCst);
            let health=unsafe {ptr::read_unaligned(owner.cast::<f32>())};
            unsafe {ptr::write_unaligned(owner.cast::<f32>(),(health+amount).min(100.0));}
        }
        let mut bytes=[0_u64;32];let player=bytes.as_mut_ptr().cast::<u8>();let owner=unsafe {player.add(0xB8)};
        unsafe {ptr::write_unaligned(owner.cast::<f32>(),100.0);}
        let mut captured=None;
        unsafe {dispatch_restore(owner,2,24,25.0,native,|p,amount|captured=Some((p,amount)));}
        assert_eq!(CALLS.load(Ordering::SeqCst),1);
        assert_eq!(unsafe {ptr::read_unaligned(owner.cast::<f32>())},100.0);
        assert_eq!(captured,Some((player,25.0)));
    }
    #[test]
    fn restoring_health_is_captured_before_a_full_health_clamp() {
        // Native health is already full because damage is owned by Minecraft;
        // the requested potion amount must still be forwarded.
        assert!(restoration(2,24,25.0));
        let mut bucket=Restore{start:0,sum:0.0};
        assert_eq!(restore(&mut bucket,25.0,100.0,1000),Some(25.0));
        for (modifier,value,amount) in [(0,24,25.0),(1,24,25.0),(2,25,25.0),
            (2,24,-25.0),(2,24,0.0),(2,24,f32::NAN)] {assert!(!restoration(modifier,value,amount));}
    }
    #[test]
    fn a_menu_healing_lease_cannot_follow_another_save_world_or_player() {
        let s=Sample{sender:1,world:2,save_id:[3;16],mc_id:[4;16],..Sample::default()};
        let key=Some(Key{sender:1,world:2,revision:5,save:[3;16],mc:[4;16]});
        assert!(same_restore_pair(key,s,5));
        assert!(!same_restore_pair(key,s,6));assert!(!same_restore_pair(None,s,5));
        for other in [Sample{sender:8,..s},Sample{world:8,..s},Sample{save_id:[8;16],..s},Sample{mc_id:[8;16],..s}] {
            assert!(!same_restore_pair(key,other,5));
        }
    }
    #[test]
    fn a_hit_is_forwarded_shielded_or_left_to_skyrim() {
        // Minecraft playing: forwarded, unless its queue is full.
        assert_eq!(route(true,false,0),Route::Forward);
        assert_eq!(route(true,false,31),Route::Forward);
        assert_eq!(route(true,false,32),Route::Native("queue_full"));
        // Playing wins over a pause that has not lapsed yet.
        assert_eq!(route(true,true,0),Route::Forward);
        // Minecraft paused: cancelled without queueing, however many hits are waiting.
        assert_eq!(route(false,true,0),Route::Shield);
        assert_eq!(route(false,true,32),Route::Shield);
        // Minecraft gone, loading or recovering: Skyrim's own damage applies.
        assert_eq!(route(false,false,0),Route::Native("not_ready"));
    }
    /// Feeds one change per frame for a while and returns everything that was forwarded.
    fn stream(per_frame:f32,frame_ms:u64,frames:u64,maximum:f32)->f32 {
        let mut bucket=Restore{start:0,sum:0.0};
        (0..frames).filter_map(|i|restore(&mut bucket,per_frame,maximum,1_000_000+i*frame_ms)).sum()
    }
    #[test]
    fn potions_and_healing_spells_are_forwarded_but_natural_recovery_is_not() {
        let mut bucket=Restore{start:0,sum:0.0};
        // A potion or a bite of food: at once, whatever is in the bucket.
        assert_eq!(restore(&mut bucket,50.0,100.0,1000),Some(50.0));
        assert_eq!(restore(&mut bucket,1.0,100.0,1001),Some(1.0));
        // Natural recovery as logged in report 20261011-094530: 0.0083 per frame at 60 a second.
        assert_eq!(stream(0.0083,16,600,100.0),0.0);
        // The same, three times as strong (recovery bonuses), and at 20 frames a second.
        assert_eq!(stream(0.025,16,600,100.0),0.0);
        assert_eq!(stream(0.035,50,200,100.0),0.0);
        // A healing spell, 10 a second for two seconds: nearly all of the 20 arrives.
        let healed=stream(10.0/60.0,16,125,100.0);
        assert!((17.0..=20.9).contains(&healed),"healed {healed}");
        // The same spell on a character with 300 health is still well above natural recovery.
        assert!(stream(10.0/60.0,16,125,300.0)>15.0);
        // One small change on its own is never forwarded, and nothing is forwarded for nonsense.
        assert_eq!(stream(0.3,16,1,100.0),0.0);
        for bad in [0.0,-5.0,f32::NAN,f32::INFINITY] {assert_eq!(restore(&mut bucket,bad,100.0,5000),None);}
        assert_eq!(restore(&mut bucket,50.0,0.0,5000),None);
    }
    #[test]
    fn only_the_minecraft_that_was_steering_keeps_the_player_protected() {
        let key=Some(Key{sender:7,world:3,revision:5,save:[1;16],mc:[2;16]});
        assert!(same_minecraft(key,7,3,5));
        // Another Minecraft session, another world (title screen, different save), or a
        // Skyrim save loaded since.
        assert!(!same_minecraft(key,8,3,5));
        assert!(!same_minecraft(key,7,4,5));
        assert!(!same_minecraft(key,7,3,6));
        // Never steered at all.
        assert!(!same_minecraft(None,7,3,5));
    }
}
