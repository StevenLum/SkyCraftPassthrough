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
}

impl Sample {
    pub fn valid(&self) -> bool {
        self.frame > 0 && self.active <= 1 && self.partial.is_finite()
            && (0.0..=1.0).contains(&self.partial)
            && self.mc.iter().all(|v| v.is_finite() && v.abs() <= 30_000_000.0)
            && (self.active == 0 || self.world != 0)
            && self.yaw.is_finite() && (-180.0..=180.0).contains(&self.yaw)
            && self.pitch.is_finite() && (-90.0..=90.0).contains(&self.pitch)
    }
    pub(crate) fn decode(bytes: &[u8; RECORD_SIZE]) -> Self {
        Self { sender: read_u64(bytes, 128), world: read_u64(bytes, 136),
            frame: read_u64(bytes, 144), sent_ms: read_u64(bytes, 152),
            active: read_u32(bytes, 160), partial: f64::from_bits(read_u64(bytes, 168)),
            mc: [176,184,192].map(|offset| f64::from_bits(read_u64(bytes, offset))),
            yaw:f64::from_bits(read_u64(bytes,200)), pitch:f64::from_bits(read_u64(bytes,208)) }
    }
    pub(crate) fn encode(&self, bytes: &mut [u8; RECORD_SIZE]) {
        for (offset, value) in [(128,self.sender), (136,self.world), (144,self.frame),
            (152,self.sent_ms), (168,self.partial.to_bits()), (176,self.mc[0].to_bits()),
            (184,self.mc[1].to_bits()), (192,self.mc[2].to_bits()),
            (200,self.yaw.to_bits()),(208,self.pitch.to_bits())] {
            bytes[offset..offset+8].copy_from_slice(&value.to_le_bytes());
        }
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
    }
    #[test]
    fn sample_roundtrip_is_bit_exact() {
        let sample = Sample { sender: 123, world: 4, frame: 991, sent_ms: 55, active: 1,
            partial: 0.37, mc: [-0.0001,64.123456789, 20000.33333], ..Sample::default() };
        let mut bytes = [0; RECORD_SIZE]; sample.encode(&mut bytes);
        let result = Sample::decode(&bytes);
        assert_eq!(sample.log_fields(), result.log_fields());
        assert_eq!(result.sent_ms, 55);
        for i in 0..3 { assert_eq!(sample.mc[i].to_bits(), result.mc[i].to_bits()); }
    }
}
