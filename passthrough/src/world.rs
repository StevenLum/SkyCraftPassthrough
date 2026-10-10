//! Versioned complete world snapshots. Coordinates are already in Minecraft space.
use std::io;
use std::fmt::Write as _;
use crate::{read_u32,read_u64,Role,native::write_log};
pub const MAX_BOXES: usize=8192;
pub const MAX_NPCS: usize=128;
pub const SIZE: usize=128+MAX_BOXES*48+MAX_NPCS*32+8+48+528;
/// What Skyrim's crosshair is on: u32 here, then 4 reserved bytes, just before the damage
/// section so every end-relative offset is unchanged.
pub const TARGET_AT: usize=128+MAX_BOXES*48+MAX_NPCS*32;
pub const MAX_AGE_MS: u64=2500;
/// Hold reasons in header word 124: 0 none .. 6 as documented in docs/ABI.md, 7 player dead.
/// A new reason must raise the maximum too, or the sender rejects its own reply.
pub const HOLD_DEAD: u32=7;
pub const MAX_HOLD_REASON: u32=HOLD_DEAD;
/// Crosshair target kinds. A new kind must raise the maximum too (see the test).
pub const TARGET_NONE: u32=0;
pub const TARGET_OTHER: u32=1;
pub const TARGET_CONTAINER: u32=2;
pub const TARGET_ALIVE: u32=3;
pub const TARGET_BODY: u32=4;
pub const TARGET_DOOR: u32=5;
pub const MAX_TARGET_KIND: u32=TARGET_DOOR;

#[derive(Clone,Default)]
pub struct Snapshot {
    pub sender:u64, pub recipient:u64, pub world:u64, pub epoch:u64,
    pub sequence:u64, pub sent_ms:u64, pub active:bool,
    pub boxes:Vec<[f64;6]>, pub npcs:Vec<(u64,[f64;3])>,
    pub center:[f64;3], pub radius:f64,
    pub cell_id:u32, pub worldspace_id:u32,
    pub attack_handle:u32,pub target_kind:u32,
    pub damage_epoch:u64,pub damage_events:Vec<(u64,f32,f32)>,
    pub player_health:f32,
    pub recovery_token:u64,pub recovery_target:[f64;3],
    pub save_id:[u8;16], pub admitted:bool, pub origin_block:u32,
}
impl Snapshot {
    pub fn encode(&self)->io::Result<Vec<u8>> {
        if self.damage_events.len()>32 || self.boxes.len()>MAX_BOXES || self.npcs.len()>MAX_NPCS { return Err(io::Error::other("world snapshot capacity exceeded")); }
        let mut b=vec![0;SIZE]; b[..8].copy_from_slice(b"PTWORL13");
        for (i,v) in [self.sender,self.recipient,self.world,self.epoch,self.sequence,self.sent_ms].into_iter().enumerate() {
            b[8+i*8..16+i*8].copy_from_slice(&v.to_le_bytes());
        }
        b[56..60].copy_from_slice(&(self.active as u32).to_le_bytes());
        b[60..64].copy_from_slice(&(self.boxes.len() as u32).to_le_bytes());
        b[64..68].copy_from_slice(&(self.npcs.len() as u32).to_le_bytes());
        b[68..72].copy_from_slice(&self.player_health.to_le_bytes());
        for (j,value) in self.center.iter().chain(std::iter::once(&self.radius)).enumerate() {
            b[80+j*8..88+j*8].copy_from_slice(&value.to_le_bytes());
        }
        b[124..128].copy_from_slice(&self.origin_block.to_le_bytes());
        b[120..124].copy_from_slice(&(self.admitted as u32).to_le_bytes());
        b[112..116].copy_from_slice(&self.cell_id.to_le_bytes());
        b[116..120].copy_from_slice(&self.worldspace_id.to_le_bytes());
        for (i,shape) in self.boxes.iter().enumerate() {
            for (j,value) in shape.iter().enumerate() { let at=128+i*48+j*8; b[at..at+8].copy_from_slice(&value.to_le_bytes()); }
        }
        for (i,(id,pos)) in self.npcs.iter().enumerate() {
            let at=128+MAX_BOXES*48+i*32; b[at..at+8].copy_from_slice(&id.to_le_bytes());
            for j in 0..3 { b[at+8+j*8..at+16+j*8].copy_from_slice(&pos[j].to_le_bytes()); }
        }
        let damage_at=SIZE-48-528;
        b[TARGET_AT..TARGET_AT+4].copy_from_slice(&self.target_kind.to_le_bytes());
        b[damage_at+12..damage_at+16].copy_from_slice(&self.attack_handle.to_le_bytes());
        b[damage_at..damage_at+8].copy_from_slice(&self.damage_epoch.to_le_bytes());
        b[damage_at+8..damage_at+12].copy_from_slice(&(self.damage_events.len() as u32).to_le_bytes());
        for (i,(seq,fraction,raw)) in self.damage_events.iter().enumerate() {
            let at=damage_at+16+i*16;b[at..at+8].copy_from_slice(&seq.to_le_bytes());
            b[at+8..at+12].copy_from_slice(&fraction.to_le_bytes());b[at+12..at+16].copy_from_slice(&raw.to_le_bytes());
        }
        b[SIZE-48..SIZE-40].copy_from_slice(&self.recovery_token.to_le_bytes());
        for (j,value) in self.recovery_target.iter().enumerate() {let at=SIZE-40+j*8;b[at..at+8].copy_from_slice(&value.to_le_bytes());}
        b[SIZE-16..].copy_from_slice(&self.save_id);
        // Validate our output as strictly as the receiver.
        Self::decode(&b)?;
        Ok(b)
    }
    pub fn decode(b:&[u8])->io::Result<Option<Self>> {
        if b.len()!=SIZE { return Err(io::Error::other("world snapshot size mismatch")); }
        if b[..8]==[0;8] { return Ok(None); }
        let bad=||io::Error::other("invalid world snapshot");
        if &b[..8]!=b"PTWORL13" || read_u32(b,56)>1 || read_u32(b,120)>1 || read_u32(b,124)>MAX_HOLD_REASON
            || read_u32(b,TARGET_AT)>MAX_TARGET_KIND { return Err(bad()); }
        let count=read_u32(b,60) as usize; let npc_count=read_u32(b,64) as usize;
        if count>MAX_BOXES || npc_count>MAX_NPCS { return Err(bad()); }
        let mut s=Self{sender:read_u64(b,8),recipient:read_u64(b,16),world:read_u64(b,24),
            epoch:read_u64(b,32),sequence:read_u64(b,40),sent_ms:read_u64(b,48),active:read_u32(b,56)==1,..Self::default()};
        let damage_at=SIZE-48-528;
        s.attack_handle=read_u32(b,damage_at+12);
        s.target_kind=read_u32(b,TARGET_AT);
        s.damage_epoch=read_u64(b,damage_at);
        let damage_count=read_u32(b,damage_at+8) as usize;if damage_count>32 {return Err(bad());}
        let mut previous=0;
        for i in 0..damage_count {
            let at=damage_at+16+i*16;let seq=read_u64(b,at);
            let fraction=f32::from_bits(read_u32(b,at+8));let raw=f32::from_bits(read_u32(b,at+12));
            if s.damage_epoch==0 || seq<=previous || !fraction.is_finite() || fraction<=0.0 || !raw.is_finite() || raw<=0.0 {return Err(bad());}
            previous=seq;s.damage_events.push((seq,fraction,raw));
        }
        s.recovery_token=read_u64(b,SIZE-48);
        s.recovery_target=std::array::from_fn(|j|f64::from_bits(read_u64(b,SIZE-40+j*8)));
        if !s.recovery_target.iter().all(|n|n.is_finite() && n.abs()<=30_000_000.0) {return Err(bad());}
        s.player_health=f32::from_bits(read_u32(b,68));
        if !s.player_health.is_finite() {return Err(bad());}
        s.origin_block=read_u32(b,124);
        s.admitted=read_u32(b,120)==1;
        s.save_id.copy_from_slice(&b[SIZE-16..]);
        let finite=|v:f64|v.is_finite() && v.abs()<=30_000_000.0;
        s.center=std::array::from_fn(|j|f64::from_bits(read_u64(b,80+j*8)));
        s.radius=f64::from_bits(read_u64(b,104));
        s.cell_id=read_u32(b,112); s.worldspace_id=read_u32(b,116);
        if !s.center.iter().copied().all(finite) || !s.radius.is_finite()
            || (s.active && s.radius!=4.0) { return Err(bad()); }
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
        if self.active && self.admitted {
            let _=writeln!(lines,"event=SCENE_HEALTH_{direction} {key} skyrim_health={}",self.player_health);
        }
        for (i,b) in self.boxes.iter().enumerate() {
            let _=writeln!(lines,"event=BOX_{direction} {key} index={i} min_x={} min_y={} min_z={} max_x={} max_y={} max_z={}",b[0],b[1],b[2],b[3],b[4],b[5]);
        }
        for (id,p) in &self.npcs {
            let _=writeln!(lines,"event=NPC_{direction} {key} id={id} mc_x={} mc_y={} mc_z={}",p[0],p[1],p[2]);
        }
        write_log(role,lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// The sender validates its own output, so every hold reason it uses must pass.
    #[test]
    fn hold_replies_encode_for_every_reason() {
        for reason in 0..=MAX_HOLD_REASON {
            let reply=Snapshot{active:true,radius:4.0,recipient:2,world:3,origin_block:reason,..Snapshot::default()};
            let bytes=reply.encode().unwrap_or_else(|error| panic!("hold reason {reason} rejected: {error}"));
            assert_eq!(Snapshot::decode(&bytes).unwrap().unwrap().origin_block,reason);
        }
        let unknown=Snapshot{active:true,radius:4.0,origin_block:MAX_HOLD_REASON+1,..Snapshot::default()};
        assert!(unknown.encode().is_err());
    }
    /// Minecraft reads the kind at a fixed place counted from the end of the packet.
    #[test]
    fn crosshair_kind_travels_at_its_documented_place() {
        assert_eq!((SIZE,TARGET_AT),(398_024,397_440));
        assert_eq!(TARGET_AT,SIZE-48-528-8);
        for kind in 0..=MAX_TARGET_KIND {
            let sent=Snapshot{active:true,radius:4.0,attack_handle:77,target_kind:kind,..Snapshot::default()};
            let bytes=sent.encode().unwrap_or_else(|error| panic!("target kind {kind} rejected: {error}"));
            assert_eq!(read_u32(&bytes,TARGET_AT),kind);
            let back=Snapshot::decode(&bytes).unwrap().unwrap();
            assert_eq!((back.target_kind,back.attack_handle),(kind,77));
        }
        let unknown=Snapshot{active:true,radius:4.0,target_kind:MAX_TARGET_KIND+1,..Snapshot::default()};
        assert!(unknown.encode().is_err());
    }
}
