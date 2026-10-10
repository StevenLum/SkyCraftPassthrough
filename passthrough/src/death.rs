//! A Minecraft death kills Skyrim's player once; Skyrim's own death and reload follow.
use std::{io,mem,ptr,sync::atomic::{AtomicUsize,Ordering}};
use crate::{native::write_log,position::Sample,Role};
type Object=*mut u8;
static DAMAGE:AtomicUsize=AtomicUsize::new(0);
/// Actor::DoDamage, the same routine already used for hits on enemies.
pub fn configure(damage:usize) {DAMAGE.store(damage,Ordering::Release);}
unsafe fn read<T:Copy>(p:Object,n:usize)->T {unsafe {ptr::read_unaligned(p.add(n).cast())}}
/// Current health, and CommonLib ActorState life state (Actor+0xC0, state word +8, bits 21-24).
unsafe fn vitals(player:Object)->(f32,u32) {
    let owner=unsafe {player.add(0xB8)};let table:*const usize=unsafe {read(owner,0)};
    let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
    (unsafe {get(owner,24)},(unsafe {read::<u32>(player,0xC8)}>>21)&0xF)
}
/// Health at or below zero. Life state is logged only: the first run read it as 0 (alive)
/// straight after a lethal hit, so it cannot yet be trusted to decide a hold.
fn gone(health:f32,_life:u32)->bool {!(health>0.0)}
pub enum Verdict {Play,Hold,Kill}
#[derive(Default)]
pub struct Death {last:Option<(u64,u64,u64)>,grace:u64,held:bool,attempted:Option<u64>,life:Option<u32>}
impl Death {
    /// A save was loaded: what happened before no longer describes this player.
    pub fn loaded(&mut self) {self.grace=0;self.attempted=None;}
    /// Decide before any puppet work whether this frame plays, holds, or must kill first.
    pub unsafe fn check(&mut self,player:Object,s:Sample)->io::Result<Verdict> {
        let now=crate::windows::now_ms();
        let (health,life)=unsafe {vitals(player)};
        if gone(health,life) {self.attempted=None;}
        // Diagnostic only: shows whether the life-state read ever reports dying (1) or dead (2).
        if self.life!=Some(life) {
            self.life=Some(life);
            write_log(Role::Skyrim,format!("event=SCENE_DEATH_LIFE life={life} health={health}"))?;
        }
        let key=(s.sender,s.world,s.death_seq);
        if s.death_seq!=0 && self.last!=Some(key) {
            self.last=Some(key);
            let age=now.saturating_sub(s.death_ms);
            let decision=if DAMAGE.load(Ordering::Acquire)==0 {"no_address"}
                else if s.death_ms==0 || s.death_ms>now || age>10_000 {"expired"}
                else if gone(health,life) {"already_dead"} else {"kill"};
            write_log(Role::Skyrim,format!("event=SCENE_DEATH_RECV sequence={} age_ms={age} decision={decision} health={health} life={life}",s.death_seq))?;
            if decision=="kill" {return Ok(Verdict::Kill);}
        }
        let held=gone(health,life) || now<self.grace;
        if held!=self.held {
            self.held=held;
            write_log(Role::Skyrim,format!("event=SCENE_DEATH state={} health={health} life={life}",if held {"hold"} else {"clear"}))?;
            if !held {
                // Still standing after our own kill attempt: report it once and let play resume.
                if let Some(sequence)=self.attempted.take() {
                    write_log(Role::Skyrim,format!("event=SCENE_DEATH state=survived sequence={sequence} health={health} life={life}"))?;
                }
            }
        }
        Ok(if held {Verdict::Hold} else {Verdict::Play})
    }
    /// Call only after the puppet has been released, so gravity and camera are Skyrim's again.
    pub unsafe fn kill(&mut self,player:Object,s:Sample)->io::Result<()> {
        let address=DAMAGE.load(Ordering::Acquire);
        if address==0 {return Ok(());}
        let (before,life)=unsafe {vitals(player)};
        // Enough to end any health pool without an absurd magnitude.
        let amount=before.max(0.0)*2.0+100.0;
        self.grace=crate::windows::now_ms()+3000;self.held=true;self.attempted=Some(s.death_seq);
        write_log(Role::Skyrim,format!("event=SCENE_DEATH state=hold reason=kill sequence={} amount={amount} health={before} life={life}",s.death_seq))?;
        let damage:unsafe extern "C" fn(Object,f32,Object,bool)->bool=unsafe {mem::transmute(address)};
        crate::damage::lethal(true);
        // No attacker; true = do not scale by Skyrim's difficulty setting.
        let returned=unsafe {damage(player,amount,ptr::null_mut(),true)};
        crate::damage::lethal(false);
        let (after,life_after)=unsafe {vitals(player)};
        if gone(after,life_after) {self.attempted=None;}
        write_log(Role::Skyrim,format!("event=SCENE_DEATH_RESULT sequence={} returned={returned} health_before={before} health_after={after} life_before={life} life_after={life_after}",s.death_seq))
    }
}
