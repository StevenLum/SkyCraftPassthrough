//! Complete latest RGBA frame, with generation/age checks. Pixels stay in memory.
use std::{io,sync::Mutex,time::Instant};
use crate::{Role,position::Sample,windows::{SharedMemory,now_ms},native::write_log,read_u32,read_u64};
pub const MAX_BYTES:usize=2560*1440*4;
static ALLOWED:Mutex<Option<Sample>>=Mutex::new(None);
pub fn allow(sample:Option<Sample>) { if let Ok(mut value)=ALLOWED.lock() { *value=sample; } }
pub fn allowed()->Option<Sample> {
    let s=(*ALLOWED.lock().ok()?)?; let now=now_ms();
    (s.sent_ms<=now && now-s.sent_ms<=250).then_some(s)
}
pub fn name()->String { std::env::var("PASSTHROUGH_SESSION").unwrap_or_else(|_|"game".into()) }
pub struct Channel { memory:SharedMemory }
pub struct Frame { pub sender:u64,pub world:u64,pub number:u64,pub sent:u64,
    pub width:u32,pub height:u32,pub checksum:u64,pub alpha:u64,pub pixels:Vec<u8> }
pub fn digest(pixels:&[u8])->(u64,u64) {
    let mut hash=0xcbf29ce484222325_u64; let mut alpha=0;
    for pixel in pixels.chunks_exact(4) {
        for byte in pixel { hash=(hash^(*byte as u64)).wrapping_mul(0x100000001b3); }
        if pixel[3]!=0 { alpha+=1; }
    }
    (hash,alpha)
}
impl Channel {
    pub fn open()->io::Result<Self> { Ok(Self{memory:SharedMemory::open_sized(&format!("{}_overlay",name()),80+MAX_BYTES)?}) }
    pub fn publish(&self,sender:u64,world:u64,number:u64,captured:u64,width:u32,height:u32,pixels:&[u8])->io::Result<()> {
        if width==0 || height==0 || width>2560 || height>1440 || pixels.len()!=width as usize*height as usize*4 {
            return Err(io::Error::other("invalid overlay image dimensions"));
        }
        let start=Instant::now(); let (checksum,alpha)=digest(pixels);
        let mut bytes=vec![0;80+pixels.len()]; bytes[..8].copy_from_slice(b"PTIMAGE5");
        for (at,value) in [(8,sender),(16,world),(24,number),(32,captured),(56,checksum),(64,alpha)] {
            bytes[at..at+8].copy_from_slice(&value.to_le_bytes());
        }
        for (at,value) in [(40,width),(44,height),(48,pixels.len() as u32),(52,1_u32)] {
            bytes[at..at+4].copy_from_slice(&value.to_le_bytes());
        }
        bytes[80..].copy_from_slice(pixels); self.memory.blob(&mut bytes,true)?;
        write_log(Role::Minecraft,format!("event=OVERLAY_SEND sender={sender} world={world} frame={number} width={width} height={height} bytes={} checksum={checksum} alpha_pixels={alpha} transfer_us={}",pixels.len(),start.elapsed().as_micros()))
    }
    pub fn receive(&self)->io::Result<Option<Frame>> {
        let Some(permit)=allowed() else { return Ok(None); };
        let b=self.memory.read_overlay()?;
        if b.is_empty() { return Ok(None); }
        if &b[..8]!=b"PTIMAGE5" { return Err(io::Error::other("overlay header mismatch")); }
        let width=read_u32(&b,40); let height=read_u32(&b,44);
        if width==0 || height==0 || width>2560 || height>1440 || b.len()!=80+width as usize*height as usize*4 || read_u32(&b,52)!=1 {
            return Err(io::Error::other("invalid received overlay dimensions/format"));
        }
        let sender=read_u64(&b,8); let world=read_u64(&b,16); let sent=read_u64(&b,32); let now=now_ms();
        if sender!=permit.sender || world!=permit.world || sent>now || now-sent>500 { return Ok(None); }
        Ok(Some(Frame{sender,world,number:read_u64(&b,24),sent,width,height,
            checksum:read_u64(&b,56),alpha:read_u64(&b,64),pixels:b[80..].to_vec()}))
    }
}
