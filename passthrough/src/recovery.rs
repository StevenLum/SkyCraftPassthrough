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
