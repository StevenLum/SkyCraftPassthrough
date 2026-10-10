//! Position payload and origin/axis conversion, shared by tests and game adapters.
use crate::{read_u32, read_u64, RECORD_SIZE};
pub const MAX_SAMPLE_AGE_MS: u64 = 250;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sample {
    pub sender: u64,
    pub world: u64,
    pub frame: u64,
    pub sent_ms: u64,
    pub active: u32,
    pub partial: f64,
    pub mc: [f64; 3],
    pub yaw: f64,
    pub pitch: f64,
    pub save_id:[u8;16],
    pub mc_id:[u8;16],
    pub interact_seq:u64, pub interact_ms:u64,
    pub attack_seq:u64,pub attack_handle:u32,pub attack_damage:f32,pub attack_ms:u64,
    pub recovery_ack:u64,
    pub damage_epoch:u64,pub damage_ack:u64,pub damage_ready:u32,
    pub menu_seq:u64,pub menu_action:u32,pub menu_context:u64,pub menu_ms:u64,
    /// Menu the pointer motion belongs to (0 = none) and its running total in window sizes.
    pub cursor_context:u64,pub cursor:[f32;2],
    /// Minecraft death counter for this session (0 = none) and when that death happened.
    pub death_seq:u64,pub death_ms:u64,
}

impl Sample {
    pub fn valid(&self) -> bool {
        self.frame > 0 && self.active <= 1 && self.damage_ready<=1 && self.partial.is_finite()
            && (0.0..=1.0).contains(&self.partial)
            && self.mc.iter().all(|v| v.is_finite() && v.abs() <= 30_000_000.0)
            && (self.active == 0 || self.world != 0)
            && self.yaw.is_finite() && (-180.0..=180.0).contains(&self.yaw)
            && self.pitch.is_finite() && (-90.0..=90.0).contains(&self.pitch)
            && self.cursor.iter().all(|v| v.is_finite())
    }
    pub(crate) fn decode(bytes: &[u8; RECORD_SIZE]) -> Self {
        Self { death_seq:read_u64(bytes,440),death_ms:read_u64(bytes,448),cursor_context:read_u64(bytes,424),cursor:[432,436].map(|offset| f32::from_bits(read_u32(bytes,offset))),attack_seq:read_u64(bytes,400),attack_handle:read_u32(bytes,408),attack_damage:f32::from_bits(read_u32(bytes,412)),attack_ms:read_u64(bytes,416),damage_epoch:read_u64(bytes,376),damage_ack:read_u64(bytes,384),damage_ready:read_u32(bytes,392),recovery_ack:read_u64(bytes,368),sender: read_u64(bytes, 128), world: read_u64(bytes, 136),
            frame: read_u64(bytes, 144), sent_ms: read_u64(bytes, 152),
            active: read_u32(bytes, 160), partial: f64::from_bits(read_u64(bytes, 168)),
            mc: [176,184,192].map(|offset| f64::from_bits(read_u64(bytes, offset))),
            yaw:f64::from_bits(read_u64(bytes,200)), pitch:f64::from_bits(read_u64(bytes,208)), save_id:bytes[240..256].try_into().unwrap(), mc_id:bytes[256..272].try_into().unwrap(), interact_seq:read_u64(bytes,272), interact_ms:read_u64(bytes,280), menu_seq:read_u64(bytes,288),menu_action:read_u32(bytes,296),menu_context:read_u64(bytes,304),menu_ms:read_u64(bytes,312) }
    }
    pub(crate) fn encode(&self, bytes: &mut [u8; RECORD_SIZE]) {
        bytes[440..448].copy_from_slice(&self.death_seq.to_le_bytes());
        bytes[448..456].copy_from_slice(&self.death_ms.to_le_bytes());
        bytes[424..432].copy_from_slice(&self.cursor_context.to_le_bytes());
        bytes[432..436].copy_from_slice(&self.cursor[0].to_le_bytes());
        bytes[436..440].copy_from_slice(&self.cursor[1].to_le_bytes());
        bytes[412..416].copy_from_slice(&self.attack_damage.to_le_bytes());
        bytes[408..412].copy_from_slice(&self.attack_handle.to_le_bytes());
        bytes[392..396].copy_from_slice(&self.damage_ready.to_le_bytes());
        for (offset, value) in [(400,self.attack_seq),(416,self.attack_ms),(376,self.damage_epoch),(384,self.damage_ack),(368,self.recovery_ack),(128,self.sender), (136,self.world), (144,self.frame),
            (152,self.sent_ms), (168,self.partial.to_bits()), (176,self.mc[0].to_bits()),
            (184,self.mc[1].to_bits()), (192,self.mc[2].to_bits()),
            (200,self.yaw.to_bits()),(208,self.pitch.to_bits())] {
            bytes[offset..offset+8].copy_from_slice(&value.to_le_bytes());
        }
        bytes[288..296].copy_from_slice(&self.menu_seq.to_le_bytes());
        bytes[296..300].copy_from_slice(&self.menu_action.to_le_bytes());
        bytes[304..312].copy_from_slice(&self.menu_context.to_le_bytes());
        bytes[312..320].copy_from_slice(&self.menu_ms.to_le_bytes());
        bytes[272..280].copy_from_slice(&self.interact_seq.to_le_bytes());
        bytes[280..288].copy_from_slice(&self.interact_ms.to_le_bytes());
        bytes[256..272].copy_from_slice(&self.mc_id);
        bytes[240..256].copy_from_slice(&self.save_id);
        bytes[160..164].copy_from_slice(&self.active.to_le_bytes());
    }
    pub fn log_fields(&self) -> String {
        // Rust's shortest round-trip decimal format preserves even tiny fractions.
        format!("sender={} world={} frame={} partial={} mc_x={} mc_y={} mc_z={} mc_yaw={} mc_pitch={}",
            self.sender,self.world,self.frame,self.partial,self.mc[0],self.mc[1],self.mc[2],self.yaw,self.pitch)
    }
}

#[derive(Clone, Copy)]
pub struct Anchor { sender: u64, world: u64, mc: [f64; 3], sky: [f64; 3] }
impl Anchor {
    pub fn coordinates(&self)->([f64;3],[f64;3]) { (self.mc,self.sky) }
    pub fn restored(sample:Sample,mc:[f64;3],sky:[f64;3])->Self {
        Self{sender:sample.sender,world:sample.world,mc,sky}
    }
    pub fn minecraft(&self, sky: [f64;3]) -> [f64;3] {
        [self.mc[0]+(sky[0]-self.sky[0])/70.0,
         self.mc[1]+(sky[2]-self.sky[2])/70.0,
         self.mc[2]-(sky[1]-self.sky[1])/70.0]
    }
    pub fn new(sample: Sample, sky: [f32; 3]) -> Self {
        Self { sender: sample.sender, world: sample.world, mc: sample.mc, sky: sky.map(f64::from) }
    }
    pub fn matches(&self, sample: Sample) -> bool {
        self.sender == sample.sender && self.world == sample.world
    }
    pub fn target(&self, sample: Sample) -> Option<[f32;3]> {
        if !self.matches(sample) || !sample.valid() { return None; }
        let d = [0,1,2].map(|i| sample.mc[i] - self.mc[i]);
        let target = [self.sky[0]+70.0*d[0], self.sky[1]-70.0*d[2], self.sky[2]+70.0*d[1]];
        target.iter().all(|v| v.is_finite() && v.abs() < 100_000_000.0).then(|| target.map(|v| v as f32))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mapping_preserves_anchor_and_converts_axes_and_scale() {
        let sample = Sample { sender: 1, world: 1, frame: 1, active: 1,
            partial: 0.5, mc: [12.25,64.0,-8.5], ..Sample::default() };
        let anchor = Anchor::new(sample, [100.0,-200.0,300.0]);
        assert_eq!(anchor.target(sample), Some([100.0,-200.0,300.0]));
        let moved = Sample { mc: [13.25,64.5,-10.5], ..sample };
        assert_eq!(anchor.target(moved), Some([170.0,-60.0,335.0]));
        assert!(anchor.target(Sample { sender: 2, ..sample }).is_none());
        assert!(anchor.target(Sample { world: 2, ..sample }).is_none());
    }
    #[test]
    fn invalid_values_cannot_move_a_player() {
        let sample = Sample { frame: 1, world: 1, active: 1, ..Sample::default() };
        assert!(!Sample { mc: [f64::NAN,0.0,0.0], ..sample }.valid());
        assert!(!Sample { partial: 1.1, ..sample }.valid());
        assert!(!Sample { frame: 0, ..sample }.valid());
        assert!(!Sample { cursor: [f32::NAN,0.0], ..sample }.valid());
    }
    #[test]
    fn sample_roundtrip_is_bit_exact() {
        let sample = Sample { sender: 123, world: 4, frame: 991, sent_ms: 55, active: 1,
            partial: 0.37, mc: [-0.0001,64.123456789, 20000.33333],
            cursor_context: 7, cursor: [-0.25,1.5], death_seq: 3, death_ms: 44, ..Sample::default() };
        let mut bytes = [0; RECORD_SIZE]; sample.encode(&mut bytes);
        let result = Sample::decode(&bytes);
        assert_eq!(sample.log_fields(), result.log_fields());
        assert_eq!(result.sent_ms, 55);
        assert_eq!((result.cursor_context,result.cursor), (7,[-0.25,1.5]));
        assert_eq!((result.death_seq,result.death_ms), (3,44));
        for i in 0..3 { assert_eq!(sample.mc[i].to_bits(), result.mc[i].to_bits()); }
    }
}
