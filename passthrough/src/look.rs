//! Minecraft look -> Skyrim's ordinary first-person camera. No raw mouse injection.
use std::{io,mem,ptr};
use crate::{position::Sample,native::write_log,Role};
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,at:usize)->T { unsafe { ptr::read_unaligned(p.add(at).cast()) } }
pub struct Api { pub singleton:usize,pub first:usize,pub third:usize }
#[derive(Default)]
pub struct Look { switched:Option<usize>,last_mode:Option<u32> }
impl Look {
    pub unsafe fn release(&mut self,api:&Api) {
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
