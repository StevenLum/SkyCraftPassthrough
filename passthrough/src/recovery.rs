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
    pub fn target(&self)->Option<[f64;3]> {self.pending.as_ref().map(|p|p.target)}
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

/// Same standing/NPC boxes as SkyCollision.java. Support is supplied only by
/// terrain; a body can block a return point but can never become its floor.
fn body_overlaps(feet:[f64;3],b:[f64;6])->bool {
    feet[0]+0.3>b[0]+1e-6 && feet[0]-0.3<b[3]-1e-6
        && feet[1]+1.8>b[1]+1e-6 && feet[1]<b[4]-1e-6
        && feet[2]+0.3>b[2]+1e-6 && feet[2]-0.3<b[5]-1e-6
}
pub fn blocking_terrain(feet:[f64;3],boxes:&[[f64;6]])->Option<[f64;6]> {
    boxes.iter().copied().find(|b|body_overlaps(feet,*b) && b[4]>feet[1]+0.5)
}
pub fn blocking_npc(feet:[f64;3],npcs:&[(u64,[f64;3])])->Option<(u64,[f64;3])> {
    npcs.iter().copied().find(|(_,p)|body_overlaps(feet,[p[0]-0.3,p[1],p[2]-0.3,p[0]+0.3,p[1]+1.8,p[2]+0.3]))
}
fn supported(feet:[f64;3],boxes:&[[f64;6]])->bool {
    boxes.iter().any(|b|feet[0]>=b[0]-1e-6 && feet[0]<=b[3]+1e-6
        && feet[2]>=b[2]-1e-6 && feet[2]<=b[5]+1e-6
        && (-1e-6..=0.1251).contains(&(feet[1]-b[4])))
}
/// Recheck a issued destination before acknowledging; a moving NPC can occupy it.
pub fn chair_destination_clear(target:[f64;3],boxes:&[[f64;6]],npcs:&[(u64,[f64;3])])->bool {
    supported(target,boxes) && blocking_npc(target,npcs).is_none()
        && clear_feet(target,boxes).is_some_and(|p|(p[1]-target[1]).abs()<1e-6)
}
/// Prefer the same authoritative pre-seat point. If an NPC moved into it, a
/// supported nearby point may be chosen, but never cross sampled terrain walls.
pub fn chair_return(entry:[f64;3],boxes:&[[f64;6]],npcs:&[(u64,[f64;3])])->Option<[f64;3]> {
    if !entry.iter().all(|v|v.is_finite() && v.abs()<30_000_000.0) {return None;}
    if let Some(clear)=clear_feet(entry,boxes) {
        if supported(clear,boxes) && blocking_npc(clear,npcs).is_none() {return Some(clear);}
    }
    let mut offsets=Vec::new();
    for dx in -6_i32..=6 {for dz in -6_i32..=6 {
        let distance=dx*dx+dz*dz;
        if distance>0 && distance<=36 {offsets.push((distance,dx,dz));}
    }}
    offsets.sort_unstable();
    for (_,dx,dz) in offsets {
        let at=[entry[0]+dx as f64*0.25,entry[1],entry[2]+dz as f64*0.25];
        // Ground at the candidate centre, no more than half a block from entry.
        let support=boxes.iter().filter(|b|at[0]>b[0]+1e-6 && at[0]<b[3]-1e-6
            && at[2]>b[2]+1e-6 && at[2]<b[5]-1e-6
            && (b[4]-entry[1]).abs()<=0.5).map(|b|b[4]).max_by(f64::total_cmp);
        let Some(y)=support else {continue;};
        let Some(clear)=clear_feet([at[0],y+0.0001,at[2]],boxes) else {continue;};
        if (clear[1]-entry[1]).abs()>0.5001 || blocking_npc(clear,npcs).is_some() {continue;}
        // Conservative swept standing box (also excludes diagonal corner cutting).
        // Small floor lift is the existing arrival correction, not a wall bypass.
        let bottom=entry[1].max(clear[1]);
        let sweep=[entry[0].min(clear[0])-0.3,bottom,entry[2].min(clear[2])-0.3,
            entry[0].max(clear[0])+0.3,bottom+1.8,entry[2].max(clear[2])+0.3];
        let blocked=boxes.iter().any(|b|(0..3).all(|j|sweep[j+3]>b[j]+1e-6 && sweep[j]<b[j+3]-1e-6));
        if !blocked {return Some(clear);}
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chair_returns_to_safe_entry_instead_of_a_low_obstructed_native_exit() {
        let entry=[0.0,128.125,0.0];
        let floor=[-3.0,127.9,-3.0,3.0,128.125,3.0];
        let table=[0.8,128.2,-0.5,1.8,129.3,0.5];
        assert!(clear_feet([1.0,127.98,0.0],&[floor,table]).is_none());
        let landed=chair_return(entry,&[floor,table],&[]).unwrap();
        assert_eq!([landed[0],landed[2]],[entry[0],entry[2]]);
        assert_eq!(landed,entry); // already at floor top: no unnecessary lift
        let npcs=[(42,[0.0,128.125,0.0])];
        assert_eq!(blocking_npc(entry,&npcs).unwrap().0,42);
        assert!(chair_destination_clear(landed,&[floor,table],&[]));
        assert!(!chair_destination_clear(landed,&[floor,table],&npcs));
        assert!(!chair_destination_clear(landed,&[],&[]));
        let around=chair_return(entry,&[floor,table],&npcs).unwrap();
        assert!(blocking_npc(around,&npcs).is_none());
        assert!((around[0].powi(2)+around[2].powi(2)).sqrt()<=1.5);
        assert!((around[1]-128.1251).abs()<1e-9);
        assert_eq!(blocking_terrain([1.0,127.98,0.0],&[floor,table]),Some(table));
    }
    #[test]
    fn chair_return_never_crosses_walls_or_uses_an_npc_as_support() {
        let entry=[0.0,128.125,0.0];let floor=[-3.0,127.9,-3.0,3.0,128.125,3.0];
        let npc=[(42,[0.0,128.125,0.0])];
        assert!(chair_return(entry,&[],&[]).is_none());
        assert!(chair_return(entry,&[[-3.0,120.0,-3.0,3.0,120.1,3.0]],&[]).is_none());
        // A body moved into entry but no terrain supports an alternative.
        assert!(chair_return(entry,&[],&npc).is_none());
        let walls=[[-0.7,128.2,-3.0,-0.6,131.0,3.0],[0.6,128.2,-3.0,0.7,131.0,3.0],
            [-3.0,128.2,-0.7,3.0,131.0,-0.6],[-3.0,128.2,0.6,3.0,131.0,0.7],floor];
        assert!(chair_return(entry,&walls,&npc).is_none());
        let ceiling=[-3.0,129.8,-3.0,3.0,130.0,3.0];
        assert!(chair_return(entry,&[floor,ceiling],&[]).is_none());
        let high=[-3.0,128.7,-3.0,3.0,128.8,3.0];
        assert!(chair_return(entry,&[high],&npc).is_none());
        assert!(chair_return([f64::NAN,0.0,0.0],&[],&[]).is_none());
    }
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
