//! Adds up how long the bridge's own per-frame work takes, for one log line every few seconds.
/// How often a line is produced.
pub const PERIOD_MS:u64=5000;
pub struct Cost {frames:u64,total_us:u64,worst_us:u64,since_ms:u64}
impl Cost {
    pub const fn new()->Self {Self{frames:0,total_us:0,worst_us:0,since_ms:0}}
    /// Counts one frame that took `micros`. Once a period has passed, returns its summary
    /// (frames, average and worst microseconds, and the period's real length) and starts again.
    pub fn add(&mut self,micros:u64,now_ms:u64)->Option<String> {
        if self.frames==0 {self.since_ms=now_ms;}
        self.frames+=1;
        self.total_us=self.total_us.saturating_add(micros);
        self.worst_us=self.worst_us.max(micros);
        let period=now_ms.saturating_sub(self.since_ms);
        if period<PERIOD_MS {return None;}
        let line=format!("frames={} avg_us={} max_us={} period_ms={period}",self.frames,self.total_us/self.frames,self.worst_us);
        *self=Self::new();
        Some(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_summary_per_period_then_a_fresh_start() {
        let mut cost=Cost::new();
        // 100 frames, 50 ms apart, each 200 microseconds except one slow one.
        let mut lines=Vec::new();
        for frame in 0..=100_u64 {
            let micros=if frame==40 {9000} else {200};
            if let Some(line)=cost.add(micros,1_000+frame*50) {lines.push(line);}
        }
        // (100 x 200 + 9000) / 101 frames = 287 on average.
        assert_eq!(lines,["frames=101 avg_us=287 max_us=9000 period_ms=5000"]);
        // The slow frame of the last period is forgotten.
        assert_eq!(cost.add(100,10_000),None);
        assert_eq!(cost.add(300,15_000).as_deref(),Some("frames=2 avg_us=200 max_us=300 period_ms=5000"));
    }
}
