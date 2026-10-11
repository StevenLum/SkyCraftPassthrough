//! Short-lived, target-bound basic punch requests, on the player update thread.
use std::{io,mem,ptr,sync::OnceLock};
use crate::{native::write_log,position::Sample,Role};
type Object=*mut u8;
/// `hostile` is Actor::IsHostileToActor; absent means no character is ever reported hostile.
struct Api {crosshair:usize,lookup:usize,damage:usize,hostile:Option<usize>}
static API:OnceLock<Api>=OnceLock::new();
pub fn configure(crosshair:usize,lookup:usize,damage:usize,hostile:Option<usize>) {let _=API.set(Api{crosshair,lookup,damage,hostile});}
unsafe fn read<T:Copy>(p:Object,n:usize)->T {unsafe {ptr::read_unaligned(p.add(n).cast())}}
unsafe fn handle(api:&Api)->u32 {
    let pick=unsafe {*(api.crosshair as *const Object)};
    if pick.is_null() {0} else {unsafe {read(pick,4)}}
}
/// The kind of thing a crosshair reference is, from its record types alone.
/// `base_type` is the form type of the object the reference is an instance of.
/// `hostile` only matters for a living character: Skyrim's own E does nothing to an enemy.
fn classify(reference_type:u8,base_type:Option<u8>,dead:bool,hostile:bool)->u32 {
    use crate::world::*;
    // CommonLib FormTypes.h: 0x3E character reference, 0x1C container, 0x1D door.
    if reference_type==0x3E {return if dead {TARGET_BODY} else if hostile {TARGET_HOSTILE} else {TARGET_ALIVE};}
    match base_type {Some(0x1C)=>TARGET_CONTAINER,Some(0x1D)=>TARGET_DOOR,_=>TARGET_OTHER}
}
/// Dying or dead: Skyrim's life state (CommonLib ActorState, confirmed for the player in
/// 0.10.8), or no health left. `actor` must be a loaded character reference.
pub(crate) unsafe fn dead(actor:Object)->bool {
    let life=(unsafe {read::<u32>(actor,0xC8)}>>21)&0xF;
    let owner=unsafe {actor.add(0xB8)};let table:*const usize=unsafe {read(owner,0)};
    let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
    life==1 || life==2 || !(unsafe {get(owner,24)}>0.0)
}
/// Look the crosshair handle up (reference counted, released on drop) and classify it.
/// The second value says the target is a seat, which is reported as "nothing to use": E on
/// it would sit Skyrim's character down and leave it seated (see seat.rs).
unsafe fn kind(api:&Api,handle:u32,player:Object)->(u32,bool) {
    use crate::world::TARGET_NONE;
    if handle==0 || handle==u32::MAX {return (TARGET_NONE,false);}
    let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe {mem::transmute(api.lookup)};
    let mut target=crate::world_export::ActorRef(ptr::null_mut());
    if !unsafe {lookup(&handle,&mut target.0)} || target.0.is_null() {return (TARGET_NONE,false);}
    let reference_type:u8=unsafe {read(target.0,0x1A)};
    if reference_type==0x3E {
        let dead=unsafe {dead(target.0)};
        // CommonLib Actor.cpp: bool Actor::IsHostileToActor(Actor*), asked of the target about the player.
        let asked=crate::switches::on(Role::Skyrim,crate::switches::HOSTILITY);
        let hostile=asked && !dead && !player.is_null() && target.0!=player && api.hostile.is_some_and(|address| {
            let test:unsafe extern "C" fn(Object,Object)->bool=unsafe {mem::transmute(address)};
            unsafe {test(target.0,player)}
        });
        return (classify(reference_type,None,dead,hostile),false);
    }
    if unsafe {crate::seat::target_is_seat(target.0)} {
        if crate::skse::sitting_available() && unsafe {crate::seat::target_is_chair(target.0)} {
            return (crate::world::TARGET_OTHER,true);
        }
        if crate::switches::on(Role::Skyrim,crate::switches::SEAT_GUARD) {return (TARGET_NONE,true);}
    }
    // CommonLib TESObjectREFR.h: the base object pointer is the first member of data at 0x40.
    let base:Object=unsafe {read(target.0,0x40)};
    (classify(reference_type,(!base.is_null()).then(|| unsafe {read::<u8>(base,0x1A)}),false,false),false)
}
/// Handle value Minecraft sends for a shot (a released bow): there is no target yet, Skyrim
/// works out what the shot hits. An older bridge reads it as "no such target" and drops it.
pub const RANGED:u32=0xFFFF_FFFE;
/// A body, for a shot: 128 units tall (1.83 blocks), 32 units (0.46 block) to either side of
/// its middle line. The shooter's eye is 113.4 units (Minecraft's 1.62 blocks) above the feet.
const BODY_HEIGHT:f64=128.0;
const BODY_RADIUS:f64=32.0;
const EYE_HEIGHT:f64=113.4;
/// How far a shot reaches: 60 blocks.
const SHOT_RANGE:f64=4200.0;
/// A Minecraft view direction (degrees) as a unit vector in Skyrim's axes: x east, y north,
/// z up. Minecraft's own vector is (-sin yaw cos pitch, -sin pitch, cos yaw cos pitch) with
/// z south and y up; the bridge maps Minecraft (x, y, z) to Skyrim (x, -z, y).
pub(crate) fn look(yaw:f64,pitch:f64)->[f64;3] {
    let (yaw,pitch)=(yaw.to_radians(),pitch.to_radians());
    [-yaw.sin()*pitch.cos(),-yaw.cos()*pitch.cos(),-pitch.sin()]
}
/// How far along a ray (from `origin` in direction `dir`, a unit vector) it passes within a
/// body's width of the upright body standing on `feet`, if it does within `range`.
/// The closest approach of the ray and the body's middle line, both held to their ends.
pub(crate) fn ray_hits_body(origin:[f64;3],dir:[f64;3],feet:[f64;3],range:f64)->Option<f64> {
    let w=[origin[0]-feet[0],origin[1]-feet[1],origin[2]-feet[2]];
    let (b,d,e)=(dir[2],dir[0]*w[0]+dir[1]*w[1]+dir[2]*w[2],w[2]);
    let denominator=1.0-b*b;
    // Height on the body nearest to the endless line of the ray; straight up or down has none.
    let mut height=(if denominator>1e-9 {(e-b*d)/denominator} else {e}).clamp(0.0,BODY_HEIGHT);
    let mut along=0.0;
    // Each side in turn takes its nearest point to the other's; a few rounds settle it.
    for _ in 0..4 {
        along=(height*b-d).clamp(0.0,range);
        height=(e+along*b).clamp(0.0,BODY_HEIGHT);
    }
    let gap=[w[0]+along*dir[0],w[1]+along*dir[1],w[2]+along*dir[2]-height];
    let distance=(gap[0]*gap[0]+gap[1]*gap[1]+gap[2]*gap[2]).sqrt();
    (distance.is_finite() && distance<=BODY_RADIUS).then_some(along)
}
static SEEN:std::sync::Mutex<(u32,u32)>=std::sync::Mutex::new((0,0));
pub unsafe fn snapshot(s:&mut crate::world::Snapshot,player:Object) {
    if !(s.active && s.admitted) {return;}
    let Some(api)=API.get() else {return;};
    s.attack_handle=unsafe {handle(api)};
    let (kind,seat)=unsafe {kind(api,s.attack_handle,player)};
    s.target_kind=kind;
    if let Ok(mut seen)=SEEN.lock() {
        if *seen!=(s.attack_handle,s.target_kind) {
            *seen=(s.attack_handle,s.target_kind);
            let _=write_log(Role::Skyrim,format!("event=SCENE_CROSSHAIR handle={} kind={}{}",s.attack_handle,s.target_kind,if seat {" seat=true"} else {""}));
        }
    }
}
pub struct Attack {last:Option<(u64,u64,u64)>,blocked:bool,called:u64}
impl Default for Attack {fn default()->Self {Self{last:None,blocked:true,called:0}}}
impl Attack {
    pub fn block(&mut self) {self.blocked=true;}
    /// A released bow: the nearest living character on the line of sight takes the damage,
    /// unless a wall or the ground is in the way.
    unsafe fn shoot(&mut self,api:&Api,world:&crate::world_export::Api,player:Object,s:Sample,now:u64)->io::Result<()> {
        let feet:[f32;3]=unsafe {read(player,0x54)};
        let origin=[feet[0] as f64,feet[1] as f64,feet[2] as f64+EYE_HEIGHT];
        self.called=now;
        match unsafe {crate::world_export::shot(world,player,origin,look(s.yaw,s.pitch),SHOT_RANGE)}? {
            crate::world_export::Shot::Nothing=>write_log(Role::Skyrim,format!(
                "event=SCENE_SHOT_RECV sequence={} state=miss reason=nothing_on_the_line requested={}",s.attack_seq,s.attack_damage)),
            crate::world_export::Shot::Blocked(distance)=>write_log(Role::Skyrim,format!(
                "event=SCENE_SHOT_RECV sequence={} state=miss reason=wall_in_the_way distance={distance:.0} requested={}",s.attack_seq,s.attack_damage)),
            crate::world_export::Shot::Hit(target,distance)=>{
                let owner=unsafe {target.0.add(0xB8)};let table:*const usize=unsafe {read(owner,0)};
                let get:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
                let before=unsafe {get(owner,24)};
                let id:u32=unsafe {read(target.0,0x14)};
                write_log(Role::Skyrim,format!("event=SCENE_SHOT_RECV sequence={} target={id:08X} state=dispatch distance={distance:.0} requested={} before={before}",s.attack_seq,s.attack_damage))?;
                let damage:unsafe extern "C" fn(Object,f32,Object,bool)->bool=unsafe {mem::transmute(api.damage)};
                let result=unsafe {damage(target.0,s.attack_damage,player,false)};
                let after=unsafe {get(owner,24)};
                write_log(Role::Skyrim,format!("event=SCENE_SHOT_RESULT sequence={} target={id:08X} returned={result} before={before} after={after}",s.attack_seq))
            }
        }
    }
    pub unsafe fn apply(&mut self,player:Object,s:Sample,world:Option<&crate::world_export::Api>)->io::Result<()> {
        if s.attack_seq==0 {self.blocked=false;return Ok(());}
        let key=(s.sender,s.world,s.attack_seq);if self.last==Some(key) {return Ok(());}self.last=Some(key);
        let Some(api)=API.get() else {return Ok(());};let now=crate::windows::now_ms();
        let ranged=s.attack_handle==RANGED;
        let reason=if self.blocked {"resume_barrier"}
            else if s.attack_ms==0 || s.attack_ms>now || now-s.attack_ms>250 {"expired"}
            else if !s.attack_damage.is_finite() || s.attack_damage<=0.0 || s.attack_damage>1000.0 {"invalid_damage"}
            else if now.saturating_sub(self.called)<100 {"cooldown"}
            else if ranged {if world.is_some() && crate::switches::on(Role::Skyrim,crate::switches::RANGED) {"ready"} else {"shots_unavailable"}}
            else if s.attack_handle==0 || s.attack_handle==u32::MAX || unsafe {handle(api)}!=s.attack_handle {"target_changed"}
            else {"ready"};
        if reason!="ready" {return write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RECV sequence={} state=dropped reason={reason}",s.attack_seq));}
        if ranged {return unsafe {self.shoot(api,world.unwrap(),player,s,now)};}
        let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe {mem::transmute(api.lookup)};
        let mut target=crate::world_export::ActorRef(ptr::null_mut());
        let found=unsafe {lookup(&s.attack_handle,&mut target.0)};
        if !found || target.0.is_null() || target.0==player || unsafe {read::<u8>(target.0,0x1A)}!=0x3E {
            return write_log(Role::Skyrim,format!("event=SCENE_ATTACK_RECV sequence={} state=dropped reason=not_actor",s.attack_seq));
        }
        let position:[f32;3]=unsafe {read(player,0x54)};let other:[f32;3]=unsafe {read(target.0,0x54)};
        let distance=(0..3).map(|i|(position[i] as f64-other[i] as f64).powi(2)).sum::<f64>().sqrt();
        // The same cell, or the next outdoor cell of the same world: a cell border is not a wall.
        if !distance.is_finite() || distance>210.0 || !unsafe {crate::world_export::same_place(read::<Object>(player,0x60),read::<Object>(target.0,0x60))} {
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
    fn a_minecraft_view_direction_becomes_a_skyrim_direction() {
        let near=|a:[f64;3],b:[f64;3]| (0..3).all(|i|(a[i]-b[i]).abs()<1e-9);
        // Minecraft yaw 0 faces south (+z), which is Skyrim -y; -90 faces east; 180 north; 90 west.
        assert!(near(look(0.0,0.0),[0.0,-1.0,0.0]));
        assert!(near(look(-90.0,0.0),[1.0,0.0,0.0]));
        assert!(near(look(180.0,0.0),[0.0,1.0,0.0]));
        assert!(near(look(90.0,0.0),[-1.0,0.0,0.0]));
        // Minecraft pitch -90 is straight up, +90 straight down.
        assert!(near(look(37.0,-90.0),[0.0,0.0,1.0]));
        assert!(near(look(37.0,90.0),[0.0,0.0,-1.0]));
        // Always a unit vector.
        let v=look(123.0,-31.0);
        assert!(((v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt()-1.0).abs()<1e-12);
    }
    #[test]
    fn a_shot_hits_a_body_on_its_line_and_nothing_else() {
        let eye=[0.0,0.0,113.4];let east=[1.0,0.0,0.0];
        // Straight at a body 500 units east: hit, about 500 along.
        let along=ray_hits_body(eye,east,[500.0,0.0,0.0],4200.0).unwrap();
        assert!((along-500.0).abs()<1e-6);
        // 31 units to the side still hits, 33 does not.
        assert!(ray_hits_body(eye,east,[500.0,31.0,0.0],4200.0).is_some());
        assert!(ray_hits_body(eye,east,[500.0,33.0,0.0],4200.0).is_none());
        // A body standing 100 units higher is hit low on the legs. One standing 100 lower is
        // missed (the line passes 85 above its head), and so is one standing 150 higher (the
        // line passes 36.6 under its feet).
        assert!(ray_hits_body(eye,east,[500.0,0.0,100.0],4200.0).is_some());
        assert!(ray_hits_body(eye,east,[500.0,0.0,-100.0],4200.0).is_none());
        assert!(ray_hits_body(eye,east,[500.0,0.0,150.0],4200.0).is_none());
        // Behind the shooter, and beyond the reach of a shot.
        assert!(ray_hits_body(eye,east,[-500.0,0.0,0.0],4200.0).is_none());
        assert!(ray_hits_body(eye,east,[4300.0,0.0,0.0],4200.0).is_none());
        assert!(ray_hits_body(eye,east,[4100.0,0.0,0.0],4200.0).is_some());
        // Aiming down a slope at a body lower and further: 45 degrees down, 1000 away, 1000 lower.
        let down=look(-90.0,45.0);
        assert!(ray_hits_body(eye,down,[1000.0,0.0,-950.0],4200.0).is_some());
        assert!(ray_hits_body(eye,down,[1000.0,0.0,-700.0],4200.0).is_none());
        // Straight down onto a body right below, and straight up at nothing.
        assert!(ray_hits_body([0.0,0.0,500.0],[0.0,0.0,-1.0],[10.0,0.0,0.0],4200.0).is_some());
        assert!(ray_hits_body(eye,[0.0,0.0,1.0],[500.0,0.0,0.0],4200.0).is_none());
        // Of two bodies on the line the nearer one has the smaller distance.
        let (near,far)=(ray_hits_body(eye,east,[300.0,5.0,0.0],4200.0).unwrap(),ray_hits_body(eye,east,[900.0,-5.0,0.0],4200.0).unwrap());
        assert!(near<far);
    }
    #[test]
    fn crosshair_targets_are_classified_by_record_type() {
        assert_eq!(classify(0x3D,Some(0x1C),false,false),TARGET_CONTAINER);
        assert_eq!(classify(0x3D,Some(0x1D),false,false),TARGET_DOOR);
        assert_eq!(classify(0x3D,Some(0x18),false,false),TARGET_OTHER);
        assert_eq!(classify(0x3D,None,false,false),TARGET_OTHER);
        // A character is a body only once it is dead, whatever its base record says.
        assert_eq!(classify(0x3E,None,false,false),TARGET_ALIVE);
        assert_eq!(classify(0x3E,None,true,false),TARGET_BODY);
        assert_eq!(classify(0x3E,Some(0x1C),true,false),TARGET_BODY);
        // An enemy is its own kind while alive; its body is a body, and only characters can be hostile.
        assert_eq!(classify(0x3E,None,false,true),TARGET_HOSTILE);
        assert_eq!(classify(0x3E,None,true,true),TARGET_BODY);
        assert_eq!(classify(0x3D,Some(0x1C),false,true),TARGET_CONTAINER);
        for kind in [TARGET_NONE,TARGET_OTHER,TARGET_CONTAINER,TARGET_ALIVE,TARGET_BODY,TARGET_DOOR,TARGET_HOSTILE] {
            assert!(kind<=MAX_TARGET_KIND);
        }
    }
}
