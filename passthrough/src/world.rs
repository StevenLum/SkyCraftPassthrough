//! Versioned complete world snapshots. Coordinates are already in Minecraft space.
use std::io;
use std::fmt::Write as _;
use crate::{read_u32,read_u64,Role,native::write_log};
pub const MAX_BOXES: usize=8192;
pub const MAX_NPCS: usize=128;
pub const HEADER:usize=144;
pub const SIZE: usize=HEADER+MAX_BOXES*48+MAX_NPCS*32+8+48+528;
/// What Skyrim's crosshair is on: u32 here, then 4 reserved bytes, just before the damage
/// section so every end-relative offset is unchanged.
pub const TARGET_AT: usize=HEADER+MAX_BOXES*48+MAX_NPCS*32;
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
/// A living character that Skyrim reports as hostile to the player.
pub const TARGET_HOSTILE: u32=6;
pub const MAX_TARGET_KIND: u32=TARGET_HOSTILE;

#[derive(Clone,Default)]
pub struct Snapshot {
    pub sender:u64, pub recipient:u64, pub world:u64, pub epoch:u64,
    pub sequence:u64, pub sent_ms:u64, pub active:bool,
    pub boxes:Vec<[f64;6]>, pub npcs:Vec<(u64,[f64;3])>,
    pub center:[f64;3], pub radius:f64,
    /// Zero in a newly constructed reply means use the horizontal radius.
    pub radius_y:f64,
    pub seated:bool,
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
        let mut b=vec![0;SIZE]; b[..8].copy_from_slice(b"PTWORL14");
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
        b[128..136].copy_from_slice(&(if self.radius_y==0.0 {self.radius} else {self.radius_y}).to_le_bytes());
        b[136..144].copy_from_slice(&self.radius.to_le_bytes());
        b[124..128].copy_from_slice(&self.origin_block.to_le_bytes());
        b[120..124].copy_from_slice(&(self.admitted as u32).to_le_bytes());
        b[112..116].copy_from_slice(&self.cell_id.to_le_bytes());
        b[116..120].copy_from_slice(&self.worldspace_id.to_le_bytes());
        for (i,shape) in self.boxes.iter().enumerate() {
            for (j,value) in shape.iter().enumerate() { let at=HEADER+i*48+j*8; b[at..at+8].copy_from_slice(&value.to_le_bytes()); }
        }
        for (i,(id,pos)) in self.npcs.iter().enumerate() {
            let at=HEADER+MAX_BOXES*48+i*32; b[at..at+8].copy_from_slice(&id.to_le_bytes());
            for j in 0..3 { b[at+8+j*8..at+16+j*8].copy_from_slice(&pos[j].to_le_bytes()); }
        }
        let damage_at=SIZE-48-528;
        b[TARGET_AT+4..TARGET_AT+8].copy_from_slice(&(self.seated as u32).to_le_bytes());
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
        if &b[..8]!=b"PTWORL14" || read_u32(b,56)>1 || read_u32(b,120)>1 || read_u32(b,124)>MAX_HOLD_REASON
            || read_u32(b,TARGET_AT)>MAX_TARGET_KIND || read_u32(b,TARGET_AT+4)>1 { return Err(bad()); }
        let count=read_u32(b,60) as usize; let npc_count=read_u32(b,64) as usize;
        if count>MAX_BOXES || npc_count>MAX_NPCS { return Err(bad()); }
        let mut s=Self{sender:read_u64(b,8),recipient:read_u64(b,16),world:read_u64(b,24),
            epoch:read_u64(b,32),sequence:read_u64(b,40),sent_ms:read_u64(b,48),active:read_u32(b,56)==1,..Self::default()};
        let damage_at=SIZE-48-528;
        s.attack_handle=read_u32(b,damage_at+12);
        s.target_kind=read_u32(b,TARGET_AT);
        s.seated=read_u32(b,TARGET_AT+4)==1;
        s.damage_epoch=read_u64(b,damage_at);
        let damage_count=read_u32(b,damage_at+8) as usize;if damage_count>32 {return Err(bad());}
        let mut previous=0;
        for i in 0..damage_count {
            let at=damage_at+16+i*16;let seq=read_u64(b,at);
            let fraction=f32::from_bits(read_u32(b,at+8));let raw=f32::from_bits(read_u32(b,at+12));
            // `raw` is Skyrim's own amount: positive for health taken, negative for health
            // restored (since protocol 31). Zero is neither.
            if s.damage_epoch==0 || seq<=previous || !fraction.is_finite() || fraction<=0.0 || !raw.is_finite() || raw==0.0 {return Err(bad());}
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
        s.radius_y=f64::from_bits(read_u64(b,128));
        let radius_z=f64::from_bits(read_u64(b,136));
        s.cell_id=read_u32(b,112); s.worldspace_id=read_u32(b,116);
        if !s.center.iter().copied().all(finite) || !s.radius.is_finite()
            || !s.radius_y.is_finite() || radius_z!=s.radius
            || (s.active && (s.radius!=4.0 || ![4.0,12.0,24.0].contains(&s.radius_y))) { return Err(bad()); }
        for i in 0..count {
            let shape=std::array::from_fn(|j|f64::from_bits(read_u64(b,HEADER+i*48+j*8)));
            if !shape.iter().copied().all(finite) || (0..3).any(|j|shape[j]>=shape[j+3] || shape[j+3]-shape[j]>16.0) { return Err(bad()); }
            s.boxes.push(shape);
        }
        for i in 0..npc_count {
            let at=HEADER+MAX_BOXES*48+i*32;
            let pos=std::array::from_fn(|j|f64::from_bits(read_u64(b,at+8+j*8)));
            if !pos.iter().copied().all(finite) { return Err(bad()); }
            s.npcs.push((read_u64(b,at),pos));
        }
        Ok(Some(s))
    }
    pub fn log(&self, role:Role, direction:&str)->io::Result<()> {
        write_log(role,self.lines(direction,every_box()))
    }
    /// One summary line per snapshot, plus health and the few characters. `boxes_sum` covers
    /// every box exactly, so equal values on the SEND and RECV lines prove the collision
    /// crossed unchanged. A line per box (thousands a second, on both games' main threads) is
    /// written only when asked for.
    fn lines(&self,direction:&str,every_box:bool)->String {
        let key=format!("sender={} world={} epoch={} snapshot={}",self.sender,self.world,self.epoch,self.sequence);
        let mut lines=format!("event=WORLD_{direction} {key} active={} boxes={} npcs={} boxes_sum={:016X}\n",
            self.active,self.boxes.len(),self.npcs.len(),self.boxes_sum());
        if self.active && self.admitted {
            let _=writeln!(lines,"event=SCENE_HEALTH_{direction} {key} skyrim_health={}",self.player_health);
        }
        if every_box {
            for (i,b) in self.boxes.iter().enumerate() {
                let _=writeln!(lines,"event=BOX_{direction} {key} index={i} min_x={} min_y={} min_z={} max_x={} max_y={} max_z={}",b[0],b[1],b[2],b[3],b[4],b[5]);
            }
        }
        for (id,p) in &self.npcs {
            let _=writeln!(lines,"event=NPC_{direction} {key} id={id} mc_x={} mc_y={} mc_z={}",p[0],p[1],p[2]);
        }
        lines
    }
    /// FNV-1a over the exact bits of every box coordinate, in order.
    fn boxes_sum(&self)->u64 {boxes_checksum(&self.boxes)}
}
/// FNV-1a over the exact bits of every box coordinate, in order. Equal boxes give an equal
/// value, so it also tells a scan that found nothing new from one that did.
pub(crate) fn boxes_checksum(boxes:&[[f64;6]])->u64 {
    let mut hash=0xcbf29ce484222325_u64;
    for value in boxes.iter().flatten() {
        for byte in value.to_bits().to_le_bytes() {hash=(hash^byte as u64).wrapping_mul(0x100000001b3);}
    }
    hash
}
/// True when PASSTHROUGH_LOG_BOXES=1 was set before the game started: log every box again.
fn every_box()->bool {
    static CHOICE:std::sync::OnceLock<bool>=std::sync::OnceLock::new();
    *CHOICE.get_or_init(|| std::env::var("PASSTHROUGH_LOG_BOXES").as_deref()==Ok("1"))
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
        assert_eq!((SIZE,TARGET_AT),(398_040,397_456));
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
    #[test]
    fn tall_coverage_keeps_half_block_boxes_and_the_tail_fields_intact() {
        for radius_y in [4.0,12.0,24.0] {
            let sent=Snapshot{active:true,radius:4.0,radius_y,attack_handle:77,
                boxes:vec![[1.0,2.0,3.0,1.5,2.5,3.5]],npcs:vec![(99,[1.0,2.0,3.0])],
                recovery_token:123,save_id:[7;16],..Snapshot::default()};
            let bytes=sent.encode().unwrap();
            assert_eq!(f64::from_bits(read_u64(&bytes,128)),radius_y);
            assert_eq!(f64::from_bits(read_u64(&bytes,136)),4.0);
            let back=Snapshot::decode(&bytes).unwrap().unwrap();
            assert_eq!((back.radius_y,back.boxes,back.npcs,back.attack_handle,back.recovery_token,back.save_id),
                (radius_y,sent.boxes,sent.npcs,77,123,[7;16]));
        }
        for bad in [3.0,12.5,25.0,f64::NAN] {
            assert!(Snapshot{active:true,radius:4.0,radius_y:bad,..Snapshot::default()}.encode().is_err());
        }
    }
    #[test]
    fn deliberate_seat_status_is_a_validated_flag_separate_from_crosshair_kind() {
        let sent=Snapshot{active:true,admitted:true,radius:4.0,seated:true,target_kind:TARGET_NONE,..Snapshot::default()};
        let mut b=sent.encode().unwrap();
        assert_eq!(read_u32(&b,TARGET_AT+4),1);
        assert!(Snapshot::decode(&b).unwrap().unwrap().seated);
        b[TARGET_AT+4..TARGET_AT+8].copy_from_slice(&2_u32.to_le_bytes());
        assert!(Snapshot::decode(&b).is_err());
    }
    /// Hits and restored health share one list; the sign of the Skyrim amount tells them apart.
    #[test]
    fn restored_health_travels_as_an_event_with_a_negative_amount() {
        let sent=Snapshot{active:true,radius:4.0,damage_epoch:3,
            damage_events:vec![(1,0.25,25.0),(2,0.5,-50.0),(3,0.01,-1.0)],..Snapshot::default()};
        let back=Snapshot::decode(&sent.encode().unwrap()).unwrap().unwrap();
        assert_eq!(back.damage_events,sent.damage_events);
        // Still refused: no amount, no share of health, a share that is not positive, wrong order.
        for bad in [vec![(1,0.5,0.0)],vec![(1,0.0,-5.0)],vec![(1,-0.5,-5.0)],vec![(1,f32::NAN,-5.0)],
            vec![(1,0.5,f32::NEG_INFINITY)],vec![(2,0.5,-5.0),(1,0.5,5.0)]] {
            let broken=Snapshot{active:true,radius:4.0,damage_epoch:3,damage_events:bad.clone(),..Snapshot::default()};
            assert!(broken.encode().is_err(),"accepted {bad:?}");
        }
    }
    /// Report 20261011-101205: 97% of all log lines were one line per collision box.
    #[test]
    fn a_snapshot_logs_a_summary_and_a_checksum_not_every_box() {
        let mut sent=Snapshot{active:true,admitted:true,radius:4.0,boxes:vec![[0.0,0.0,0.0,1.0,1.0,1.0];1000],
            npcs:vec![(7,[1.0,2.0,3.0])],..Snapshot::default()};
        let brief=sent.lines("SEND",false);
        // The summary, the health line and one character.
        assert_eq!(brief.lines().count(),3);
        assert!(brief.contains(" boxes=1000 ") && !brief.contains("event=BOX_"));
        assert_eq!(sent.lines("SEND",true).lines().filter(|line|line.starts_with("event=BOX_SEND ")).count(),1000);
        // What Minecraft decodes logs the same checksum, and one changed coordinate changes it.
        let sum=|s:&Snapshot| s.lines("X",false).split("boxes_sum=").nth(1).unwrap()[..16].to_owned();
        let received=Snapshot::decode(&sent.encode().unwrap()).unwrap().unwrap();
        assert_eq!(sum(&sent),sum(&received));
        let before=sum(&sent);
        sent.boxes[999][3]=1.5;
        assert_ne!(before,sum(&sent));
        // No boxes at all still has a well-defined value.
        assert_eq!(sum(&Snapshot::default()),"CBF29CE484222325");
    }
}
