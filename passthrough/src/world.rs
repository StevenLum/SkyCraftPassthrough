//! Versioned complete world snapshots. Coordinates are already in Minecraft space.
use std::io;
use std::fmt::Write as _;
use crate::{read_u32,read_u64,Role,native::write_log};
pub const MAX_BOXES: usize=8192;
pub const MAX_NPCS: usize=128;
pub const SIZE: usize=128+MAX_BOXES*48+MAX_NPCS*32;
pub const MAX_AGE_MS: u64=2500;

#[derive(Clone,Default)]
pub struct Snapshot {
    pub sender:u64, pub recipient:u64, pub world:u64, pub epoch:u64,
    pub sequence:u64, pub sent_ms:u64, pub active:bool,
    pub boxes:Vec<[f64;6]>, pub npcs:Vec<(u64,[f64;3])>,
}
impl Snapshot {
    pub fn encode(&self)->io::Result<Vec<u8>> {
        if self.boxes.len()>MAX_BOXES || self.npcs.len()>MAX_NPCS { return Err(io::Error::other("world snapshot capacity exceeded")); }
        let mut b=vec![0;SIZE]; b[..8].copy_from_slice(b"PTWORLD3");
        for (i,v) in [self.sender,self.recipient,self.world,self.epoch,self.sequence,self.sent_ms].into_iter().enumerate() {
            b[8+i*8..16+i*8].copy_from_slice(&v.to_le_bytes());
        }
        b[56..60].copy_from_slice(&(self.active as u32).to_le_bytes());
        b[60..64].copy_from_slice(&(self.boxes.len() as u32).to_le_bytes());
        b[64..68].copy_from_slice(&(self.npcs.len() as u32).to_le_bytes());
        for (i,shape) in self.boxes.iter().enumerate() {
            for (j,value) in shape.iter().enumerate() { let at=128+i*48+j*8; b[at..at+8].copy_from_slice(&value.to_le_bytes()); }
        }
        for (i,(id,pos)) in self.npcs.iter().enumerate() {
            let at=128+MAX_BOXES*48+i*32; b[at..at+8].copy_from_slice(&id.to_le_bytes());
            for j in 0..3 { b[at+8+j*8..at+16+j*8].copy_from_slice(&pos[j].to_le_bytes()); }
        }
        // Validate our output as strictly as the receiver.
        Self::decode(&b)?;
        Ok(b)
    }
    pub fn decode(b:&[u8])->io::Result<Option<Self>> {
        if b.len()!=SIZE { return Err(io::Error::other("world snapshot size mismatch")); }
        if b[..8]==[0;8] { return Ok(None); }
        let bad=||io::Error::other("invalid world snapshot");
        if &b[..8]!=b"PTWORLD3" || read_u32(b,56)>1 { return Err(bad()); }
        let count=read_u32(b,60) as usize; let npc_count=read_u32(b,64) as usize;
        if count>MAX_BOXES || npc_count>MAX_NPCS { return Err(bad()); }
        let mut s=Self{sender:read_u64(b,8),recipient:read_u64(b,16),world:read_u64(b,24),
            epoch:read_u64(b,32),sequence:read_u64(b,40),sent_ms:read_u64(b,48),active:read_u32(b,56)==1,..Self::default()};
        let finite=|v:f64|v.is_finite() && v.abs()<=30_000_000.0;
        for i in 0..count {
            let shape=std::array::from_fn(|j|f64::from_bits(read_u64(b,128+i*48+j*8)));
            if !shape.iter().copied().all(finite) || (0..3).any(|j|shape[j]>=shape[j+3] || shape[j+3]-shape[j]>16.0) { return Err(bad()); }
            s.boxes.push(shape);
        }
        for i in 0..npc_count {
            let at=128+MAX_BOXES*48+i*32;
            let pos=std::array::from_fn(|j|f64::from_bits(read_u64(b,at+8+j*8)));
            if !pos.iter().copied().all(finite) { return Err(bad()); }
            s.npcs.push((read_u64(b,at),pos));
        }
        Ok(Some(s))
    }
    pub fn log(&self, role:Role, direction:&str)->io::Result<()> {
        let key=format!("sender={} world={} epoch={} snapshot={}",self.sender,self.world,self.epoch,self.sequence);
        let mut lines=format!("event=WORLD_{direction} {key} active={} boxes={} npcs={}\n",self.active,self.boxes.len(),self.npcs.len());
        for (i,b) in self.boxes.iter().enumerate() {
            let _=writeln!(lines,"event=BOX_{direction} {key} index={i} min_x={} min_y={} min_z={} max_x={} max_y={} max_z={}",b[0],b[1],b[2],b[3],b[4],b[5]);
        }
        for (id,p) in &self.npcs {
            let _=writeln!(lines,"event=NPC_{direction} {key} id={id} mc_x={} mc_y={} mc_z={}",p[0],p[1],p[2]);
        }
        write_log(role,lines)
    }
}
