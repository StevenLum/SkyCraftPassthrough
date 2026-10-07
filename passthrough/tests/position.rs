use passthrough_link::{Endpoint, PROTOCOL_VERSION, Role, position::{Sample, MAX_SAMPLE_AGE_MS}};
use std::{thread,time::{Duration,SystemTime,UNIX_EPOCH}};

#[test]
fn fresh_latest_position_requires_handshake_and_expires() {
    let name = format!("positions_{}",SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
    let mut mc = Endpoint::open(&name,Role::Minecraft,PROTOCOL_VERSION).unwrap();
    let mut sky = Endpoint::open(&name,Role::Skyrim,PROTOCOL_VERSION).unwrap();
    let sample = Sample { world:1, frame:1, active:1, partial:0.375,
        mc:[12.375,64.1875,-5.25], ..Sample::default() };
    mc.send_position(sample).unwrap();
    assert!(sky.receive_position().unwrap().is_none());
    mc.poll().unwrap(); sky.poll().unwrap(); mc.poll().unwrap();
    let first = sky.receive_position().unwrap().unwrap();
    assert_eq!(first.mc,sample.mc); assert_eq!(first.partial,0.375);
    assert!(sky.send_position(sample).is_err());
    assert!(mc.receive_position().is_err());
    assert!(mc.send_position(sample).is_err()); // Repeated frame rejected.
    for frame in 2..=5 { mc.send_position(Sample{frame,mc:[frame as f64,64.0,0.0],..sample}).unwrap(); }
    assert_eq!(sky.receive_position().unwrap().unwrap().frame,5); // Latest, not a backlog.
    thread::sleep(Duration::from_millis(MAX_SAMPLE_AGE_MS+50));
    assert!(sky.receive_position().unwrap().is_none());
    mc.send_position(Sample{frame:6,..sample}).unwrap();
    assert!(sky.receive_position().unwrap().is_some());
    mc.send_position(Sample{frame:7,active:0,..sample}).unwrap();
    assert!(sky.receive_position().unwrap().is_none());
    mc.close().unwrap();
    let mut replacement = Endpoint::open(&name,Role::Minecraft,PROTOCOL_VERSION).unwrap();
    replacement.poll().unwrap(); sky.poll().unwrap(); replacement.poll().unwrap();
    assert!(sky.receive_position().unwrap().is_none()); // Old generation cannot move the puppet.
    replacement.send_position(sample).unwrap();
    let fresh = sky.receive_position().unwrap().unwrap();
    assert_ne!(fresh.sender,first.sender);
}
