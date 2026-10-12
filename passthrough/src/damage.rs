//! Session-local incoming damage queue. Game callbacks can arrive on different threads.
use std::{cell::Cell,collections::VecDeque,io,mem,ptr,sync::{Mutex,atomic::{AtomicUsize,AtomicU64,Ordering}}};
use crate::{native::write_log,position::Sample,Role};
type Object=*mut u8;
type Clamp=unsafe extern "C" fn(Object,u32,f32)->f32;
static ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static RESTORE_ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static EFFECT_TABLE:AtomicUsize=AtomicUsize::new(0);
static EFFECT_ORIGINAL:AtomicUsize=AtomicUsize::new(0);
static EFFECT_INSTALL_ATTEMPTED:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
type EffectModify=unsafe extern "C" fn(Object,Object,f32,u32);
thread_local! {static EFFECT_CAPTURE_DEPTH:Cell<u32>=const {Cell::new(0)};}
/// A qualifying effect owns its restoration, including nested calls to owner slot 6.
struct EffectCaptureGuard(u32);
impl EffectCaptureGuard {
    fn enter()->Self {EFFECT_CAPTURE_DEPTH.with(|depth| {let old=depth.get();depth.set(old.saturating_add(1));Self(old)})}
}
impl Drop for EffectCaptureGuard {
    fn drop(&mut self) {EFFECT_CAPTURE_DEPTH.with(|depth|depth.set(self.0));}
}
fn effect_owns_restore()->bool {EFFECT_CAPTURE_DEPTH.with(|depth|depth.get()!=0)}
static MODIFIER:AtomicUsize=AtomicUsize::new(0);
static NEXT:AtomicU64=AtomicU64::new(1);
static INSTALL:Mutex<()>=Mutex::new(());
#[derive(Clone,Copy,PartialEq)]
struct Key {sender:u64,world:u64,revision:u64,save:[u8;16],mc:[u8;16]}
/// `shield` is the time until which a paused Minecraft protects the player; `shielded`
/// counts the hits cancelled since it came on.
/// Queue entries are (sequence, share of full health, Skyrim amount, time). A negative
/// Skyrim amount means health restored instead of taken.
/// `bonus` is the health Skyrim's character had on top of its plain amount when last looked
/// at in this generation (a fortify-health potion, an enchantment); `None` until first seen.
struct State {key:Option<Key>,epoch:u64,sequence:u64,player:usize,until:u64,restore_until:u64,maximum:f32,queue:VecDeque<(u64,f32,f32,u64)>,shield:u64,shielded:u64,restore:Restore,bonus:Option<f32>}
static STATE:Mutex<State>=Mutex::new(State{key:None,epoch:0,sequence:0,player:0,until:0,restore_until:0,maximum:0.0,queue:VecDeque::new(),shield:0,shielded:0,restore:Restore{start:0,sum:0.0},bonus:None});
/// The character the bridge is steering, for the raw effect log; and how many such lines
/// this session has written.
static TRACKED:AtomicUsize=AtomicUsize::new(0);
static RAW_LINES:AtomicU64=AtomicU64::new(0);
const RAW_LINE_LIMIT:u64=200;
/// A change of the health bonus smaller than this is noise, not a potion.
const BONUS_STEP:f32=0.5;
/// How much the health bonus rose since the look before, if it did. Skyrim's "Potion of
/// Health" does not restore health: it raises the maximum, and the current health with it,
/// for a minute (report 20261012-052857: 100 to 120 after each drink, back sixty seconds
/// later). The first look of a generation only remembers: a save loaded with the potion
/// still running must not heal.
fn bonus_gain(before:Option<f32>,now:f32)->Option<f32> {
    let before=before?;
    let gain=now-before;
    (before.is_finite() && now.is_finite() && gain>=BONUS_STEP).then_some(gain)
}
/// That rise as a share of the plain health: 20 on a plain 100 is a fifth, two hearts.
fn bonus_share(gain:f32,plain:f32)->Option<f32> {
    (gain.is_finite() && gain>0.0 && plain.is_finite() && plain>0.0).then(||(gain/plain).min(1.0))
}
/// What an effect call is about, read before the game runs it and without any filter: the
/// actor value it changes, the base effect's flags, and the form IDs of the base effect and
/// of the item or spell. Zero where a pointer is missing.
unsafe fn effect_facts(effect:Object,value:u32)->(u32,u32,u32,u32) {
    if effect.is_null() {return (value,0,0,0);}
    let value=if value==u32::MAX {unsafe {ptr::read_unaligned(effect.add(0x90).cast::<u32>())}} else {value};
    let item=unsafe {ptr::read_unaligned(effect.add(0x40).cast::<Object>())};
    let item_id=if item.is_null() {0} else {unsafe {ptr::read_unaligned(item.add(0x14).cast::<u32>())}};
    let definition=unsafe {ptr::read_unaligned(effect.add(0x48).cast::<Object>())};
    if definition.is_null() {return (value,0,0,item_id);}
    let base=unsafe {ptr::read_unaligned(definition.add(0x10).cast::<Object>())};
    if base.is_null() {return (value,0,0,item_id);}
    (value,unsafe {ptr::read_unaligned(base.add(0x68).cast::<u32>())},unsafe {ptr::read_unaligned(base.add(0x14).cast::<u32>())},item_id)
}
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
    let ready=restore_ready(&s,player,now,crate::skse::menu_ready(),crate::save_identity::revision());
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
/// The effect route uses the same live player/menu lease as the existing owner route.
fn restore_ready(s:&State,player:Object,now:u64,menu_ready:bool,revision:u64)->bool {
    s.restore_until>now && s.player==player as usize && menu_ready
        && s.key.is_some_and(|k|k.revision==revision)
}
/// ValueModifierEffect has no FormID: its small native effect ID identifies an instance.
#[derive(Clone,Copy,Debug,PartialEq)]
struct EffectHeal {value:u32,effect_id:u16,base_id:u32,item_id:u32}
/// Read the native request before the callback; an instant effect may be released afterward.
unsafe fn effect_heal(effect:Object,player:Object,requested:f32,value:u32,enabled:bool)->Option<EffectHeal> {
    if !enabled || effect.is_null() || player.is_null() || !requested.is_finite() || requested<=0.0 {return None;}
    let value=if value==u32::MAX {unsafe {ptr::read_unaligned(effect.add(0x90).cast::<u32>())}} else {value};
    if value!=24 {return None;}
    let definition=unsafe {ptr::read_unaligned(effect.add(0x48).cast::<Object>())};
    if definition.is_null() {return None;}
    let base=unsafe {ptr::read_unaligned(definition.add(0x10).cast::<Object>())};
    if base.is_null() {return None;}
    let flags=unsafe {ptr::read_unaligned(base.add(0x68).cast::<u32>())};
    // kRecover modifies permanent/temporary values; fortifying maximum health isn't healing.
    if flags&2!=0 {return None;}
    let item=unsafe {ptr::read_unaligned(effect.add(0x40).cast::<Object>())};
    Some(EffectHeal{value,effect_id:unsafe {ptr::read_unaligned(effect.add(0x84).cast::<u16>())},
        base_id:unsafe {ptr::read_unaligned(base.add(0x14).cast::<u32>())},
        item_id:if item.is_null() {0} else {unsafe {ptr::read_unaligned(item.add(0x14).cast::<u32>())}}})
}
/// ValueModifierEffect::ModifyActorValue slot 0x20. Ordinary potion effects call the
/// native actor modifier directly, bypassing ActorValueOwner's virtual slot 6.
unsafe extern "C" fn intercept_effect(effect:Object,player:Object,requested:f32,value:u32) {
    let original=EFFECT_ORIGINAL.load(Ordering::Acquire);if original==0 {return;}
    let call:EffectModify=unsafe {mem::transmute(original)};
    let enabled=crate::switches::on(Role::Skyrim,crate::switches::EFFECT_HEALING)
        && crate::switches::on(Role::Skyrim,crate::switches::HEALING);
    // Every call on the steered character, before any filter, so that a log can tell
    // "the game never called this" from "the filter left it out".
    if !player.is_null() && player as usize==TRACKED.load(Ordering::Acquire)
        && RAW_LINES.fetch_add(1,Ordering::Relaxed)<RAW_LINE_LIMIT {
        let _=std::panic::catch_unwind(|| {
            let (resolved,flags,base_id,item_id)=unsafe {effect_facts(effect,value)};
            let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_EFFECT_RAW requested={requested} value={resolved} recover={} flags=0x{flags:08X} base_id=0x{base_id:08X} item_id=0x{item_id:08X}",
                flags&2!=0));
        });
    }
    let _=std::panic::catch_unwind(|| unsafe {dispatch_effect(effect,player,requested,value,enabled,call,|p,amount,metadata| {
        let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_EFFECT_REQUEST requested={amount} actor={p:p} value={} effect_id={} base_id=0x{:08X} item_id=0x{:08X}",
            metadata.value,metadata.effect_id,metadata.base_id,metadata.item_id));
        forward_restore(p,amount);
    })});
}
unsafe fn dispatch_effect(effect:Object,player:Object,requested:f32,value:u32,enabled:bool,
    call:EffectModify,capture:impl FnOnce(Object,f32,EffectHeal)) {
    let metadata=unsafe {effect_heal(effect,player,requested,value,enabled)};
    {
        let _ownership=metadata.map(|_|EffectCaptureGuard::enter());
        // Native calls can reenter this module. No damage-state lock is held here.
        unsafe {call(effect,player,requested,value);}
    }
    if let Some(metadata)=metadata {capture(player,requested,metadata);}
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
    if !effect_owns_restore() && restoration(modifier,value,requested) {capture(unsafe {owner.sub(0xB8)},requested);}
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
/// Optional table: missing runtime support leaves the old owner route available.
pub fn configure_effect(table:usize) {EFFECT_TABLE.store(table,Ordering::Release);}
pub fn suspend() {if let Ok(mut s)=STATE.lock() {s.until=0;}}
static LETHAL:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);
/// While set, the bridge's own lethal hit on the player must reach Skyrim unchanged.
pub fn lethal(on:bool) {LETHAL.store(on,Ordering::Release);}
pub fn reset() {
    if let Ok(mut s)=STATE.lock() {
        if !s.queue.is_empty() {let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_DROP epoch={} count={} reason=reset",s.epoch,s.queue.len()));}
        shield_off(&mut s,"reset");
        s.key=None;s.until=0;s.restore_until=0;s.restore=Restore{start:0,sum:0.0};s.queue.clear();s.bonus=None;
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
    let effect_table=EFFECT_TABLE.load(Ordering::Acquire);
    if effect_table!=0 && crate::switches::on(Role::Skyrim,crate::switches::EFFECT_HEALING)
        && !EFFECT_INSTALL_ATTEMPTED.swap(true,Ordering::AcqRel) {
        let slot=unsafe {(effect_table as *mut usize).add(0x20)};let mut protection=0;
        if unsafe {VirtualProtect(slot.cast(),8,0x04,&mut protection)}==0 {
            // This optional repair must not disable the already-working movement/damage link.
            let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_EFFECT_HOOK state=unavailable reason=protect_error error={:?}",io::Error::last_os_error()));
        } else {
            EFFECT_ORIGINAL.store(unsafe {*slot},Ordering::Release);
            unsafe {ptr::write(slot,intercept_effect as *const () as usize);}
            let mut ignored=0;let restored=unsafe {VirtualProtect(slot.cast(),8,protection,&mut ignored)};
            let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_EFFECT_HOOK state=installed slot=32 protection_restored={}",restored!=0));
        }
    }
    drop(installation);
    let permanent:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(2))};
    let temporary:unsafe extern "C" fn(Object,u32,u32)->f32=unsafe {mem::transmute(modifier)};
    let (lasting,passing)=unsafe {(permanent(owner,24),temporary(player,1,24))};
    let maximum=lasting+passing;
    // ActorValueOwner slot 3: the plain value, without any bonus (CommonLib ActorValueOwner.h).
    let plain_of:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(3))};
    let plain=unsafe {plain_of(owner,24)};
    TRACKED.store(player as usize,Ordering::Release);
    let key=Key{sender:sample.sender,world:sample.world,revision:crate::save_identity::revision(),save:sample.save_id,mc:sample.mc_id};
    let mut s=STATE.lock().map_err(|_|io::Error::other("damage state poisoned"))?;
    // Minecraft is playing again: its own health takes the hits from here.
    shield_off(&mut s,"resumed");
    if s.key!=Some(key) {
        if !s.queue.is_empty() {let _=write_log(Role::Skyrim,format!("event=SCENE_DAMAGE_DROP epoch={} count={} reason=generation_changed",s.epoch,s.queue.len()));}
        s.key=Some(key);s.epoch=NEXT.fetch_add(1,Ordering::Relaxed);s.sequence=0;s.queue.clear();s.restore=Restore{start:0,sum:0.0};s.bonus=None;
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
    // A fortify-health potion raises the maximum and the current health together; no
    // "restore" call is made for it. Forward the rise as a heal (see `bonus_gain`).
    let usable=plain.is_finite() && plain>0.0;
    let (bonus,whole)=if usable {(maximum-plain,plain)} else {(passing,maximum-passing)};
    let before=s.bonus;
    s.bonus=bonus.is_finite().then_some(bonus);
    if before.is_some_and(|old|(old-bonus).abs()>=BONUS_STEP) || (before.is_none() && bonus.is_finite() && bonus.abs()>=BONUS_STEP) {
        let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_FORTIFY epoch={} state={} bonus={bonus} was={} plain={plain} lasting={lasting} passing={passing} skyrim_max={maximum}",
            s.epoch,match before {None=>"first_seen",Some(old) if bonus>old=>"raised",Some(_)=>"lowered"},
            before.map_or_else(||"unseen".to_owned(),|old|old.to_string())));
    }
    if let Some(gain)=bonus_gain(before,bonus) {
        let on=crate::switches::on(Role::Skyrim,crate::switches::HEALING) && crate::switches::on(Role::Skyrim,crate::switches::FORTIFY_HEALING);
        match bonus_share(gain,whole) {
            Some(fraction) if on && s.queue.len()<32=>{
                s.sequence+=1;let seq=s.sequence;
                s.queue.push_back((seq,fraction,-gain,now));
                let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_CAPTURE epoch={} sequence={seq} skyrim_heal={gain} skyrim_max={whole} fraction={fraction} source=fortify",s.epoch));
            }
            _=>{let _=write_log(Role::Skyrim,format!("event=SCENE_HEAL_DROP epoch={} skyrim_heal={gain} reason={}",s.epoch,if !on {"switched_off"} else if s.queue.len()>=32 {"queue_full"} else {"no_plain_health"}));}
        }
    }
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
        // Positive requests belong to the effect/owner routes before native clamping.
        // Never capture again from a nested clamp call.
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
    /// These are only interface-sized fake native objects; no installed game data is used.
    struct EffectFixture {effect:[u64;20],definition:[u64;4],base:[u64;24],item:[u64;4],player:[u64;40]}
    impl EffectFixture {
        fn new()->Self {Self{effect:[0;20],definition:[0;4],base:[0;24],item:[0;4],player:[0;40]}}
        fn link(&mut self)->(Object,Object) {
            let effect=self.effect.as_mut_ptr().cast::<u8>();let player=self.player.as_mut_ptr().cast::<u8>();
            unsafe {
                ptr::write_unaligned(effect.add(0x48).cast::<Object>(),self.definition.as_mut_ptr().cast());
                ptr::write_unaligned(self.definition.as_mut_ptr().cast::<u8>().add(0x10).cast::<Object>(),self.base.as_mut_ptr().cast());
                ptr::write_unaligned(effect.add(0x40).cast::<Object>(),self.item.as_mut_ptr().cast());
                ptr::write_unaligned(effect.add(0x90).cast::<u32>(),24);
                ptr::write_unaligned(effect.add(0x84).cast::<u16>(),17);
                ptr::write_unaligned(self.base.as_mut_ptr().cast::<u8>().add(0x14).cast::<u32>(),0x3EB15);
                ptr::write_unaligned(self.item.as_mut_ptr().cast::<u8>().add(0x14).cast::<u32>(),0x3EADD);
                ptr::write_unaligned(player.cast::<f32>(),120.0);
            }
            (effect,player)
        }
    }
    thread_local! {
        static NATIVE_EFFECT_CALLS:std::cell::RefCell<Vec<(usize,usize,f32,u32)>>=const {std::cell::RefCell::new(Vec::new())};
        static NATIVE_OWNER_CALLS:Cell<usize>=const {Cell::new(0)};
        static OWNER_CAPTURES:Cell<usize>=const {Cell::new(0)};
    }
    unsafe extern "C" fn direct_effect(effect:Object,player:Object,amount:f32,value:u32) {
        NATIVE_EFFECT_CALLS.with(|calls|calls.borrow_mut().push((effect as usize,player as usize,amount,value)));
        // Emulate the real direct actor-modifier path: no owner-slot dispatch, full-health delta zero.
        let health=unsafe {ptr::read_unaligned(player.cast::<f32>())};
        unsafe {ptr::write_unaligned(player.cast::<f32>(),(health+amount).min(120.0));}
        // Proves the callback copied metadata before native code can clear an instant effect.
        if !effect.is_null() {unsafe {ptr::write_unaligned(effect.add(0x48).cast::<Object>(),ptr::null_mut());}}
    }
    unsafe extern "C" fn native_owner(_owner:Object,_modifier:u32,_value:u32,_amount:f32) {
        NATIVE_OWNER_CALLS.with(|calls|calls.set(calls.get()+1));
    }
    unsafe extern "C" fn owner_from_effect(_effect:Object,player:Object,amount:f32,_value:u32) {
        unsafe {dispatch_restore(player.add(0xB8),2,24,amount,native_owner,|_,_|OWNER_CAPTURES.with(|calls|calls.set(calls.get()+1)));}
    }
    #[test]
    fn native_effect_direct_call_preserves_full_health_request_and_copies_metadata() {
        let mut fixture=EffectFixture::new();let (effect,player)=fixture.link();
        NATIVE_EFFECT_CALLS.with(|calls|calls.borrow_mut().clear());
        let mut captured=None;
        unsafe {dispatch_effect(effect,player,25.0,u32::MAX,true,direct_effect,|p,amount,metadata|captured=Some((p,amount,metadata)));}
        NATIVE_EFFECT_CALLS.with(|calls|assert_eq!(*calls.borrow(),[(effect as usize,player as usize,25.0,u32::MAX)]));
        assert_eq!(unsafe {ptr::read_unaligned(player.cast::<f32>())},120.0);
        assert_eq!(captured,Some((player,25.0,EffectHeal{value:24,effect_id:17,base_id:0x3EB15,item_id:0x3EADD})));
        assert!(!effect_owns_restore());
    }
    #[test]
    fn effect_fallback_preserves_one_native_call_and_unchanged_arguments() {
        unsafe extern "C" fn observe(effect:Object,player:Object,amount:f32,value:u32) {
            NATIVE_EFFECT_CALLS.with(|calls|calls.borrow_mut().push((effect as usize,player as usize,amount,value)));
        }
        let mut fixture=EffectFixture::new();let (effect,player)=fixture.link();
        for (e,p,amount,value,enabled) in [(effect,player,25.0,u32::MAX,false),
            (effect,player,25.0,25,true),(effect,player,-25.0,24,true),
            (effect,player,f32::NAN,24,true),(ptr::null_mut(),player,25.0,24,true),
            (effect,ptr::null_mut(),25.0,24,true)] {
            NATIVE_EFFECT_CALLS.with(|calls|calls.borrow_mut().clear());
            let mut captures=0;
            unsafe {dispatch_effect(e,p,amount,value,enabled,observe,|_,_,_|captures+=1);}
            assert_eq!(captures,0);assert!(!effect_owns_restore());
            NATIVE_EFFECT_CALLS.with(|calls| {
                let calls=calls.borrow();assert_eq!(calls.len(),1);
                assert_eq!((calls[0].0,calls[0].1,calls[0].2.to_bits(),calls[0].3),
                    (e as usize,p as usize,amount.to_bits(),value));
            });
        }
    }
    #[test]
    fn qualifying_effect_suppresses_only_nested_owner_capture_and_off_restores_it() {
        let mut fixture=EffectFixture::new();let (effect,player)=fixture.link();
        NATIVE_OWNER_CALLS.with(|calls|calls.set(0));OWNER_CAPTURES.with(|calls|calls.set(0));
        let mut effect_captures=0;
        unsafe {dispatch_effect(effect,player,25.0,u32::MAX,true,owner_from_effect,|_,_,_|effect_captures+=1);}
        assert_eq!(effect_captures,1);
        NATIVE_OWNER_CALLS.with(|calls|assert_eq!(calls.get(),1));OWNER_CAPTURES.with(|calls|assert_eq!(calls.get(),0));
        // Disabled or nonqualifying effect routes leave the preexisting owner route alone.
        unsafe {dispatch_effect(effect,player,25.0,u32::MAX,false,owner_from_effect,|_,_,_|effect_captures+=1);}
        unsafe {ptr::write_unaligned(fixture.base.as_mut_ptr().cast::<u8>().add(0x68).cast::<u32>(),2);}
        unsafe {dispatch_effect(effect,player,25.0,u32::MAX,true,owner_from_effect,|_,_,_|effect_captures+=1);}
        assert_eq!(effect_captures,1);
        NATIVE_OWNER_CALLS.with(|calls|assert_eq!(calls.get(),3));OWNER_CAPTURES.with(|calls|assert_eq!(calls.get(),2));
        assert!(!effect_owns_restore());
    }
    #[test]
    fn effect_filter_rejects_other_values_recovering_buffs_and_missing_metadata() {
        let mut fixture=EffectFixture::new();let (effect,player)=fixture.link();
        assert_eq!(unsafe {effect_heal(effect,player,25.0,24,true)}.map(|m|m.value),Some(24));
        for (value,amount,enabled) in [(25,25.0,true),(24,-25.0,true),(24,0.0,true),
            (24,f32::NAN,true),(24,f32::INFINITY,true),(24,25.0,false)] {
            assert!(unsafe {effect_heal(effect,player,amount,value,enabled)}.is_none());
        }
        assert!(unsafe {effect_heal(ptr::null_mut(),player,25.0,24,true)}.is_none());
        assert!(unsafe {effect_heal(effect,ptr::null_mut(),25.0,24,true)}.is_none());
        unsafe {ptr::write_unaligned(effect.add(0x90).cast::<u32>(),25);}
        assert!(unsafe {effect_heal(effect,player,25.0,u32::MAX,true)}.is_none());
        // Explicit health takes precedence over the sentinel field, matching the native ABI.
        assert!(unsafe {effect_heal(effect,player,25.0,24,true)}.is_some());
        unsafe {ptr::write_unaligned(fixture.base.as_mut_ptr().cast::<u8>().add(0x68).cast::<u32>(),2);}
        assert!(unsafe {effect_heal(effect,player,25.0,24,true)}.is_none());
        unsafe {ptr::write_unaligned(fixture.base.as_mut_ptr().cast::<u8>().add(0x68).cast::<u32>(),0);}
        unsafe {ptr::write_unaligned(effect.add(0x40).cast::<Object>(),ptr::null_mut());}
        assert_eq!(unsafe {effect_heal(effect,player,25.0,24,true)}.map(|m|m.item_id),Some(0));
        unsafe {ptr::write_unaligned(fixture.definition.as_mut_ptr().cast::<u8>().add(0x10).cast::<Object>(),ptr::null_mut());}
        assert!(unsafe {effect_heal(effect,player,25.0,24,true)}.is_none());
        unsafe {ptr::write_unaligned(effect.add(0x48).cast::<Object>(),ptr::null_mut());}
        assert!(unsafe {effect_heal(effect,player,25.0,24,true)}.is_none());
    }
    #[test]
    fn effect_capture_guard_restores_nesting_after_unwind() {
        assert!(!effect_owns_restore());
        let outer=EffectCaptureGuard::enter();
        EFFECT_CAPTURE_DEPTH.with(|depth|assert_eq!(depth.get(),1));
        let caught=std::panic::catch_unwind(|| {let _inner=EffectCaptureGuard::enter();
            EFFECT_CAPTURE_DEPTH.with(|depth|assert_eq!(depth.get(),2));panic!("fixture unwind");});
        assert!(caught.is_err());
        EFFECT_CAPTURE_DEPTH.with(|depth|assert_eq!(depth.get(),1));
        drop(outer);assert!(!effect_owns_restore());
    }
    #[test]
    fn effect_forwarding_requires_fresh_player_revision_and_menu_lease() {
        let key=Key{sender:1,world:2,revision:5,save:[3;16],mc:[4;16]};
        let player=0x1000_usize as Object;
        let mut state=State{key:Some(key),epoch:1,sequence:0,player:player as usize,until:0,
            restore_until:1250,maximum:120.0,queue:VecDeque::new(),shield:0,shielded:0,restore:Restore{start:0,sum:0.0},bonus:None};
        assert!(restore_ready(&state,player,1000,true,5));
        assert!(!restore_ready(&state,player,1250,true,5));
        assert!(!restore_ready(&state,player,1000,false,5));
        assert!(!restore_ready(&state,player,1000,true,6));
        assert!(!restore_ready(&state,0x2000_usize as Object,1000,true,5));
        state.key=None;assert!(!restore_ready(&state,player,1000,true,5));
    }
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
    /// Numbers of report 20261012-052857: maximum 100, then 120 after the potion, then 100
    /// again a minute later; and 120 already at the start of the session.
    #[test]
    fn a_rise_of_the_health_bonus_is_a_heal_and_nothing_else_is() {
        // The first look only remembers, whatever it finds.
        assert_eq!(bonus_gain(None,20.0),None);
        assert_eq!(bonus_gain(None,0.0),None);
        // The potion: 0 to 20. A stronger one on top: 20 to 40.
        assert_eq!(bonus_gain(Some(0.0),20.0),Some(20.0));
        assert_eq!(bonus_gain(Some(20.0),40.0),Some(20.0));
        // It runs out, or nothing changes: no heal, and no harm either.
        assert_eq!(bonus_gain(Some(20.0),0.0),None);
        assert_eq!(bonus_gain(Some(20.0),20.0),None);
        // Rounding noise is not a potion; half a point is the least that counts.
        assert_eq!(bonus_gain(Some(20.0),20.4),None);
        assert_eq!(bonus_gain(Some(0.0),0.5),Some(0.5));
        for bad in [f32::NAN,f32::INFINITY,f32::NEG_INFINITY] {
            assert_eq!(bonus_gain(Some(0.0),bad),None);
            assert_eq!(bonus_gain(Some(bad),20.0),None);
        }
        // 20 on a plain 100 is a fifth of Minecraft's health: two of ten hearts.
        assert_eq!(bonus_share(20.0,100.0),Some(0.2));
        assert_eq!(bonus_share(500.0,100.0),Some(1.0));
        for (gain,plain) in [(0.0,100.0),(-20.0,100.0),(20.0,0.0),(20.0,-1.0),(f32::NAN,100.0),(20.0,f32::NAN)] {
            assert_eq!(bonus_share(gain,plain),None,"{gain} {plain}");
        }
        // The event that travels is valid for Minecraft's receiver: a share above zero and
        // a negative Skyrim amount (BridgeRules.validHealthEvent, isRestore).
        let (fraction,raw)=(bonus_share(20.0,100.0).unwrap(),-20.0_f32);
        assert!(fraction>0.0 && fraction.is_finite() && raw<0.0);
    }
    #[test]
    fn the_raw_effect_line_reads_no_further_than_the_pointers_allow() {
        let mut fixture=EffectFixture::new();
        let (effect,_)=fixture.link();
        unsafe {ptr::write_unaligned(fixture.base.as_mut_ptr().cast::<u8>().add(0x68).cast::<u32>(),0x200A02);}
        // The value is taken from the effect when the caller passes "the effect's own".
        assert_eq!(unsafe {effect_facts(effect,u32::MAX)},(24,0x200A02,0x3EB15,0x3EADD));
        assert_eq!(unsafe {effect_facts(effect,26)},(26,0x200A02,0x3EB15,0x3EADD));
        // No definition: the item is still named. No effect at all: nothing is read.
        unsafe {ptr::write_unaligned(effect.add(0x48).cast::<Object>(),ptr::null_mut());}
        assert_eq!(unsafe {effect_facts(effect,24)},(24,0,0,0x3EADD));
        assert_eq!(unsafe {effect_facts(ptr::null_mut(),24)},(24,0,0,0));
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
