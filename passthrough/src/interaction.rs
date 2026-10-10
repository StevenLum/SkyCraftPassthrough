//! One-shot requests dispatched through Skyrim's ordinary activation routine.
use std::{io,mem,ptr};
use crate::{position::Sample,Role,native::write_log};
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,offset:usize)->T { unsafe {ptr::read_unaligned(p.add(offset).cast())} }
pub struct Api { pub activate:usize,pub crosshair:usize }
pub struct Interaction { last:Option<(u64,u64,u64)>,blocked:bool,last_call:u64 }
impl Default for Interaction {
    fn default()->Self {Self{last:None,blocked:true,last_call:0}}
}
/// UI counters documented in CommonLib UI.h; dialogue retains the cursor.
pub unsafe fn menu_blocks(ui:Object)->bool {
    ui.is_null() || unsafe {
        read::<u32>(ui,0x160)!=0 || read::<u32>(ui,0x164)!=0
        || read::<u32>(ui,0x170)!=0 || read::<u32>(ui,0x178)!=0 || read::<u8>(ui,0x17C)!=0
    }
}
impl Interaction {
    pub fn block(&mut self) {self.blocked=true;}
    pub unsafe fn apply(&mut self,api:&Api,player:Object,sample:Sample)->io::Result<()> {
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
        self.last_call=now;
        write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RECV sender={} world={} sequence={} state=dispatch target_handle={} age_ms={}",
            sample.sender,sample.world,sample.interact_seq,target,age))?;
        let activate:unsafe extern "C" fn(Object)=unsafe {mem::transmute(api.activate)};
        unsafe {activate(player);}
        write_log(Role::Skyrim,format!("event=SCENE_INTERACT_RESULT sequence={} state=returned outcome=game_owned",sample.interact_seq))
    }
}
