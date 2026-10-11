//! One bounded request per loaded save/session; never changes the saved mapping.
use crate::{position::Sample,native::write_log,Role};
#[derive(Clone,Copy,PartialEq)]
struct Key {revision:u64,sender:u64,world:u64,cell:u32,space:u32,save:[u8;16],mc:[u8;16]}
struct Pending {key:Key,token:u64,started:u64,target:[f64;3],sky:[f32;3],failed:bool}
#[derive(Default)]
pub struct Recovery {pending:Option<Pending>}
pub enum Reply {Waiting(u64,[f64;3]),Complete,Failed}
static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
impl Recovery {
    pub fn clear(&mut self) {self.pending=None;}
    pub fn pending(&self)->bool {self.pending.is_some()}
    pub fn poll(&mut self,s:Sample,sky:[f32;3],target:[f64;3],cell:u32,space:u32)->Reply {
        let key=Key{revision:crate::save_identity::revision(),sender:s.sender,world:s.world,cell,space,save:s.save_id,mc:s.mc_id};
        let now=crate::windows::now_ms();
        if self.pending.as_ref().is_none_or(|p|p.key!=key) {
            let token=NEXT.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
            let _=write_log(Role::Skyrim,format!("event=SCENE_RECOVERY state=request token={token} revision={} world={} mc_target={target:?} sky={sky:?}",key.revision,s.world));
            self.pending=Some(Pending{key,token,started:now,target,sky,failed:false});
        }
        let p=self.pending.as_mut().unwrap();
        let drift=(0..3).map(|i|(sky[i] as f64-p.sky[i] as f64).powi(2)).sum::<f64>();
        if !p.failed && (now.saturating_sub(p.started)>30_000 || drift>140.0*140.0) {
            p.failed=true;
            let _=write_log(Role::Skyrim,format!("event=SCENE_RECOVERY state=held token={} reason=timeout_or_skyrim_moved",p.token));
        }
        if p.failed {return Reply::Failed;}
        let error=(0..3).map(|i|(s.mc[i]-p.target[i]).powi(2)).sum::<f64>();
        if s.recovery_ack==p.token && error<=0.05*0.05 {
            let _=write_log(Role::Skyrim,format!("event=SCENE_RECOVERY state=accepted token={} mc={:?} error_squared={error}",p.token,s.mc));
            self.pending=None;return Reply::Complete;
        }
        Reply::Waiting(p.token,p.target)
    }
}

/// Native feet can lie just inside Minecraft's sampled floor. Resolve only a
/// bounded upward terrain overlap; walls/ceilings must not turn into a teleport.
pub fn clear_feet(mut target:[f64;3],boxes:&[[f64;6]])->Option<[f64;3]> {
    let start=target[1];
    let overlaps=|y:f64,b:&[f64;6]| {
        target[0]+0.3>b[0]+1e-6 && target[0]-0.3<b[3]-1e-6
        && target[2]+0.3>b[2]+1e-6 && target[2]-0.3<b[5]-1e-6
        && y+1.8>b[1]+1e-6 && y<b[4]-1e-6
    };
    let mut y=start;
    for _ in 0..=boxes.len() {
        let mut next=y;
        for b in boxes {
            if overlaps(y,b) {
                // Only terrain beneath/near the feet may supply support.
                if b[4]>start+0.5 {return None;}
                next=next.max(b[4]+0.0001);
            }
        }
        if next==y {target[1]=y;return Some(target);}
        if next-start>0.5001 {return None;}
        y=next;
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_low_chair_or_door_arrival_is_above_the_sampled_floor() {
        let chair=[-3131406.2493033274,127.99730584919453,-4151278.0556762693];
        let floor=[chair[0]-1.0,128.0,chair[2]-1.0,chair[0]+1.0,128.125,chair[2]+1.0];
        let fixed=clear_feet(chair,&[floor]).unwrap();
        assert!((fixed[1]-128.1251).abs()<1e-9);
        assert_eq!([fixed[0],fixed[2]],[chair[0],chair[2]]);
        let door=[322.6573091817958,40.01372506277902,-464.13952649426346];
        let floor=[322.0,40.0,-465.0,323.0,40.145,-464.0];
        assert!((clear_feet(door,&[floor]).unwrap()[1]-40.1451).abs()<1e-9);
    }
    #[test]
    fn recovery_does_not_lift_through_a_wall_or_low_ceiling() {
        let feet=[0.0,128.0,0.0];
        let floor=[-1.0,127.9,-1.0,1.0,128.125,1.0];
        let ceiling=[-1.0,129.85,-1.0,1.0,130.0,1.0];
        assert!(clear_feet(feet,&[floor,ceiling]).is_none());
        let wall=[-0.1,127.0,-1.0,0.1,131.0,1.0];
        assert!(clear_feet(feet,&[wall]).is_none());
        assert_eq!(clear_feet(feet,&[]),Some(feet));
        let nearby=[1.0,127.9,-1.0,2.0,128.125,1.0];
        assert_eq!(clear_feet(feet,&[nearby]),Some(feet));
        assert!(clear_feet(feet,&[[-1.0,127.9,-1.0,1.0,128.501,1.0]]).is_none());
    }
}
