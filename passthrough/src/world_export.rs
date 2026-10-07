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
struct ActorRef(Object);
impl Drop for ActorRef { fn drop(&mut self) { unsafe {
    if self.0.is_null() { return; }
    // TESObjectREFR's BSHandleRefObject base starts at 0x20.
    let base=self.0.add(0x20); let refs=&*base.add(8).cast::<AtomicU32>();
    if (refs.fetch_sub(1,Ordering::SeqCst).wrapping_sub(1)&0x3ff)==0 {
        let table:*const usize=read(base,0);
        let delete:unsafe extern "C" fn(Object)=mem::transmute(*table.add(1)); delete(base);
    }
} } }

#[derive(Default)]
pub struct Exporter {
    epoch:u64,sequence:u64,cell:usize,center:[f64;3],ray:usize,
    boxes:Vec<[f64;6]>,started:Option<Instant>,rays:usize,hits:usize,clipped:usize,
}
impl Exporter {
    pub fn reset(&mut self) { self.epoch=self.epoch.wrapping_add(1); self.cell=0; self.started=None; self.boxes.clear(); }
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
        if self.cell!=cell as usize { self.reset(); self.cell=cell as usize; }
        if self.started.is_none() {
            self.center=sample.mc.map(|x|(x*2.0).floor()/2.0); self.center[1]+=1.0;
            self.ray=0; self.rays=0; self.hits=0; self.clipped=0; self.boxes.clear(); self.started=Some(Instant::now());
        }
        // 16x16 rays for each of six faces. At most eight hits per ray.
        let budget=Instant::now(); let mut casts=0;
        {
            let lock_fn:unsafe extern "C" fn(Object)=unsafe { mem::transmute(api.lock) };
            unsafe { lock_fn(bhk.add(0xC598)); }
            let _guard=ReadLock{object:unsafe { bhk.add(0xC598) },unlock:api.unlock};
            let cast:unsafe extern "C" fn(Object,*const Input,*mut Output)=unsafe { mem::transmute(api.cast) };
            while self.ray<1536 && casts<384 && budget.elapsed().as_micros()<2000 {
                let axis=self.ray/512; let positive=(self.ray/256)%2==0;
                let u=(axis+1)%3; let v=(axis+2)%3; let index=self.ray%256;
                let mut a=self.center; let mut b=self.center;
                a[u]+=((index%16) as f64+0.5)*0.5-4.0; a[v]+=((index/16) as f64+0.5)*0.5-4.0;
                b[u]=a[u]; b[v]=a[v]; a[axis]+=if positive {-4.0} else {4.0}; b[axis]+=if positive {4.0} else {-4.0};
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
        if self.ray<1536 { return Ok(None); }
        let elapsed=self.started.take().unwrap().elapsed().as_millis();
        self.sequence+=1;
        let mut snapshot=Snapshot{recipient:sample.sender,world:sample.world,epoch:self.epoch,
            sequence:self.sequence,active:true,boxes:mem::take(&mut self.boxes),..Snapshot::default()};
        unsafe { self.npcs(api,player,cell,anchor,sample,&mut snapshot)?; }
        write_log(Role::Skyrim,format!("event=WORLD_SCAN epoch={} snapshot={} rays={} hits={} hit_limit_rays={} elapsed_ms={elapsed} radius_blocks=4 spacing=0.5",
            self.epoch,self.sequence,self.rays,self.hits,self.clipped))?;
        // Never refresh the timestamp on geometry that took too long to sample.
        if elapsed>1000 || (0..3).any(|j|(sample.mc[j]-self.center[j]).abs()>3.0) {
            snapshot.active=false; snapshot.boxes.clear(); snapshot.npcs.clear();
            write_log(Role::Skyrim,"event=WORLD_DISCARDED reason=scan_too_old_or_player_outside")?;
        }
        Ok(Some(snapshot))
    }
    unsafe fn npcs(&self,api:&Api,player:Object,cell:Object,anchor:Anchor,sample:Sample,s:&mut Snapshot)->io::Result<()> {
        let lists=unsafe { *(api.processes as *const Object) };
        if lists.is_null() { return Ok(()); }
        let data:*const u32=unsafe { read(lists,0x30) }; let count:u32=unsafe { read(lists,0x40) };
        if count>16384 || (data.is_null() && count!=0) { return Err(io::Error::other("invalid high actor list")); }
        let lookup:unsafe extern "C" fn(*const u32,*mut Object)->bool=unsafe { mem::transmute(api.lookup) };
        for i in 0..count as usize {
            let mut actor=ActorRef(ptr::null_mut()); unsafe { lookup(data.add(i),&mut actor.0); }
            if actor.0.is_null() || actor.0==player || unsafe { read::<Object>(actor.0,0x60) }!=cell { continue; }
            let sky:[f32;3]=unsafe { read(actor.0,0x54) }; let p=anchor.minecraft(sky.map(f64::from));
            if p.iter().any(|x|!x.is_finite()) || (0..3).any(|j|(p[j]-sample.mc[j]).abs()>8.0) { continue; }
            if s.npcs.len()==MAX_NPCS { return Err(io::Error::other("NPC snapshot overflow")); }
            s.npcs.push((unsafe { read::<u32>(actor.0,0x14) } as u64,p));
        }
        Ok(())
    }
}
