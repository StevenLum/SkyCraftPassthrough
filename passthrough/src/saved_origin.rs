//! One persisted practice-location mapping; save history and travel belong to 0.9.
use crate::{position::{Anchor,Sample},native::write_log,Role};
pub const RECORD:u32=u32::from_le_bytes(*b"PBOR");
pub const SIZE:usize=72;
/// Interiors: a grid of 2048-block squares centred on the Minecraft origin, picked by the
/// low 24 bits of the cell ID. Other outdoor worlds: 8192-block squares east of that grid,
/// picked by the low 16 bits of the worldspace ID. Skyrim (0,0,0) is each square's centre.
const INTERIOR_SQUARE:i64=2048;
const OUTDOOR_SQUARE:i64=8192;
const OUTDOOR_WEST:i64=2048*INTERIOR_SQUARE+524_288;
const INTERIOR_HEIGHT:f64=128.0;
/// Puts Skyrim's sea level (-14000 units, 200 blocks down) on the Minecraft world floor.
const OUTDOOR_HEIGHT:f64=136.0;
const HOME_CLEARANCE:f64=(2*OUTDOOR_SQUARE) as f64;
/// Minecraft position of Skyrim (0,0,0) for a place; `flip` selects the far alternative row.
fn square(cell:u32,worldspace:u32,flip:bool)->[f64;3] {
    if worldspace==0 {
        let key=cell as i64;
        let row=((key>>12)&0xFFF)^if flip {0x800} else {0};
        [(((key&0xFFF)-2048)*INTERIOR_SQUARE) as f64,INTERIOR_HEIGHT,((row-2048)*INTERIOR_SQUARE) as f64]
    } else {
        let key=worldspace as i64;
        let row=((key>>8)&0xFF)^if flip {0x80} else {0};
        [(OUTDOOR_WEST+(key&0xFF)*OUTDOOR_SQUARE+OUTDOOR_SQUARE/2) as f64,OUTDOOR_HEIGHT,
            ((row-128)*OUTDOOR_SQUARE+OUTDOOR_SQUARE/2) as f64]
    }
}
#[derive(Clone,Copy)]
pub struct Origin { pub mc_id:[u8;16], pub cell:u32, pub worldspace:u32, pub mc:[f64;3], pub sky:[f64;3] }
impl Origin {
    pub fn new(sample:Sample,current:[f32;3],cell:u32,worldspace:u32)->Self {
        let (mc,sky)=Anchor::new(sample,current).coordinates();
        Self{mc_id:sample.mc_id,cell,worldspace,mc,sky}
    }
    pub fn valid(&self)->bool {
        self.mc_id!=[0;16] && self.cell!=0
            && self.mc.iter().all(|v|v.is_finite() && v.abs()<=30_000_000.0)
            && self.sky.iter().all(|v|v.is_finite() && v.abs()<100_000_000.0)
    }
    pub fn encode(&self)->[u8;SIZE] {
        let mut b=[0;SIZE];b[..16].copy_from_slice(&self.mc_id);
        b[16..20].copy_from_slice(&self.cell.to_le_bytes());
        b[20..24].copy_from_slice(&self.worldspace.to_le_bytes());
        for (j,v) in self.mc.iter().chain(self.sky.iter()).enumerate() {
            b[24+j*8..32+j*8].copy_from_slice(&v.to_le_bytes());
        }
        b
    }
    pub fn decode(b:&[u8;SIZE])->Option<Self> {
        let origin=Self{mc_id:b[..16].try_into().ok()?,
            cell:crate::read_u32(b,16),worldspace:crate::read_u32(b,20),
            mc:std::array::from_fn(|j|f64::from_bits(crate::read_u64(b,24+j*8))),
            sky:std::array::from_fn(|j|f64::from_bits(crate::read_u64(b,48+j*8)))};
        origin.valid().then_some(origin)
    }
    /// Exterior cells share worldspace coordinates; interiors have no worldspace.
    pub fn same_location(&self,cell:u32,worldspace:u32)->bool {
        cell!=0 && self.worldspace==worldspace && (worldspace!=0 || self.cell==cell)
    }
    /// The origin to use in a Skyrim place. The saved origin's own place keeps it unchanged.
    /// Any other place gets a region of the Minecraft world worked out from its record ID
    /// alone, so nothing new is saved and an older Skyrim save cannot hand a built-in
    /// region to a different building. Formula and limits: docs/DESIGN.md, 0.10.9.
    pub fn for_location(&self,cell:u32,worldspace:u32)->Option<Origin> {
        if cell==0 {return None;}
        if self.same_location(cell,worldspace) {return Some(*self);}
        // Stay well clear of everything the saved place itself can reach.
        let near_home=|mc:[f64;3]| (mc[0]-self.mc[0]).abs()<HOME_CLEARANCE && (mc[2]-self.mc[2]).abs()<HOME_CLEARANCE;
        let mut mc=square(cell,worldspace,false);
        if near_home(mc) {mc=square(cell,worldspace,true);}
        Some(Origin{mc_id:self.mc_id,cell,worldspace,mc,sky:[0.0;3]})
    }
    pub fn restore(&self,sample:Sample,current:[f32;3],cell:u32,worldspace:u32)->Result<Anchor,u32> {
        if self.mc_id!=sample.mc_id {return Err(3);}
        let Some(place)=self.for_location(cell,worldspace) else {return Err(4);};
        if !sample.valid() {return Err(2);}
        let anchor=Anchor::restored(sample,place.mc,place.sky);
        // Too far to express as a Skyrim position (Minecraft is still in another place's
        // region) is a position mismatch to recover from, not a broken origin.
        let Some(target)=anchor.target(sample) else {return Err(5);};
        let distance=(0..3).map(|j|(target[j] as f64-current[j] as f64).powi(2)).sum::<f64>().sqrt();
        if !distance.is_finite() || distance>140.0 {return Err(5);}
        Ok(anchor)
    }
    pub fn log(&self,state:&str) {
        let _=write_log(Role::Skyrim,format!(
            "event=SCENE_ORIGIN state={} mc_id={:02X?} cell_id={:08X} worldspace_id={:08X} mc_origin={:?} sky_origin={:?}",
            state,self.mc_id,self.cell,self.worldspace,self.mc,self.sky));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// The user's real saved origin: open country (worldspace 3C), outdoor cell 9732.
    fn home()->Origin {
        Origin{mc_id:[9;16],cell:0x9732,worldspace:0x3C,mc:[131.3,100.0,-45.7],sky:[17405.0,-47335.9,-141.6]}
    }
    #[test]
    fn the_saved_place_keeps_its_saved_origin() {
        let home=home();
        let neighbour=home.for_location(0x9731,0x3C).unwrap();
        assert_eq!((neighbour.mc,neighbour.sky),(home.mc,home.sky));
        assert!(home.for_location(0,0x3C).is_none());
    }
    #[test]
    fn an_interior_gets_a_fixed_square_from_its_cell_id() {
        let home=home();
        let inn=home.for_location(0x133C8,0).unwrap();
        assert_eq!(inn.mc,[((0x3C8-2048)*2048) as f64,128.0,((0x13-2048)*2048) as f64]);
        assert_eq!((inn.sky,inn.mc_id,inn.cell,inn.worldspace),([0.0;3],home.mc_id,0x133C8,0));
        // Asking again, or with a different load-order byte, gives the same square.
        assert_eq!(home.for_location(0x133C8,0).unwrap().mc,inn.mc);
        assert_eq!(home.for_location(0x0501_33C8,0).unwrap().mc,inn.mc);
        // Neighbouring cell IDs are at least one whole square apart.
        let next=home.for_location(0x133C9,0).unwrap();
        assert_eq!(next.mc[0]-inn.mc[0],2048.0);
        // Skyrim height 7000 above the interior's zero is 100 blocks above Minecraft 128.
        let sample=Sample{mc_id:home.mc_id,..Sample::default()};
        assert_eq!(Anchor::restored(sample,inn.mc,inn.sky).minecraft([0.0,0.0,7000.0])[1],228.0);
    }
    #[test]
    fn another_outdoor_world_sits_beside_the_interior_grid() {
        let home=home();
        // A walled city: any of its cells, keyed by its worldspace ID.
        let city=home.for_location(0x1A280,0x1A26F).unwrap();
        assert_eq!(city.mc,[(4_718_592+0x6F*8192+4096) as f64,136.0,((0xA2-128)*8192+4096) as f64]);
        assert_eq!(home.for_location(0x1A281,0x1A26F).unwrap().mc,city.mc);
        // The furthest interior square ends before the first outdoor square begins.
        let east=home.for_location(0x000F_FFFF,0).unwrap();
        assert!(east.mc[0]+1024.0<4_718_592.0);
        // Skyrim's sea level lands on the Minecraft world floor.
        let sample=Sample{mc_id:home.mc_id,..Sample::default()};
        assert_eq!(Anchor::restored(sample,city.mc,city.sky).minecraft([0.0,0.0,-14000.0])[1],-64.0);
    }
    #[test]
    fn a_square_that_would_touch_the_saved_place_is_moved_away() {
        let home=home();
        // Low bits that would otherwise put this interior at the Minecraft origin.
        let clash=home.for_location(0x0080_0800,0).unwrap();
        assert_eq!(clash.mc,[0.0,128.0,(-2048*2048) as f64]);
        assert!((clash.mc[2]-home.mc[2]).abs()>=16_384.0);
    }
    #[test]
    fn restore_asks_for_recovery_when_minecraft_is_elsewhere() {
        let home=home();
        let sample=Sample{mc_id:home.mc_id,mc:[131.3,100.0,-45.7],frame:1,world:1,active:1,..Sample::default()};
        // Standing at the interior's zero while Minecraft is still at the outdoor position.
        assert_eq!(home.restore(sample,[0.0;3],0x133C8,0).err(),Some(5));
        let inn=home.for_location(0x133C8,0).unwrap();
        assert!(home.restore(Sample{mc:inn.mc,..sample},[0.0;3],0x133C8,0).is_ok());
        assert_eq!(home.restore(Sample{mc_id:[1;16],..sample},[0.0;3],0x133C8,0).err(),Some(3));
    }
}
