//! Bounded textured-world snapshots. Runtime textures never touch the filesystem.
use std::{io,sync::Mutex};
use crate::{position::{Anchor,Sample},windows::{SharedMemory,now_ms},native::write_log,Role,read_u32,read_u64};
pub const HEADER:usize=128;
pub const ATLAS_MAX:usize=4096*4096*4;
pub const MESH_MAX:usize=262_140*24;
static ALIGNMENT:Mutex<Option<(Sample,Anchor)>>=Mutex::new(None);
pub fn align(value:Option<(Sample,Anchor)>) { if let Ok(mut a)=ALIGNMENT.lock() { *a=value; } }
pub fn alignment()->Option<(Sample,Anchor)> {
    let a=(*ALIGNMENT.lock().ok()?)?;
    (now_ms().saturating_sub(a.0.sent_ms)<=250).then_some(a)
}
/// Meshes are built around the Minecraft player (a 16-block cube). One further away than
/// this on any axis is left over from a place the player has just left through a door, a
/// load or a death; it must be skipped, not mapped with the new place's origin.
pub const MESH_REACH:f64=64.0;
/// Where to draw a mesh in Skyrim coordinates, or None when it belongs to another place.
pub fn mesh_origin(mesh:[f64;3],sample:Sample,anchor:Anchor)->Option<[f32;3]> {
    if !(0..3).all(|axis|(mesh[axis]-sample.mc[axis]).abs()<=MESH_REACH) {return None;}
    anchor.target(Sample{mc:mesh,..sample})
}
pub fn hash(bytes:&[u8])->u64 { bytes.iter().fold(0xcbf29ce484222325,|h,b|(h^u64::from(*b)).wrapping_mul(0x100000001b3)) }
pub struct Packet { pub bytes:Vec<u8> }
impl Packet {
    pub fn field(&self,at:usize)->u64 { read_u64(&self.bytes,at) }
    pub fn count(&self)->u32 { read_u32(&self.bytes,44) }
    pub fn cracks(&self)->u32 { read_u32(&self.bytes,96) }
    pub fn items(&self)->u32 { read_u32(&self.bytes,100) }
    pub fn item_count(&self)->u32 { read_u32(&self.bytes,104) }
    pub fn translucent(&self)->u32 { read_u32(&self.bytes,108) }
    pub fn models(&self)->u32 { read_u32(&self.bytes,112) }
    pub fn translucent_items(&self)->u32 { read_u32(&self.bytes,116) }
    pub fn fluid_opaque(&self)->u32 { read_u32(&self.bytes,120) }
    pub fn fluid_translucent(&self)->u32 { read_u32(&self.bytes,124) }
    pub fn height(&self)->u32 { read_u32(&self.bytes,52) }
    pub fn kind(&self)->u32 { read_u32(&self.bytes,40) }
    pub fn origin(&self)->[f64;3] { [72,80,88].map(|at|f64::from_bits(self.field(at))) }
    pub fn validate(&self)->io::Result<()> {
        let b=&self.bytes;
        if b.len()<HEADER || &b[..8]!=b"PTSCE012" || b.len()!=HEADER+read_u32(b,48) as usize { return Err(io::Error::other("scene header/length mismatch")); }
        let n=self.count() as usize; let h=self.height() as usize;
        let item_total=u64::from(self.items())+u64::from(self.translucent_items());
        let valid_items=self.item_count()<=64 && self.models()<=self.item_count()
            && self.items()%6==0 && self.translucent_items()%6==0
            && self.items()>=(self.item_count()-self.models())*6
            && item_total>=u64::from(self.item_count()*6)
            && item_total<=u64::from((self.item_count()-self.models())*6+self.models()*576)
            && self.translucent_items()<=self.translucent();
        let valid=match self.kind() {
            1=>n<=262_140 && n%3==0 && self.cracks()<=32*36 && self.cracks()%36==0 && valid_items && self.translucent()<=262_140 && self.translucent()%6==0 && (self.cracks()+self.items()+self.translucent()) as usize<=n && b.len()==HEADER+n*24 && self.origin().iter().all(|x|x.is_finite() && x.abs()<30_000_001.0),
            2=>n>0 && h>0 && n<=4096 && h<=4096 && self.cracks()==0 && self.items()==0 && self.item_count()==0 && self.translucent()==0 && self.models()==0 && self.translucent_items()==0 && b.len()==HEADER+n*h*4,
            _=>false,
        };
        if !valid { return Err(io::Error::other("scene counts out of bounds")); }
        if self.fluid_opaque()%6!=0 || self.fluid_translucent()%6!=0 { return Err(io::Error::other("invalid fluid quad counts")); }
        if self.kind()==2 && (self.fluid_opaque()!=0 || self.fluid_translucent()!=0) { return Err(io::Error::other("fluid counts on atlas")); }
        if self.kind()==1 {
            if self.fluid_opaque()>self.count()-self.cracks()-self.translucent()-self.items()
                || self.fluid_translucent()>self.translucent()-self.translucent_items() {
                return Err(io::Error::other("fluid counts exceed geometry subsets"));
            }
            for v in b[HEADER..].chunks_exact(24) { for at in [0,4,8,12,16] {
                let x=f32::from_le_bytes(v[at..at+4].try_into().unwrap());
                if !x.is_finite() || (at<12 && x.abs()>64.0) { return Err(io::Error::other("invalid scene vertex")); }
            } }
        }
        Ok(())
    }
    pub fn log(&self,role:Role,direction:&str)->io::Result<()> {
        write_log(role,format!("event=SCENE_{direction} kind={} sender={} world={} snapshot={} atlas={} count={} height={} bytes={} checksum={} origin_x={} origin_y={} origin_z={} crack_vertices={} item_vertices={} items={} translucent_vertices={} models={} translucent_item_vertices={} fluid_opaque_vertices={} fluid_translucent_vertices={}",
            self.kind(),self.field(8),self.field(16),self.field(24),self.field(64),self.count(),self.height(),self.bytes.len()-HEADER,self.field(56),self.origin()[0],self.origin()[1],self.origin()[2],self.cracks(),self.items(),self.item_count(),self.translucent(),self.models(),self.translucent_items(),self.fluid_opaque(),self.fluid_translucent()))
    }
}
pub struct Mailbox { memory:SharedMemory,limit:usize }
impl Mailbox {
    pub fn open(atlas:bool)->io::Result<Self> {
        let limit=if atlas { ATLAS_MAX } else { MESH_MAX };
        let suffix=if atlas { "atlas" } else { "mesh" };
        Ok(Self{memory:SharedMemory::open_sized(&format!("{}_{}",crate::overlay::name(),suffix),HEADER+limit)?,limit})
    }
    pub fn send(&self,mut p:Packet,sender:u64)->io::Result<()> {
        p.validate()?;
        if p.bytes.len()>HEADER+self.limit { return Err(io::Error::other("wrong scene mailbox")); }
        p.bytes[8..16].copy_from_slice(&sender.to_le_bytes());
        p.bytes[32..40].copy_from_slice(&now_ms().to_le_bytes());
        let checksum=hash(&p.bytes[HEADER..]); p.bytes[56..64].copy_from_slice(&checksum.to_le_bytes());
        self.memory.blob(&mut p.bytes,true)?;
        p.log(Role::Minecraft,"SEND")
    }
    pub fn receive(&self,last:Option<(u64,u64)>)->io::Result<Option<Packet>> {
        let Some((permit,_))=alignment() else { return Ok(None); };
        let b=self.memory.read_scene(self.limit,last)?;
        if b.is_empty() { return Ok(None); }
        let p=Packet{bytes:b}; p.validate()?;
        if p.field(8)!=permit.sender || p.field(16)!=permit.world { return Ok(None); }
        if p.field(32)>now_ms() || (p.kind()==1 && now_ms().saturating_sub(p.field(32))>2000) { return Ok(None); }
        if hash(&p.bytes[HEADER..])!=p.field(56) { return Err(io::Error::other("scene checksum mismatch")); }
        p.log(Role::Skyrim,"RECV")?;
        Ok(Some(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Numbers from report 20261011-012218, where this case disabled all block drawing.
    #[test]
    fn a_mesh_left_over_from_another_place_is_skipped() {
        let indoors=Sample{sender:1,world:1,frame:1,active:1,mc:[-1245186.7,129.1,-4149243.4],..Sample::default()};
        let anchor=Anchor::restored(indoors,[-1245184.0,128.0,-4149248.0],[0.0;3]);
        // The last outdoor mesh, seen for a moment after stepping through the door.
        assert!(mesh_origin([122.0,26.0,-580.0],indoors,anchor).is_none());
        // The mesh built around the player indoors lands near that interior's zero.
        assert_eq!(mesh_origin([-1245195.0,121.0,-4149252.0],indoors,anchor),Some([-770.0,280.0,-490.0]));
        // Too far on a single axis is enough to count as another place.
        assert!(mesh_origin([-1245186.7+65.0,129.1,-4149243.4],indoors,anchor).is_none());
        assert!(mesh_origin([-1245186.7+63.0,129.1,-4149243.4],indoors,anchor).is_some());
    }
}
