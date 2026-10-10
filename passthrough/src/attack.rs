//! Short-lived, target-bound basic punch requests, on the player update thread.
use std::{io,mem,ptr,sync::OnceLock};
use crate::{native::write_log,position::Sample,Role};
type Object=*mut u8;
struct Api {crosshair:usize,lookup:usize,damage:usize}
static API:OnceLock<Api>=OnceLock::new();
pub fn configure(crosshair:usize,lookup:usize,damage:usize) {let _=API.set(Api{crosshair,lookup,damage});}
unsafe fn read<T:Copy>(p:Object,n:usize)->T {unsafe {ptr::read_unaligned(p.add(n).cast())}}
unsafe fn handle(api:&Api)->u32 {
    let pick=unsafe {*(api.crosshair as *const Object)};
    if pick.is_null() {0} else {unsafe {read(pick,4)}}
}
/// The kind of thing a crosshair reference is, from its record types alone.
/// `base_type` is the form type of the object the reference is an instance of.
fn classify(reference_type:u8,base_type:Option<u8>,dead:bool)->u32 {
    use crate::world::*;
    // CommonLib FormTypes.h: 0x3E character reference, 0x1C container, 0x1D door.
    if reference_type==0x3E {return if dead {TARGET_BODY} else {TARGET_ALIVE};}
    match base_type {Some(0x1C)=>TARGET_CONTAINER,Some(0x1D)=>TARGET_DOOR,_=>TARGET_OTHER}
}
/// Look the crosshair handle up (reference counted, released on drop) and classify it.
unsafe fn kind(api:&Api,handle:u32)->u32 {
    if handle==0 || handle==u32::MAX {return crate::world::TARGET_NONE;}
    let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe {mem::transmute(api.lookup)};
    let mut target=crate::world_export::ActorRef(ptr::null_mut());
    if !unsafe {lookup(&handle,&mut target.0)} || target.0.is_null() {return crate::world::TARGET_NONE;}
    let reference_type:u8=unsafe {read(target.0,0x1A)};
    if reference_type==0x3E {
        // Life state as confirmed for the player in 0.10.8, with health as a cross-check.
        let life=(unsafe {read::<u32>(target.0,0xC8)}>>21)&0xF;
        let owner=unsafe {target.0.add(0xB8)};let table:*const usize=unsafe {read(owner,0)};
        let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
        let health=unsafe {get(owner,24)};
        return classify(reference_type,None,life==1 || life==2 || !(health>0.0));
    }
    // CommonLib TESObjectREFR.h: the base object pointer is the first member of data at 0x40.
    let base:Object=unsafe {read(target.0,0x40)};
    classify(reference_type,(!base.is_null()).then(|| unsafe {read::<u8>(base,0x1A)}),false)
}
static SEEN:std::sync::Mutex<(u32,u32)>=std::sync::Mutex::new((0,0));
pub unsafe fn snapshot(s:&mut crate::world::Snapshot) {
    if !(s.active && s.admitted) {return;}
    let Some(api)=API.get() else {return;};
    s.attack_handle=unsafe {handle(api)};
    s.target_kind=unsafe {kind(api,s.attack_handle)};
    if let Ok(mut seen)=SEEN.lock() {
        if *seen!=(s.attack_handle,s.target_kind) {
            *seen=(s.attack_handle,s.target_kind);
            let _=write_log(Role::Skyrim,format!("event=SCENE_CROSSHAIR handle={} kind={}",s.attack_handle,s.target_kind));
        }
    }
}
pub struct Attack {last:Option<(u64,u64,u64)>,blocked:bool,called:u64}
impl Default for Attack {fn default()->Self {Self{last:None,blocked:true,called:0}}}
impl Attack {
    pub fn block(&mut self) {self.blocked=true;}
    pub unsafe fn apply(&mut self,player:Object,s:Sample)->io::Result<()> {
        if s.attack_seq==0 {self.blocked=false;return Ok(());}
        let key=(s.sender,s.world,s.attack_seq);if self.last==Some(key) {return Ok(());}self.last=Some(key);
        let Some(api)=API.get() else {return Ok(());};let now=crate::windows::now_ms();
        let reason=if self.blocked {"resume_barrier"}
            else if s.attack_ms==0 || s.attack_ms>now || now-s.attack_ms>250 {"expired"}
            else if !s.attack_damage.is_finite() || s.attack_damage<=0.0 || s.attack_damage>1000.0 {"invalid_damage"}
            else if now.saturating_sub(self.called)<100 {"cooldown"}
            else if s.attack_handle==0 || s.attack_handle==u32::MAX || unsafe {handle(api)}!=s.attack_handle {"target_changed"}
            else {"ready"};
        if reason!="ready" {return write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RECV sequence={} state=dropped reason={reason}",s.attack_seq));}
        let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe {mem::transmute(api.lookup)};
        let mut target=crate::world_export::ActorRef(ptr::null_mut());
        let found=unsafe {lookup(&s.attack_handle,&mut target.0)};
        if !found || target.0.is_null() || target.0==player || unsafe {read::<u8>(target.0,0x1A)}!=0x3E {
            return write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RECV sequence={} state=dropped reason=not_actor",s.attack_seq));
        }
        let position:[f32;3]=unsafe {read(player,0x54)};let other:[f32;3]=unsafe {read(target.0,0x54)};
        let distance=(0..3).map(|i|(position[i] as f64-other[i] as f64).powi(2)).sum::<f64>().sqrt();
        if !distance.is_finite() || distance>210.0 || unsafe {read::<Object>(player,0x60)!=read::<Object>(target.0,0x60)} {
            return write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RECV sequence={} state=dropped reason=range_or_cell distance={distance}",s.attack_seq));
        }
        let owner=unsafe {target.0.add(0xB8)};let table:*const usize=unsafe {read(owner,0)};
        let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
        let before=unsafe {get(owner,24)};
        if !before.is_finite() || before<=0.0 {return Ok(());}
        let id:u32=unsafe {read(target.0,0x14)};self.called=now;
        write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RECV sequence={} handle={} target={id:08X} state=dispatch distance={distance} requested={} before={before}",s.attack_seq,s.attack_handle,s.attack_damage))?;
        let damage:unsafe extern "C" fn(Object,f32,Object,bool)->bool=unsafe {mem::transmute(api.damage)};
        let result=unsafe {damage(target.0,s.attack_damage,player,false)};
        let after=unsafe {get(owner,24)};
        write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RESULT sequence={} target={id:08X} returned={result} before={before} after={after}",s.attack_seq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::*;
    #[test]
    fn crosshair_targets_are_classified_by_record_type() {
        assert_eq!(classify(0x3D,Some(0x1C),false),TARGET_CONTAINER);
        assert_eq!(classify(0x3D,Some(0x1D),false),TARGET_DOOR);
        assert_eq!(classify(0x3D,Some(0x18),false),TARGET_OTHER);
        assert_eq!(classify(0x3D,None,false),TARGET_OTHER);
        // A character is a body only once it is dead, whatever its base record says.
        assert_eq!(classify(0x3E,None,false),TARGET_ALIVE);
        assert_eq!(classify(0x3E,None,true),TARGET_BODY);
        assert_eq!(classify(0x3E,Some(0x1C),true),TARGET_BODY);
        for kind in [TARGET_NONE,TARGET_OTHER,TARGET_CONTAINER,TARGET_ALIVE,TARGET_BODY,TARGET_DOOR] {
            assert!(kind<=MAX_TARGET_KIND);
        }
    }
}
