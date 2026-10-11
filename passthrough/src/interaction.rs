//! One-shot requests dispatched through Skyrim's ordinary activation routine.
use std::{io,mem,ptr};
use crate::{position::Sample,Role,native::write_log};
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,offset:usize)->T { unsafe {ptr::read_unaligned(p.add(offset).cast())} }
/// `lookup` turns a crosshair handle into its reference; `lock` is TESObjectREFR::GetLock
/// (CommonLib TESObjectREFR.cpp), and without it the log says "unknown" for the lock.
pub struct Api { pub activate:usize,pub crosshair:usize,pub lookup:usize,pub lock:Option<usize>,pub midair:Option<usize> }
/// A lock as the log names it: no lock data, lock data that is open, or locked (CommonLib
/// ExtraLock.h REFR_LOCK: flags at 0x10, bit 0). `None` means it could not be asked.
fn lock_state(lock:Option<Option<u8>>)->&'static str {
    match lock {None=>"unknown",Some(None)=>"none",Some(Some(flags)) if flags&1!=0=>"locked",Some(Some(_))=>"open"}
}
/// What a press is about to meet, as one log line, and whether it is a seat. The reference
/// is held (and released on drop) only for the time of the reads.
unsafe fn describe(api:&Api,player:Object,target:u32,sequence:u64)->io::Result<(bool,bool,bool)> {
    let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe {mem::transmute(api.lookup)};
    let mut reference=crate::world_export::ActorRef(ptr::null_mut());
    if !unsafe {lookup(&target,&mut reference.0)} || reference.0.is_null() {
        write_log(Role::Skyrim,format!("event=SCENE_INTERACT_TARGET sequence={sequence} target_handle={target} state=not_found"))?;
        return Ok((false,false,false));
    }
    // CommonLib TESForm.h: form ID at 0x14.
    let form:u32=unsafe {read(reference.0,0x14)};
    let base:Object=unsafe {read(reference.0,0x40)};
    let base_form:u32=if base.is_null() {0} else {unsafe {read(base,0x14)}};
    let (base_type,bench,markers)=unsafe {crate::seat::furniture(reference.0)};
    // REFR_LOCK: level at 0x00, key at 0x08, flags at 0x10.
    let (lock,level,key)=match api.lock {
        None=>(None,0,0),
        Some(address)=>{
            let get:unsafe extern "C" fn(Object)->Object=unsafe {mem::transmute(address)};
            let data=unsafe {get(reference.0)};
            if data.is_null() {(Some(None),0,0)} else {
                let key:Object=unsafe {read(data,0x08)};
                (Some(Some(unsafe {read::<u8>(data,0x10)})),unsafe {read::<i8>(data,0)},if key.is_null() {0} else {unsafe {read::<u32>(key,0x14)}})
            }
        }
    };
    let seat=crate::seat::is_seat(base_type,bench,markers);
    let state=unsafe {crate::seat::of(player)};
    write_log(Role::Skyrim,format!("event=SCENE_INTERACT_TARGET sequence={sequence} target_handle={target} form={form:08X} base={base_form:08X} base_type={base_type:#04X} bench={bench} markers={markers:#010X} seat={seat} lock={} lock_level={level} key={key:08X} player_sit_state={state} player_sit={}",
        lock_state(lock),crate::seat::label(state)))?;
    Ok((seat,crate::seat::is_chair(base_type,bench,markers),base_type==0x28 && !seat))
}
#[derive(Clone,Copy)]
struct Ground {sample:Sample,target:u32,started:u64,revision:u64}
#[derive(Debug,PartialEq)]
enum GroundDecision {Wait,Dispatch,Cancel}
fn ground_decision(g:Ground,s:Sample,target:u32,revision:u64,now:u64,midair:bool)->GroundDecision {
    if s.active!=1 || (s.sender,s.world,s.save_id,s.mc_id)!=(g.sample.sender,g.sample.world,g.sample.save_id,g.sample.mc_id)
        || revision!=g.revision || now<g.started || now-g.started>=2000 {return GroundDecision::Cancel;}
    if midair {return GroundDecision::Wait;}
    if target!=g.target {GroundDecision::Cancel} else {GroundDecision::Dispatch}
}
pub struct Interaction { last:Option<(u64,u64,u64)>,blocked:bool,last_call:u64,chair_started:bool,ground:Option<Ground> }
impl Default for Interaction {
    fn default()->Self {Self{last:None,blocked:true,last_call:0,chair_started:false,ground:None}}
}
/// UI counters documented in CommonLib UI.h; dialogue retains the cursor.
pub unsafe fn menu_blocks(ui:Object)->bool {
    ui.is_null() || unsafe {
        read::<u32>(ui,0x160)!=0 || read::<u32>(ui,0x164)!=0
        || read::<u32>(ui,0x170)!=0 || read::<u32>(ui,0x178)!=0 || read::<u8>(ui,0x17C)!=0
    }
}
impl Interaction {
    pub fn block(&mut self) {self.blocked=true;self.chair_started=false;self.ground=None;}
    pub fn take_chair(&mut self)->bool {std::mem::take(&mut self.chair_started)}
    pub fn consume(&mut self,s:Sample) {
        if s.interact_seq!=0 {self.last=Some((s.sender,s.world,s.interact_seq));}
        else {self.blocked=false;}
    }
    pub fn grounding(&self)->bool {self.ground.is_some()}
    /// Called before steering; gravity is restored while the accepted request waits.
    pub unsafe fn poll_ground(&mut self,api:&Api,player:Object,sample:Sample)->io::Result<bool> {
        let Some(g)=self.ground else {return Ok(false);};
        self.consume(sample);
        let now=crate::windows::now_ms();
        let pick=unsafe {*(api.crosshair as *const Object)};
        let target=if pick.is_null() {0} else {unsafe {read::<u32>(pick,4)}};
        let airborne=api.midair.is_some_and(|address| {
            let check:unsafe extern "C" fn(Object)->bool=unsafe {mem::transmute(address)};
            unsafe {check(player)}
        });
        let decision=ground_decision(g,sample,target,crate::save_identity::revision(),now,airborne);
        if decision==GroundDecision::Wait {return Ok(true);}
        self.ground=None;
        write_log(Role::Skyrim,format!("event=SCENE_FURNITURE_GROUND state={} sequence={} midair={airborne} target_handle={target} age_ms={}",
            if decision==GroundDecision::Dispatch {"dispatch"} else {"cancelled"},g.sample.interact_seq,now.saturating_sub(g.started)))?;
        if decision==GroundDecision::Dispatch {
            self.last_call=now;
            let activate:unsafe extern "C" fn(Object)=unsafe {mem::transmute(api.activate)};
            unsafe {activate(player);}
        }
        Ok(false)
    }
    pub unsafe fn apply(&mut self,api:&Api,player:Object,sample:Sample)->io::Result<()> {
        self.chair_started=false;
        if sample.interact_seq==0 {self.blocked=false;return Ok(());}
        let key=(sample.sender,sample.world,sample.interact_seq);
        if self.last==Some(key) {return Ok(());}
        self.last=Some(key);
        let now=crate::windows::now_ms();
        let age=now.saturating_sub(sample.interact_ms);
        let reason=if self.blocked {"resume_barrier"}
            else if sample.interact_ms==0 || sample.interact_ms>now || age>250 {"expired"}
            else if self.last_call!=0 && now.saturating_sub(self.last_call)<250 {"cooldown"}
            else {"ready"};
        // Remain blocked until an idle request is seen, never replay on resume.
        if reason!="ready" {
            return write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RECV sender={} world={} sequence={} state=dropped reason={} age_ms={}",
                sample.sender,sample.world,sample.interact_seq,reason,age));
        }
        let pick=unsafe {*(api.crosshair as *const Object)};
        let target=if pick.is_null() {0} else {unsafe {read::<u32>(pick,4)}};
        if target==0 || target==u32::MAX {
            return write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RECV sender={} world={} sequence={} state=no_target target_handle={} age_ms={}",
                sample.sender,sample.world,sample.interact_seq,target,age));
        }
        // Ordinary chairs get deliberate ownership; non-menu work markers remain guarded.
        // The R key gets here without a crosshair kind check.
        let (seat,chair,menu_furniture)=unsafe {describe(api,player,target,sample.interact_seq)}?;
        let allow_chair=chair && crate::skse::sitting_available();
        if seat && !allow_chair && crate::switches::on(Role::Skyrim,crate::switches::SEAT_GUARD) {
            return write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RECV sender={} world={} sequence={} state=refused reason=seat target_handle={} age_ms={}",
                sample.sender,sample.world,sample.interact_seq,target,age));
        }
        if menu_furniture && api.midair.is_some() && crate::switches::on(Role::Skyrim,crate::switches::FURNITURE_GROUND) {
            self.ground=Some(Ground{sample,target,started:now,revision:crate::save_identity::revision()});
            return write_log(Role::Skyrim,format!("event=SCENE_FURNITURE_GROUND state=waiting sequence={} target_handle={target}",sample.interact_seq));
        }
        self.last_call=now;
        write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RECV sender={} world={} sequence={} state=dispatch target_handle={} age_ms={}",
            sample.sender,sample.world,sample.interact_seq,target,age))?;
        let activate:unsafe extern "C" fn(Object)=unsafe {mem::transmute(api.activate)};
        unsafe {activate(player);}
        self.chair_started=allow_chair;
        write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RESULT sequence={} state=returned outcome=game_owned",sample.interact_seq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grounding_is_bounded_and_never_activates_another_target_or_pair() {
        let s=Sample{sender:1,world:2,active:1,save_id:[3;16],mc_id:[4;16],..Sample::default()};
        let g=Ground{sample:s,target:9,started:1000,revision:5};
        assert_eq!(ground_decision(g,s,0,5,1100,true),GroundDecision::Wait);
        assert_eq!(ground_decision(g,s,9,5,1100,false),GroundDecision::Dispatch);
        for (other,target,rev,now) in [(Sample{sender:8,..s},9,5,1100),(Sample{world:8,..s},9,5,1100),
            (Sample{mc_id:[8;16],..s},9,5,1100),(s,9,6,1100),(s,10,5,1100),(s,9,5,3000),(s,9,5,999)] {
            assert_eq!(ground_decision(g,other,target,rev,now,false),GroundDecision::Cancel);
        }
    }
    #[test]
    fn a_lock_is_named_from_its_first_flag_bit() {
        assert_eq!(lock_state(None),"unknown");
        assert_eq!(lock_state(Some(None)),"none");
        // The meadery's front door carries lock data (level 25) that is open by day.
        assert_eq!(lock_state(Some(Some(0))),"open");
        assert_eq!(lock_state(Some(Some(1))),"locked");
        // Bit 2 is "levelled", not "locked".
        assert_eq!(lock_state(Some(Some(4))),"open");
        assert_eq!(lock_state(Some(Some(5))),"locked");
    }
}
