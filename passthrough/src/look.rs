//! Minecraft look -> Skyrim's ordinary first-person camera. No raw mouse injection.
use std::{io,mem,ptr,sync::atomic::{AtomicU32,Ordering}};
use crate::{position::Sample,native::write_log,Role};
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,at:usize)->T { unsafe { ptr::read_unaligned(p.add(at).cast()) } }
pub struct Api { pub singleton:usize,pub first:usize,pub third:usize }
/// Retain the node so reload/model replacement cannot leave a dangling restore pointer.
#[derive(Default)]
struct Hands {node:Option<(usize,bool)>}
impl Hands {
    unsafe fn restore(&mut self) {
        let Some((address,hidden))=self.node.take() else {return;};
        let node=address as Object;
        let flags=unsafe {read::<u32>(node,0xF4)};
        unsafe {ptr::write_unaligned(node.add(0xF4).cast::<u32>(),if hidden {flags|1} else {flags&!1});}
        let _=write_log(Role::Skyrim,format!("event=SCENE_NATIVE_HANDS state=restored hidden={hidden}"));
        let count=unsafe {&*node.add(8).cast::<AtomicU32>()};
        if count.fetch_sub(1,Ordering::SeqCst)==1 {
            let table:*const usize=unsafe {read(node,0)};
            let delete:unsafe extern "C" fn(Object)=unsafe {mem::transmute(*table.add(1))};
            unsafe {delete(node);}
        }
    }
    unsafe fn hide(&mut self,node:Object) {
        if self.node.map(|n|n.0)!=(!node.is_null()).then_some(node as usize) {
            unsafe {self.restore();}
            if !node.is_null() {
                let hidden=unsafe {read::<u32>(node,0xF4)}&1!=0;
                unsafe {(&*node.add(8).cast::<AtomicU32>()).fetch_add(1,Ordering::SeqCst);}
                self.node=Some((node as usize,hidden));
                let _=write_log(Role::Skyrim,format!("event=SCENE_NATIVE_HANDS state=hidden was_hidden={hidden}"));
            }
        }
        if !node.is_null() {
            let flags=unsafe {read::<u32>(node,0xF4)};
            unsafe {ptr::write_unaligned(node.add(0xF4).cast::<u32>(),flags|1);}
        }
    }
}
impl Drop for Hands {fn drop(&mut self) {unsafe {self.restore();}}}
#[derive(Default)]
pub struct Look { switched:Option<usize>,last_mode:Option<u32>,hands:Hands }
impl Look {
    pub unsafe fn release(&mut self,api:&Api) {
        unsafe {self.hands.restore();}
        let camera=unsafe { *(api.singleton as *const Object) };
        if let Some(saved)=self.switched.take() {
            if !camera.is_null() && camera as usize==saved {
                let state:Object=unsafe { read(camera,0x28) };
                if !state.is_null() && unsafe { read::<u32>(state,0x18) }==0 {
                    let third:unsafe extern "C" fn(Object)=unsafe { mem::transmute(api.third) };
                    unsafe { third(camera); }
                    let _=write_log(Role::Skyrim,"event=LOOK_RELEASE restored_third_person=true");
                }
            }
        }
        self.last_mode=None;
    }
    pub unsafe fn apply(&mut self,api:&Api,player:Object,sample:Sample,log:bool)->io::Result<()> {
        if crate::switches::on(Role::Skyrim,crate::switches::NATIVE_HANDS) {
            let table:*const usize=unsafe {read(player,0)};
            let get:unsafe extern "C" fn(Object,bool)->Object=unsafe {mem::transmute(*table.add(0x6F))};
            unsafe {self.hands.hide(get(player,true));}
        } else {unsafe {self.hands.restore();}}
        let camera=unsafe { *(api.singleton as *const Object) };
        if camera.is_null() { return Ok(()); }
        let mut state:Object=unsafe { read(camera,0x28) };
        if state.is_null() { return Ok(()); }
        let mode=unsafe { read::<u32>(state,0x18) };
        if self.last_mode!=Some(mode) {
            write_log(Role::Skyrim,format!("event=LOOK_MODE camera_state={mode} supported={}",mode==0 || mode==9))?;
            self.last_mode=Some(mode);
        }
        if mode==9 {
            let first:unsafe extern "C" fn(Object)=unsafe { mem::transmute(api.first) };
            self.switched=Some(camera as usize);
            unsafe { first(camera); }
            state=unsafe { read(camera,0x28) };
        }
        // Do not fight furniture, kill moves, mounts, free camera or transitions.
        if state.is_null() || unsafe { read::<u32>(state,0x18) }!=0 { return Ok(()); }
        let heading=(sample.yaw-180.0).rem_euclid(360.0).to_radians() as f32;
        let pitch=sample.pitch.to_radians() as f32;
        unsafe {
            ptr::write_unaligned(player.add(0x48).cast::<f32>(),pitch);
            ptr::write_unaligned(player.add(0x50).cast::<f32>(),heading);
        }
        if log {
            let actual_pitch:f32=unsafe { read(player,0x48) };
            let actual_heading:f32=unsafe { read(player,0x50) };
            let table:*const usize=unsafe { read(state,0) };
            let rotation:unsafe extern "C" fn(Object,*mut [f32;4])=unsafe { mem::transmute(*table.add(4)) };
            let mut q=[0.0;4]; unsafe { rotation(state,&mut q); }
            write_log(Role::Skyrim,format!("event=LOOK_RECV {} target_yaw_rad={heading} target_pitch_rad={pitch} actual_yaw_rad={actual_heading} actual_pitch_rad={actual_pitch} camera_state=0 camera_sample=before_next_camera_update camera_qw={} camera_qx={} camera_qy={} camera_qz={}",sample.log_fields(),q[0],q[1],q[2],q[3]))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn only_the_hidden_bit_is_owned_and_native_model_lifetime_is_retained() {
        let mut a=[0_usize;40];let one=a.as_mut_ptr().cast::<u8>();
        let mut b=[0_usize;40];let two=b.as_mut_ptr().cast::<u8>();
        unsafe {
            ptr::write_unaligned(one.add(8).cast::<u32>(),1);ptr::write_unaligned(two.add(8).cast::<u32>(),1);
            ptr::write_unaligned(one.add(0xF4).cast::<u32>(),0x20);
            ptr::write_unaligned(two.add(0xF4).cast::<u32>(),0x41); // already hidden
        }
        let mut hands=Hands::default();unsafe {hands.hide(one);hands.hide(one);}
        assert_eq!(unsafe {read::<u32>(one,8)},2);
        assert_eq!(unsafe {read::<u32>(one,0xF4)},0x21);
        // Native code changes an unrelated flag: releasing must preserve that change.
        unsafe {ptr::write_unaligned(one.add(0xF4).cast::<u32>(),0xA1);hands.hide(two);}
        assert_eq!(unsafe {read::<u32>(one,0xF4)},0xA0);
        assert_eq!(unsafe {read::<u32>(one,8)},1);assert_eq!(unsafe {read::<u32>(two,8)},2);
        unsafe {hands.hide(ptr::null_mut());}
        assert_eq!(unsafe {read::<u32>(two,0xF4)},0x41);assert_eq!(unsafe {read::<u32>(two,8)},1);
        unsafe {hands.hide(one);}
        drop(hands);
        assert_eq!(unsafe {read::<u32>(one,0xF4)},0xA0);assert_eq!(unsafe {read::<u32>(one,8)},1);
    }
}
