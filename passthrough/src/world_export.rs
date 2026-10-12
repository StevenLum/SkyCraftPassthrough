//! SkyCraft's ray-sampling MVP, using published CommonLib 1.7.104 interfaces.
use std::{io,mem,ptr,sync::atomic::{AtomicU32,Ordering},time::Instant};
use crate::{position::{Anchor,Sample},world::{Snapshot,MAX_BOXES,MAX_NPCS},native::write_log,Role};
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,n:usize)->T { unsafe { ptr::read_unaligned(p.add(n).cast()) } }
pub struct Api { pub cell_world:usize,pub cast:usize,pub scale:usize,pub processes:usize,
    pub lookup:usize,pub lock:usize,pub unlock:usize }
#[repr(C,align(16))]
struct Input { from:[f32;4],to:[f32;4],collection:u32,filter:u32,pad:[u32;2] }
#[repr(C,align(16))]
struct Output { normal:[f32;4],fraction:f32,extra:i32,key:u32,pad:u32,
    keys:[u32;8],key_index:u32,pad44:u32,pad48:u64,root:Object,pad58:u64 }
struct ReadLock { object:Object,unlock:usize }
impl Drop for ReadLock { fn drop(&mut self) { unsafe {
    let f:unsafe extern "C" fn(Object)=mem::transmute(self.unlock); f(self.object);
} } }
/// Whether two cells are one place for the bridge: the same cell, or two outdoor cells of
/// the same world (CommonLib TESObjectCELL worldSpace at +0x128; an interior has none).
/// A character or an enemy one step over a cell border is still next to the player.
pub(crate) unsafe fn same_place(a:Object,b:Object)->bool {
    if a.is_null() || b.is_null() {return false;}
    same_world(a as usize,b as usize,unsafe {read::<usize>(a,0x128)},unsafe {read::<usize>(b,0x128)})
}
fn same_world(cell_a:usize,cell_b:usize,world_a:usize,world_b:usize)->bool {
    cell_a==cell_b || (world_a!=0 && world_a==world_b)
}
pub(crate) struct ActorRef(pub(crate) Object);
impl Drop for ActorRef { fn drop(&mut self) { unsafe {
    if self.0.is_null() { return; }
    // TESObjectREFR's BSHandleRefObject base starts at 0x20.
    let base=self.0.add(0x20); let refs=&*base.add(8).cast::<AtomicU32>();
    if (refs.fetch_sub(1,Ordering::SeqCst).wrapping_sub(1)&0x3ff)==0 {
        let table:*const usize=read(base,0);
        let delete:unsafe extern "C" fn(Object)=mem::transmute(*table.add(1)); delete(base);
    }
} } }

/// What a shot met first.
pub(crate) enum Shot {
    /// A living character, and how far along the line.
    Hit(ActorRef,f64),
    /// A character was on the line, but a wall or the ground is in front of it.
    Blocked(f64),
    Nothing,
}
/// Whether a wall, the ground or another solid part of the world lies between two points
/// (Skyrim units). The same ray, layers and lock the collision scan uses; things that are
/// not part of the world's solid shape (characters, loose clutter) are passed through.
unsafe fn blocked(api:&Api,cell:Object,from:[f64;3],to:[f64;3])->io::Result<bool> {
    let get_world:unsafe extern "C" fn(Object)->Object=unsafe { mem::transmute(api.cell_world) };
    let bhk=unsafe { get_world(cell) };
    if bhk.is_null() { return Ok(false); }
    let vt:*const usize=unsafe { read(bhk,0) };
    let get_hk:unsafe extern "C" fn(Object)->Object=unsafe { mem::transmute(*vt.add(0x27)) };
    let hk=unsafe { get_hk(bhk) };
    if hk.is_null() { return Ok(false); }
    let scale=unsafe { *(api.scale as *const f32) } as f64;
    if !scale.is_finite() || !(0.001..0.1).contains(&scale) { return Err(io::Error::other("invalid Havok world scale")); }
    let lock_fn:unsafe extern "C" fn(Object)=unsafe { mem::transmute(api.lock) };
    unsafe { lock_fn(bhk.add(0xC598)); }
    let _guard=ReadLock{object:unsafe { bhk.add(0xC598) },unlock:api.unlock};
    let cast:unsafe extern "C" fn(Object,*const Input,*mut Output)=unsafe { mem::transmute(api.cast) };
    let mut start=from.map(|v|v*scale); let end=to.map(|v|v*scale);
    for _ in 0..8 {
        let input=Input{from:[start[0] as f32,start[1] as f32,start[2] as f32,0.0],
            to:[end[0] as f32,end[1] as f32,end[2] as f32,0.0],collection:1,filter:30,pad:[0;2]};
        let mut out=Output{normal:[0.0;4],fraction:1.0,extra:-1,key:u32::MAX,pad:0,
            keys:[u32::MAX;8],key_index:0,pad44:0,pad48:0,root:ptr::null_mut(),pad58:0};
        unsafe { cast(hk,&input,&mut out); }
        if out.root.is_null() || !out.fraction.is_finite() || !(0.0..1.0).contains(&out.fraction) { return Ok(false); }
        let layer=unsafe { read::<u32>(out.root,0x2c) }&0x7f;
        if matches!(layer,1|2|3|9|10|13|17|27|31) { return Ok(true); }
        let hit:[f64;3]=std::array::from_fn(|j|start[j]+(end[j]-start[j])*out.fraction as f64);
        let distance=(0..3).map(|j|(end[j]-hit[j]).powi(2)).sum::<f64>().sqrt();
        let epsilon=0.5*scale;
        if distance<=epsilon { return Ok(false); }
        start=std::array::from_fn(|j|hit[j]+(end[j]-hit[j])*epsilon/distance);
    }
    Ok(false)
}
/// What a shot from `origin` along the unit vector `dir` (Skyrim units) meets first: the
/// nearest living character whose body the line passes through, among the characters Skyrim
/// is simulating around the player, unless something solid is in front of it.
pub(crate) unsafe fn shot(api:&Api,player:Object,origin:[f64;3],dir:[f64;3],range:f64)->io::Result<Shot> {
    let cell:Object=unsafe { read(player,0x60) };
    let lists=unsafe { *(api.processes as *const Object) };
    if cell.is_null() || lists.is_null() { return Ok(Shot::Nothing); }
    let data:*const u32=unsafe { read(lists,0x30) }; let count:u32=unsafe { read(lists,0x40) };
    if count>16384 || (data.is_null() && count!=0) { return Err(io::Error::other("invalid high actor list")); }
    let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe { mem::transmute(api.lookup) };
    let mut best:Option<(ActorRef,f64)>=None;
    for i in 0..count as usize {
        let mut actor=ActorRef(ptr::null_mut()); unsafe { lookup(data.add(i),&mut actor.0); }
        if actor.0.is_null() || actor.0==player || !unsafe { same_place(cell,read::<Object>(actor.0,0x60)) } { continue; }
        let feet:[f32;3]=unsafe { read(actor.0,0x54) };
        let Some(along)=crate::attack::ray_hits_body(origin,dir,feet.map(f64::from),range) else { continue; };
        if best.as_ref().is_some_and(|(_,nearest)|*nearest<=along) || unsafe { crate::attack::dead(actor.0) } { continue; }
        best=Some((actor,along));
    }
    let Some((actor,along))=best else { return Ok(Shot::Nothing); };
    let to:[f64;3]=std::array::from_fn(|j|origin[j]+dir[j]*along);
    Ok(if unsafe { blocked(api,cell,origin,to)? } {Shot::Blocked(along)} else {Shot::Hit(actor,along)})
}
#[derive(Default)]
pub struct Exporter {
    /// `world` is Skyrim's physics world being scanned (its two object addresses). Outdoor
    /// cells of one world share it, so walking into the next cell does not restart a scan.
    epoch:u64,sequence:u64,world:(usize,usize),center:[f64;3],ray:usize,
    boxes:Vec<[f64;6]>,started:Option<Instant>,rays:usize,hits:usize,clipped:usize,
    previous:Option<Sample>,velocity:[f64;3],lead:[f64;3],horizon:f64,last_scan_ms:f64,
    location:Option<(u32,u32)>,
    /// Time spent casting rays in the current scan, and the frames it has taken so far.
    cast_us:u64,scan_frames:u32,
    /// The last finished scan: its boxes, their centre and checksum. `calm` says the scan
    /// before it had the same centre and the same boxes; `slow` that the scan now running is
    /// paced down because of that.
    done:Vec<[f64;6]>,done_center:[f64;3],done_sum:u64,calm:bool,slow:bool,
    /// When a snapshot last went out, and when this function last ran.
    published_ms:u64,last_frame:Option<Instant>,
    radius_y:f64,done_radius_y:f64,fast:bool,
}
/// Ray casts allowed in one frame at full rate.
const CASTS_PER_FRAME:usize=384;
/// A paced-down scan sends the last finished boxes again this often, so the characters,
/// health, hits and crosshair that travel with them are as fresh as with full-rate scans.
const RESEND_MS:u64=125;
/// Ray casts allowed in this frame. A calm scan (the player in the same spot and the same
/// boxes as the scan before) is spread over about four times as many frames; over fewer
/// when frames are long, so that it still finishes in about half a second.
fn casts_per_frame(calm:bool,frame_ms:u64)->usize {
    if !calm {CASTS_PER_FRAME} else if frame_ms<=20 {CASTS_PER_FRAME/4} else if frame_ms<=40 {CASTS_PER_FRAME/2} else {CASTS_PER_FRAME}
}
/// Whether the last finished boxes go out again now: only while a calm scan is running.
fn resend_due(calm:bool,since_last_ms:u64)->bool {calm && since_last_ms>=RESEND_MS}
fn scan_budget(fast:bool,calm:bool,frame_ms:u64)->usize {
    if fast && !calm {1536} else {casts_per_frame(calm,frame_ms)}
}
/// How far ahead of the player a scan may be centred, sideways and up or down. The cube
/// reaches 4 blocks from its centre and the player needs 0.8 of room on every side, and a
/// scan whose centre ends more than 3 blocks from the player is thrown away; these leave
/// room for both. Until 0.10.41 they were 1.5 and 0.75, which sprinting already reached, so
/// anything faster (sprint-jumping, a speed effect, flying, a long fall) outran the cube.
const LEAD_SIDE:f64=2.75;
const LEAD_UP:f64=1.75;
/// Up or down the full distance is only for speeds no jump reaches. A jump turns round
/// within a third of a second (up at 8.4 blocks a second, down at 8.9 when it lands, where
/// the ground ends the fall), so a cube aimed far along it is in the wrong place when it
/// arrives. 0.10.41 used 1.75 for every speed, and Minecraft held the player at the start
/// of the next jump: the cube had been aimed down for a fall that was already over (report
/// 20261011-184225, seven times). Up to `JUMP_SPEED` the limit is the 0.75 that was in use
/// until then; from there it grows evenly to the full distance at `FALL_SPEED`, which a
/// fall of four blocks reaches.
const LEAD_UP_JUMP:f64=0.75;
const JUMP_SPEED:f64=13.0;
const FALL_SPEED:f64=19.0;
/// The limit up or down for a speed on the height axis, in blocks a second.
fn lead_up(speed:f64)->f64 {
    if !speed.is_finite() { return LEAD_UP_JUMP; }
    (LEAD_UP_JUMP+(speed.abs()-JUMP_SPEED)/(FALL_SPEED-JUMP_SPEED)*(LEAD_UP-LEAD_UP_JUMP)).clamp(LEAD_UP_JUMP,LEAD_UP)
}
/// Where a scan starting now is centred: ahead of the player by its speed times the time a
/// scan takes (within the limits above), on the half-block grid, one block above the feet.
/// Returns the look-ahead in seconds, the lead and the centre.
fn aim(last_scan_ms:f64,velocity:[f64;3],mc:[f64;3])->(f64,[f64;3],[f64;3]) {
    let horizon=(last_scan_ms/1000.0+0.15).clamp(0.2,0.5);
    let mut lead=velocity.map(|v|v*horizon);
    let horizontal=lead[0].hypot(lead[2]);
    if horizontal>LEAD_SIDE { lead[0]*=LEAD_SIDE/horizontal; lead[2]*=LEAD_SIDE/horizontal; }
    let limit=lead_up(velocity[1]);
    lead[1]=lead[1].clamp(-limit,limit);
    let mut center:[f64;3]=std::array::from_fn(|j|((mc[j]+lead[j])*2.0).floor()/2.0);
    center[1]+=1.0;
    (horizon,lead,center)
}
/// Keep half-block face spacing when the vertical scan is taller; no widened holes.
fn ray_count(radius_y:f64)->usize {let height=(radius_y*4.0) as usize;2*(256+32*height)}
fn face_ray(mut index:usize,center:[f64;3],radius_y:f64)->([f64;3],[f64;3]) {
    let radii=[4.0,radius_y,4.0];let cells=[16,(radius_y*4.0) as usize,16];
    for axis in 0..3 {
        let (u,v)=((axis+1)%3,(axis+2)%3);let count=cells[u]*cells[v];
        if index>=2*count {index-=2*count;continue;}
        let positive=index<count;let tile=index%count;
        let mut a=center;let mut b=center;
        a[u]+=((tile%cells[u]) as f64+0.5)*0.5-radii[u];
        a[v]+=((tile/cells[u]) as f64+0.5)*0.5-radii[v];
        b[u]=a[u];b[v]=a[v];
        a[axis]+=if positive {-radii[axis]} else {radii[axis]};
        b[axis]+=if positive {radii[axis]} else {-radii[axis]};
        return (a,b);
    }
    unreachable!("face index outside the scan")
}
/// Per-render-frame position differences occasionally spike or vanish. Average over at
/// least a Minecraft tick, retaining the last estimate between those observations.
fn velocity(previous:&mut Option<Sample>,current:Sample,old:[f64;3])->[f64;3] {
    let Some(before)=*previous else {*previous=Some(current);return [0.0;3];};
    if before.frame==current.frame {return old;}
    let dt=current.sent_ms.saturating_sub(before.sent_ms);
    if current.sender!=before.sender || current.world!=before.world || current.frame<before.frame
        || current.sent_ms<before.sent_ms || dt>250 {
        *previous=Some(current);return [0.0;3];
    }
    if dt<50 {return old;}
    *previous=Some(current);
    let v=std::array::from_fn(|j|(current.mc[j]-before.mc[j])*1000.0/dt as f64);
    if v.iter().all(|n|n.is_finite()) && v[0].hypot(v[2])<=64.0 && v[1].abs()<=100.0 {v} else {[0.0;3]}
}
fn scan_aim(last_ms:f64,v:[f64;3],sample:Sample,enabled:bool)->(f64,[f64;3],[f64;3],f64,bool) {
    let (horizon,mut lead,mut center)=aim(last_ms,v,sample.mc);
    if !enabled {return (horizon,lead,center,4.0,false);}
    let flying=sample.controls&crate::position::CONTROL_FLY!=0;
    let tall=flying || v[1].abs()>JUMP_SPEED;
    let radius_y=if !tall {4.0} else if v[1].abs()>40.0 {24.0} else {12.0};
    if tall {
        lead[1]=(v[1]*horizon).clamp(-(radius_y-3.0),radius_y-3.0);
        center[1]=((sample.mc[1]+lead[1])*2.0).floor()/2.0+1.0;
    }
    (horizon,lead,center,radius_y,tall || v[0].hypot(v[2])>8.0)
}
impl Exporter {
    pub fn reset(&mut self) { self.epoch=self.epoch.wrapping_add(1); self.world=(0,0); self.started=None; self.boxes.clear();
        self.location=None; self.previous=None; self.velocity=[0.0;3]; self.lead=[0.0;3]; self.last_scan_ms=0.0;
        self.done.clear(); self.done_sum=0; self.calm=false; self.slow=false; }
    pub fn empty(&mut self)->Snapshot {
        self.sequence+=1; Snapshot{epoch:self.epoch,sequence:self.sequence,..Snapshot::default()}
    }
    pub unsafe fn frame(&mut self,api:&Api,player:Object,anchor:Anchor,sample:Sample)->io::Result<Option<Snapshot>> {
        let cell:Object=unsafe { read(player,0x60) };
        if cell.is_null() { return Ok(None); }
        let get_world:unsafe extern "C" fn(Object)->Object=unsafe { mem::transmute(api.cell_world) };
        let bhk=unsafe { get_world(cell) };
        if bhk.is_null() { return Ok(None); }
        let vt:*const usize=unsafe { read(bhk,0) };
        let get_hk:unsafe extern "C" fn(Object)->Object=unsafe { mem::transmute(*vt.add(0x27)) };
        let hk=unsafe { get_hk(bhk) };
        if hk.is_null() { return Ok(None); }
        let scale=unsafe { *(api.scale as *const f32) } as f64;
        if !scale.is_finite() || !(0.001..0.1).contains(&scale) { return Err(io::Error::other("invalid Havok world scale")); }
        if self.world!=(bhk as usize,hk as usize) { self.reset(); self.world=(bhk as usize,hk as usize); }
        self.velocity=velocity(&mut self.previous,sample,self.velocity);
        let frame_ms=self.last_frame.replace(Instant::now()).map_or(0,|at|at.elapsed().as_millis() as u64);
        let (horizon,lead,center,radius_y,fast)=scan_aim(self.last_scan_ms,self.velocity,sample,crate::switches::on(Role::Skyrim,crate::switches::FAST_SCAN));
        if self.started.is_some() && self.slow && (center!=self.center || radius_y!=self.radius_y) {
            // The player moved on: this paced-down scan is of the wrong spot. Start over at full rate.
            self.started=None; self.calm=false;
        }
        if self.started.is_none() {
            self.horizon=horizon; self.lead=lead; self.center=center; self.radius_y=radius_y; self.fast=fast;
            // Same spot and same boxes twice in a row: nothing is happening here, scan slowly.
            self.slow=self.calm && center==self.done_center && radius_y==self.done_radius_y && crate::switches::on(Role::Skyrim,crate::switches::SLOW_SCAN);
            self.ray=0; self.rays=0; self.hits=0; self.clipped=0; self.boxes.clear(); self.started=Some(Instant::now());
            self.cast_us=0; self.scan_frames=0;
        }
        // Half-block grid on every face, including tall faces. At most eight hits per ray.
        let budget=Instant::now(); let mut casts=0; let allowed=scan_budget(self.fast,self.slow,frame_ms);
        let total=ray_count(self.radius_y);
        {
            let lock_fn:unsafe extern "C" fn(Object)=unsafe { mem::transmute(api.lock) };
            unsafe { lock_fn(bhk.add(0xC598)); }
            let _guard=ReadLock{object:unsafe { bhk.add(0xC598) },unlock:api.unlock};
            let cast:unsafe extern "C" fn(Object,*const Input,*mut Output)=unsafe { mem::transmute(api.cast) };
            while self.ray<total && casts<allowed && budget.elapsed().as_micros()<2000 {
                let (a,b)=face_ray(self.ray,self.center,self.radius_y);
                let to_sky=|p:[f64;3]|anchor.target(Sample{mc:p,..sample}).map(|p|p.map(|x|x as f64*scale));
                let mut from=to_sky(a).ok_or_else(||io::Error::other("invalid ray origin"))?;
                let to=to_sky(b).ok_or_else(||io::Error::other("invalid ray end"))?;
                for depth in 0..8 {
                    let input=Input{from:[from[0] as f32,from[1] as f32,from[2] as f32,0.0],
                        to:[to[0] as f32,to[1] as f32,to[2] as f32,0.0],collection:1,filter:30,pad:[0;2]};
                    let mut out=Output{normal:[0.0;4],fraction:1.0,extra:-1,key:u32::MAX,pad:0,
                        keys:[u32::MAX;8],key_index:0,pad44:0,pad48:0,root:ptr::null_mut(),pad58:0};
                    unsafe { cast(hk,&input,&mut out); } casts+=1; self.rays+=1;
                    if out.root.is_null() || !out.fraction.is_finite() || !(0.0..1.0).contains(&out.fraction) { break; }
                    let hit:[f64;3]=std::array::from_fn(|j|from[j]+(to[j]-from[j])*out.fraction as f64);
                    let layer=unsafe { read::<u32>(out.root,0x2c) }&0x7f;
                    if matches!(layer,1|2|3|9|10|13|17|27|31) {
                        self.hits+=1;
                        let p=anchor.minecraft(hit.map(|x|x/scale));
                        let normal=[out.normal[0],out.normal[2],-out.normal[1]];
                        let n=(0..3).max_by(|a,b|normal[*a].abs().total_cmp(&normal[*b].abs())).unwrap();
                        // Thin tile behind the hit surface, half-block wide tangentially.
                        let mut lo=p.map(|x|x-0.25); let mut hi=p.map(|x|x+0.25);
                        if normal[n]>=0.0 { lo[n]=p[n]-0.125; hi[n]=p[n]; }
                        else { lo[n]=p[n]; hi[n]=p[n]+0.125; }
                        if self.boxes.len()>=MAX_BOXES { return Err(io::Error::other("collision snapshot overflow; refusing incomplete geometry")); }
                        self.boxes.push([lo[0],lo[1],lo[2],hi[0],hi[1],hi[2]]);
                    }
                    if depth==7 { self.clipped+=1; }
                    let distance=(0..3).map(|j|(to[j]-hit[j]).powi(2)).sum::<f64>().sqrt();
                    let epsilon=0.5*scale;
                    if distance<=epsilon { break; }
                    from=std::array::from_fn(|j|hit[j]+(to[j]-hit[j])*epsilon/distance);
                }
                self.ray+=1;
            }
        }
        self.cast_us+=budget.elapsed().as_micros() as u64; self.scan_frames+=1;
        let now_ms=crate::windows::now_ms();
        // `finished` is how long the scan took when this frame completed one; otherwise the
        // last finished boxes are sent again, or nothing is sent.
        let (boxes,finished)=if self.ray<total {
            if !resend_due(self.slow,now_ms.saturating_sub(self.published_ms)) { return Ok(None); }
            (self.done.clone(),None)
        } else {
            let elapsed=self.started.take().unwrap().elapsed().as_millis();
            // A paced-down scan is slow on purpose; it must not stretch the look-ahead.
            if !self.slow { self.last_scan_ms=elapsed as f64; }
            let boxes=mem::take(&mut self.boxes);
            let sum=crate::world::boxes_checksum(&boxes);
            self.calm=self.center==self.done_center && self.radius_y==self.done_radius_y && sum==self.done_sum;
            self.done=boxes.clone(); self.done_center=self.center; self.done_radius_y=self.radius_y; self.done_sum=sum;
            (boxes,Some(elapsed))
        };
        self.published_ms=now_ms;
        self.sequence+=1;
        let mut snapshot=Snapshot{recipient:sample.sender,world:sample.world,epoch:self.epoch,
            sequence:self.sequence,active:true,center:self.done_center,radius:4.0,radius_y:self.done_radius_y,boxes,..Snapshot::default()};
        // CommonLib TESForm::formID +0x14; TESObjectCELL worldSpace +0x128
        // for AE >=1.6.629, including the supported 1.7.104 runtime.
        snapshot.admitted=true;
        // CommonLib AsActorValueOwner: +0xB8 since 1.6.629, including 1.7.104; slot 1.
        let owner=unsafe {player.add(0xB8)};
        let table:*const usize=unsafe {read(owner,0)};
        let get_health:unsafe extern "C" fn(Object,u32)->f32=unsafe {mem::transmute(*table.add(1))};
        snapshot.player_health=unsafe {get_health(owner,24)};
        snapshot.save_id=crate::save_identity::published_id();
        snapshot.cell_id=unsafe { read(cell,0x14) };
        let worldspace:Object=unsafe { read(cell,0x128) };
        snapshot.worldspace_id=if worldspace.is_null() {0} else {unsafe { read(worldspace,0x14) }};
        let location=(snapshot.cell_id,snapshot.worldspace_id);
        if self.location!=Some(location) {
            write_log(Role::Skyrim,format!("event=SCENE_LOCATION_SEND epoch={} snapshot={} cell_id={:08X} worldspace_id={:08X}",
                self.epoch,self.sequence,location.0,location.1))?;
            self.location=Some(location);
        }
        let bodies=unsafe { self.npcs(api,player,cell,anchor,sample,&mut snapshot)? };
        let outside=(0..3).any(|j|(sample.mc[j]-snapshot.center[j]).abs()>if j==1 {snapshot.radius_y-1.0} else {3.0});
        let Some(elapsed)=finished else {
            if outside {
                snapshot.active=false; snapshot.boxes.clear(); snapshot.npcs.clear();
                write_log(Role::Skyrim,"event=WORLD_DISCARDED reason=player_outside_resent_boxes")?;
            }
            write_log(Role::Skyrim,format!("event=SCENE_WORLD_RESEND epoch={} snapshot={} active={} boxes={} characters={} bodies_left_out={bodies}",
                self.epoch,self.sequence,snapshot.active,snapshot.boxes.len(),snapshot.npcs.len()))?;
            return Ok(Some(snapshot));
        };
        write_log(Role::Skyrim,format!("event=WORLD_SCAN epoch={} snapshot={} rays={} hits={} hit_limit_rays={} elapsed_ms={elapsed} radius_blocks=4 radius_y={} spacing=0.5",
            self.epoch,self.sequence,self.rays,self.hits,self.clipped,self.radius_y))?;
        // Never refresh the timestamp on geometry that took too long to sample. A paced-down
        // scan takes long on purpose and only runs while nothing around the player changes.
        if (elapsed>1000 && !self.slow) || outside {
            snapshot.active=false; snapshot.boxes.clear(); snapshot.npcs.clear();
            write_log(Role::Skyrim,"event=WORLD_DISCARDED reason=scan_too_old_or_player_outside")?;
        }
        write_log(Role::Skyrim,format!("event=SCENE_WORLD_SCAN epoch={} snapshot={} active={} pace={} elapsed_ms={elapsed} cast_us={} scan_frames={} rays={} boxes={} characters={} bodies_left_out={bodies} center={:?} lead={:?} horizon_ms={} radius=4 radius_y={} fast={} velocity_end={:?}",
            self.epoch,self.sequence,snapshot.active,if self.slow {"calm"} else {"full"},self.cast_us,self.scan_frames,self.rays,snapshot.boxes.len(),snapshot.npcs.len(),self.center,self.lead,self.horizon*1000.0,self.radius_y,self.fast,self.velocity))?;
        Ok(Some(snapshot))
    }
    /// Lists the living characters near the player as stand-ins. A dead one is left out and
    /// counted: a body on the ground must not be an invisible pillar.
    pub(crate) unsafe fn npcs(&self,api:&Api,player:Object,cell:Object,anchor:Anchor,sample:Sample,s:&mut Snapshot)->io::Result<usize> {
        let lists=unsafe { *(api.processes as *const Object) };
        if lists.is_null() { return Ok(0); }
        let mut bodies=0;
        let data:*const u32=unsafe { read(lists,0x30) }; let count:u32=unsafe { read(lists,0x40) };
        if count>16384 || (data.is_null() && count!=0) { return Err(io::Error::other("invalid high actor list")); }
        let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe { mem::transmute(api.lookup) };
        for i in 0..count as usize {
            let mut actor=ActorRef(ptr::null_mut()); unsafe { lookup(data.add(i),&mut actor.0); }
            if actor.0.is_null() || actor.0==player || !unsafe { same_place(cell,read::<Object>(actor.0,0x60)) } { continue; }
            let sky:[f32;3]=unsafe { read(actor.0,0x54) }; let p=anchor.minecraft(sky.map(f64::from));
            if p.iter().any(|x|!x.is_finite()) || (0..3).any(|j|(p[j]-sample.mc[j]).abs()>8.0) { continue; }
            if unsafe { crate::attack::dead(actor.0) } { bodies+=1; continue; }
            if s.npcs.len()==MAX_NPCS { return Err(io::Error::other("NPC snapshot overflow")); }
            s.npcs.push((unsafe { read::<u32>(actor.0,0x14) } as u64,p));
        }
        Ok(bodies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_calm_scan_is_spread_over_four_times_the_frames_and_resends_meanwhile() {
        // Moving or changing surroundings: full rate whatever the frame time.
        for frame_ms in [0,8,16,33,100] {assert_eq!(casts_per_frame(false,frame_ms),384);}
        // Calm at 60 or more frames a second: a quarter. Slower frames get a larger share,
        // so a scan of about 2,700 casts still takes about half a second.
        assert_eq!(casts_per_frame(true,16),96);
        assert_eq!(casts_per_frame(true,20),96);
        assert_eq!(casts_per_frame(true,33),192);
        assert_eq!(casts_per_frame(true,50),384);
        // Creative standing still must retain the quiet pacing despite tall coverage.
        assert_eq!(scan_budget(true,true,16),96);
        assert_eq!(scan_budget(true,false,16),1536);
        for (frame_ms,frames) in [(16_u64,2700/96+1),(33,2700/192+1)] {assert!(frame_ms*frames as u64<=500,"{frame_ms}");}
        // The last boxes go out again every 125 ms, and only during a calm scan.
        assert!(!resend_due(true,124));
        assert!(resend_due(true,125));
        assert!(!resend_due(false,10_000));
    }
    #[test]
    fn a_scan_is_centred_ahead_of_a_moving_player() {
        // Standing: the half-block cell of the feet, one block up. (0.10.2x logs: centre
        // [324.5, 40.5, -468.5] for a player at 324.9, 39.7, -468.2.)
        assert_eq!(aim(110.0,[0.0;3],[324.9456,39.6964,-468.1520]).2,[324.5,40.5,-468.5]);
        // Sprinting east at 5.6 blocks a second, a scan taking 0.12 s: 0.27 s ahead is 1.512
        // blocks (the old limit of 1.5 changed this by a hundredth of a block).
        let (horizon,lead,centre)=aim(120.0,[5.6,0.0,0.0],[10.2,64.0,0.2]);
        assert!((horizon-0.27).abs()<1e-9);
        assert!((lead[0]-1.512).abs()<1e-9 && lead[1]==0.0 && lead[2]==0.0);
        assert_eq!(centre,[11.5,65.0,0.0]);
        // Flying at 20 blocks a second (report 20261011-155446) and falling at 14: held to
        // the limits, diagonally too.
        assert!((aim(120.0,[20.0,0.0,0.0],[0.0;3]).1[0]-LEAD_SIDE).abs()<1e-9);
        let diagonal=aim(120.0,[-19.0,0.0,-7.3],[0.0;3]).1;
        assert!((diagonal[0].hypot(diagonal[2])-LEAD_SIDE).abs()<1e-9 && diagonal[0]<0.0 && diagonal[2]<0.0);
        // Up or down the limit follows the speed: 0.75 for anything a jump does, the full
        // 1.75 for a long fall or fast flight, evenly in between.
        for speed in [4.0,8.4,8.9,13.0] {
            assert_eq!(aim(120.0,[0.0,-speed,0.0],[0.2,37.2,0.2]).1[1],-LEAD_UP_JUMP,"{speed}");
            assert_eq!(aim(120.0,[0.0,speed,0.0],[0.2,37.2,0.2]).1[1],LEAD_UP_JUMP,"{speed}");
        }
        assert!((aim(120.0,[0.0,-16.0,0.0],[0.2,37.2,0.2]).1[1]+1.25).abs()<1e-9);
        for speed in [19.0,30.0,78.0] {
            assert_eq!(aim(120.0,[0.0,-speed,0.0],[0.2,37.2,0.2]).1[1],-LEAD_UP,"{speed}");
            assert_eq!(aim(120.0,[0.0,speed,0.0],[0.2,37.2,0.2]).1[1],LEAD_UP,"{speed}");
        }
        // A slow rise or fall is not stretched to the limit.
        assert!((aim(120.0,[0.0,-2.0,0.0],[0.2,37.2,0.2]).1[1]+0.54).abs()<1e-9);
        assert_eq!(lead_up(f64::NAN),LEAD_UP_JUMP);
        // Wherever the limits put the centre, the player (0.8 of room on each side, 1.8
        // tall plus half a block) is inside the cube and within 3 blocks of the centre.
        for velocity in [[20.0,0.0,0.0],[-20.0,0.0,0.0],[0.0,0.0,20.0],[14.0,0.0,-14.0],[0.0,-30.0,0.0],[0.0,30.0,0.0],[9.0,-9.0,9.0]] {
            for feet in [[0.0,0.0,0.0],[0.49,0.49,0.49],[-0.01,63.99,-0.01],[100.26,-12.74,7.51]] {
                let centre=aim(180.0,velocity,feet).2;
                for axis in [0,2] {
                    assert!((feet[axis]-centre[axis]).abs()<=3.0,"{velocity:?} {feet:?}");
                    assert!(feet[axis]-0.8>=centre[axis]-4.0 && feet[axis]+0.8<=centre[axis]+4.0);
                }
                assert!((feet[1]-centre[1]).abs()<=3.0,"{velocity:?} {feet:?}");
                assert!(feet[1]-0.5>=centre[1]-4.0 && feet[1]+2.3<=centre[1]+4.0,"{velocity:?} {feet:?}");
            }
        }
        // A very slow or very fast last scan keeps the look-ahead between 0.2 and 0.5 s.
        assert_eq!(aim(0.0,[0.0;3],[0.0;3]).0,0.2);
        assert_eq!(aim(900.0,[0.0;3],[0.0;3]).0,0.5);
    }
    /// Minecraft's own jump, one entry per game tick (a twentieth of a second): the height
    /// above the ground and the movement of the coming tick. Gravity 0.08 and drag 0.98 per
    /// tick, take-off at 0.42; holding the jump key jumps again in the tick of landing.
    fn jumps(ticks:usize)->Vec<(f64,f64)> {
        let (mut height,mut speed)=(0.0_f64,0.42_f64);
        (0..ticks).map(|_| {
            let now=(height,speed);
            height+=speed; speed=(speed-0.08)*0.98;
            if height<=0.0 { height=0.0; speed=0.42; }
            now
        }).collect()
    }
    /// The rule 0.10.41 broke. A cube is used for some time after the moment it was aimed
    /// at: the scan takes 0.1 to 0.2 s, and the next one as long again. Whatever moment of a
    /// run of jumps it was aimed at, it must still hold the player for the following 0.4 s:
    /// half a block under the feet, 2.3 above them, and the movement of that tick.
    #[test]
    fn a_cube_aimed_during_a_jump_still_holds_the_player_through_the_next_one() {
        let path=jumps(60);
        // The arithmetic is Minecraft's: 1.25 blocks high, landing at 8.9 blocks a second.
        let top=path.iter().map(|p|p.0).fold(0.0,f64::max);
        assert!((top-1.2522).abs()<0.001,"{top}");
        let landing=path.iter().map(|p|p.1*20.0).fold(0.0,f64::min);
        assert!((-9.0..=-8.8).contains(&landing),"{landing}");
        for scan_ms in [100.0,150.0,190.0,350.0] {
            for ground in [64.0,64.125,64.25,64.375,64.49,-12.74] {
                for aimed in 1..40 {
                    let velocity=[0.0,(path[aimed].0-path[aimed-1].0)*20.0,0.0];
                    let centre=aim(scan_ms,velocity,[0.3,ground+path[aimed].0,0.3]).2[1];
                    for used in aimed..=aimed+8 {
                        let (height,step)=path[used];
                        let feet=ground+height;
                        assert!(feet-0.5+step.min(0.0)>=centre-4.0 && feet+2.3+step.max(0.0)<=centre+4.0,
                            "scan {scan_ms} ms, ground {ground}, aimed at tick {aimed}, used at tick {used}: feet {feet}, step {step}, centre {centre}");
                    }
                }
            }
        }
        // The limit of 0.10.41 fails this at the landing: aimed 1.75 down, the top of the
        // cube is below the head of the next jump.
        let landing_tick=11;
        let low=((64.0+path[landing_tick].0-LEAD_UP)*2.0).floor()/2.0+1.0;
        assert!(64.0+path[landing_tick+4].0+2.3>low+4.0);
    }
    #[test]
    fn every_tall_face_is_sampled_on_the_same_half_block_grid() {
        for (radius_y,count) in [(4.0,1536),(12.0,3584),(24.0,6656)] {
            assert_eq!(ray_count(radius_y),count);
            let mut faces=[0;6];
            for i in 0..count {
                let (a,b)=face_ray(i,[0.0;3],radius_y);
                let axis=(0..3).find(|&j|a[j]!=b[j]).unwrap();
                let radii=[4.0,radius_y,4.0];
                assert_eq!(a[axis].abs(),radii[axis]);assert_eq!(b[axis],-a[axis]);
                faces[axis*2+usize::from(a[axis]>0.0)]+=1;
                for j in 0..3 {if j!=axis {
                    assert_eq!(a[j],b[j]);assert!(a[j].abs()<radii[j]);
                    assert_eq!(((a[j]+radii[j])*2.0).fract(),0.5);
                }}
            }
            assert_eq!(faces,[16*(radius_y*4.0) as usize,16*(radius_y*4.0) as usize,
                256,256,16*(radius_y*4.0) as usize,16*(radius_y*4.0) as usize]);
        }
    }
    #[test]
    fn flight_and_terminal_falls_fit_two_scan_intervals_and_the_next_tick() {
        // Empty fast scans: 100 ms is deliberately longer than the observed 67 ms normal
        // scans. Through two such intervals plus a 50 ms movement tick, at actual creative
        // ascent and a full vanilla terminal fall. Dense/slow scans remain a game test.
        for (v,controls) in [(7.5,crate::position::CONTROL_FLY),(-25.0,0),(-40.0,0),(-78.4,0)] {
            for feet in [64.0,64.49,-12.74] {
                let sample=Sample{mc:[0.0,feet,0.0],controls,..Sample::default()};
                let (_,_,center,radius,fast)=scan_aim(100.0,[0.0,v,0.0],sample,true);
                assert!(fast);
                for elapsed in [0.0,0.05,0.1,0.15,0.2] {
                    let at=feet+v*elapsed;let step=v*0.05;
                    assert!(at-0.5+step.min(0.0)>=center[1]-radius
                        && at+2.3+step.max(0.0)<=center[1]+radius,
                        "v={v} feet={feet} at={at} center={center:?} radius={radius}");
                }
                let old=scan_aim(100.0,[0.0,v,0.0],sample,false);
                assert_eq!(old.3,4.0);assert!(!old.4);
            }
        }
        // The observed fall at 25 blocks/sec cannot fit the old cube at this age.
        let old=aim(100.0,[0.0,-25.0,0.0],[0.0,64.0,0.0]).2;
        assert!(64.0-25.0*0.2-1.25-0.5<old[1]-4.0);
        // Ordinary jumps retain exactly the corrected aim and full face grid.
        for v in [-12.0,-8.9,0.0,8.4,13.0] {
            let sample=Sample{mc:[0.0,64.0,0.0],..Sample::default()};
            let new=scan_aim(100.0,[0.0,v,0.0],sample,true);
            assert_eq!((new.0,new.1,new.2),aim(100.0,[0.0,v,0.0],sample.mc));
            assert_eq!((new.3,new.4),(4.0,false));
        }
    }
    #[test]
    fn velocity_averages_render_jitter_and_accepts_a_real_long_fall() {
        let first=Sample{sender:1,world:2,frame:1,sent_ms:100,mc:[0.0,80.0,0.0],..Sample::default()};
        let mut previous=None;
        assert_eq!(velocity(&mut previous,first,[9.0;3]),[0.0;3]);
        let short=Sample{frame:2,sent_ms:108,mc:[0.0,78.0,0.0],..first};
        assert_eq!(velocity(&mut previous,short,[2.0;3]),[2.0;3]);
        let next=Sample{frame:3,sent_ms:150,mc:[1.0,76.08,0.0],..first};
        let v=velocity(&mut previous,next,[0.0;3]);
        assert!((v[0]-20.0).abs()<1e-9 && (v[1]+78.4).abs()<1e-9);
        assert_eq!(velocity(&mut previous,next,v),v);
        assert_eq!(velocity(&mut previous,Sample{sender:9,frame:4,sent_ms:200,..next},v),[0.0;3]);
        assert_eq!(velocity(&mut previous,Sample{frame:5,sent_ms:600,..next},v),[0.0;3]);
        assert_eq!(velocity(&mut previous,Sample{frame:6,sent_ms:500,..next},v),[0.0;3]);
    }
    #[test]
    fn a_cell_border_outdoors_is_not_a_wall() {
        // The same cell, indoors or out.
        assert!(same_world(0x1000,0x1000,0,0));
        assert!(same_world(0x1000,0x1000,0x50,0x50));
        // Two outdoor cells of one world.
        assert!(same_world(0x1000,0x2000,0x50,0x50));
        // Two buildings (no world), a building and the street, two different worlds.
        assert!(!same_world(0x1000,0x2000,0,0));
        assert!(!same_world(0x1000,0x2000,0,0x50));
        assert!(!same_world(0x1000,0x2000,0x50,0x60));
    }
}
