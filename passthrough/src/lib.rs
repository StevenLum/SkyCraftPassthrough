//! Local connection and per-render-frame player positions.

#[cfg(not(all(windows, target_arch = "x86_64", target_env = "msvc")))]
compile_error!("Build with Rust stable for x86_64-pc-windows-msvc.");

mod windows;
pub mod position;
mod native;
mod skse;
mod world;
mod world_export;
mod look;

use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use windows::SharedMemory;

pub const PROTOCOL_VERSION: u32 = 4;
pub const HEARTBEAT_TIMEOUT_MS: u64 = 2_000;
pub const POLL_INTERVAL_MS: u64 = 100;
const RECORD_SIZE: usize = 256;
const MAGIC: &[u8; 8] = b"PASSLNK2";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Skyrim,
    Minecraft,
}

impl Role {
    fn index(self) -> usize {
        match self {
            Self::Skyrim => 0,
            Self::Minecraft => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Waiting,
    Handshaking,
    Connected,
    TimedOut,
    Incompatible,
}

#[derive(Clone, Copy, Debug)]
pub struct Observation {
    pub state: State,
    pub peer_pid: u32,
    pub peer_session: u64,
    pub peer_heartbeat: u64,
    pub peer_age_ms: u64,
}

#[derive(Clone, Copy, Default)]
struct Slot {
    pid: u32,
    protocol: u32,
    session: u64,
    heartbeat_ms: u64,
    acknowledged_session: u64,
    heartbeat_count: u64,
}

#[derive(Default)]
struct Record {
    slots: [Slot; 2],
    position: position::Sample,
}

// Fixed byte layout, never a Rust struct copied across processes:
// 0..8 magic; 8..12 layout version; 12..16 size; 16..32 reserved.
// Skyrim at 32, Minecraft at 80: each 48-byte slot contains two u32s,
// four u64s, then 8 reserved bytes. Every integer is little-endian.
impl Record {
    fn decode(bytes: &[u8; RECORD_SIZE]) -> io::Result<Self> {
        if &bytes[..8] != MAGIC || read_u32(bytes, 8) != 2
            || read_u32(bytes, 12) != RECORD_SIZE as u32
        {
            return Err(io::Error::other("invalid shared-memory header; stop both peers and restart"));
        }
        let mut record = Self::default();
        for (index, slot) in record.slots.iter_mut().enumerate() {
            let base = 32 + index * 48;
            *slot = Slot {
                pid: read_u32(bytes, base),
                protocol: read_u32(bytes, base + 4),
                session: read_u64(bytes, base + 8),
                heartbeat_ms: read_u64(bytes, base + 16),
                acknowledged_session: read_u64(bytes, base + 24),
                heartbeat_count: read_u64(bytes, base + 32),
            };
        }
        record.position = position::Sample::decode(bytes);
        Ok(record)
    }

    fn encode(&self) -> [u8; RECORD_SIZE] {
        let mut bytes = [0; RECORD_SIZE];
        bytes[..8].copy_from_slice(MAGIC);
        bytes[8..12].copy_from_slice(&2_u32.to_le_bytes());
        bytes[12..16].copy_from_slice(&(RECORD_SIZE as u32).to_le_bytes());
        for (index, slot) in self.slots.iter().enumerate() {
            let base = 32 + index * 48;
            bytes[base..base + 4].copy_from_slice(&slot.pid.to_le_bytes());
            bytes[base + 4..base + 8].copy_from_slice(&slot.protocol.to_le_bytes());
            for (offset, value) in [slot.session, slot.heartbeat_ms,
                slot.acknowledged_session, slot.heartbeat_count].iter().enumerate()
            {
                let start = base + 8 + offset * 8;
                bytes[start..start + 8].copy_from_slice(&value.to_le_bytes());
            }
        }
        self.position.encode(&mut bytes);
        bytes
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn read_u64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}

/// One role in a pair. Call `poll` regularly from a dedicated worker thread.
/// Only `Connected` permits future gameplay exchange; all other states are idle.
pub struct Endpoint {
    memory: SharedMemory,
    world_memory: SharedMemory,
    role: Role,
    session: u64,
    closed: bool,
}

impl Endpoint {
    pub fn open(name: &str, role: Role, protocol: u32) -> io::Result<Self> {
        if name.is_empty() || name.len() > 64
            || !name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
        {
            return Err(io::Error::other("session name must contain 1..64 letters, digits, '-' or '_'"));
        }
        if protocol == 0 {
            return Err(io::Error::other("protocol version must be greater than zero"));
        }
        let memory = SharedMemory::open(name)?;
        let world_memory = SharedMemory::open_sized(&format!("{name}_world"), world::SIZE)?;
        // A generation identifier, not an authentication secret. It changes on restart
        // so an acknowledgement from a previous process cannot complete a handshake.
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(io::Error::other)?.as_nanos() as u64;
        let session = (nanos ^ ((std::process::id() as u64) << 32)).max(1);
        memory.update(|record| {
            let slot = &mut record.slots[role.index()];
            // A stalled but living process still owns its role. Never steal its slot.
            if slot.pid != 0 && windows::process_alive(slot.pid)? {
                return Err(io::Error::other(format!("duplicate role {role:?}: process {} still owns it", slot.pid)));
            }
            *slot = Slot {
                pid: std::process::id(), protocol, session,
                heartbeat_ms: windows::now_ms(),
                ..Slot::default()
            };
            Ok(())
        })?;
        Ok(Self { memory, world_memory, role, session, closed: false })
    }

    pub fn poll(&mut self) -> io::Result<Observation> {
        if self.closed {
            return Err(io::Error::other("endpoint is closed"));
        }
        self.memory.update(|record| {
            let peer = record.slots[1 - self.role.index()];
            let own = &mut record.slots[self.role.index()];
            if own.session != self.session || own.pid != std::process::id() {
                return Err(io::Error::other("lost ownership of shared-memory slot"));
            }
            let now = windows::now_ms();
            own.heartbeat_ms = now;
            own.heartbeat_count = own.heartbeat_count.saturating_add(1);
            let age = now.saturating_sub(peer.heartbeat_ms);
            let state = if peer.pid == 0 || peer.session == 0 {
                own.acknowledged_session = 0;
                State::Waiting
            } else if peer.heartbeat_ms > now || age > HEARTBEAT_TIMEOUT_MS {
                own.acknowledged_session = 0;
                State::TimedOut
            } else if peer.protocol != own.protocol {
                own.acknowledged_session = 0;
                State::Incompatible
            } else {
                own.acknowledged_session = peer.session;
                if peer.acknowledged_session == self.session {
                    State::Connected
                } else {
                    State::Handshaking
                }
            };
            Ok(Observation {
                state, peer_pid: peer.pid, peer_session: peer.session,
                peer_heartbeat: peer.heartbeat_count,
                peer_age_ms: if peer.pid == 0 { 0 } else { age },
            })
        })
    }

    pub fn close(&mut self) -> io::Result<()> {
        if self.closed { return Ok(()); }
        self.memory.update(|record| {
            let own = &mut record.slots[self.role.index()];
            if own.session == self.session && own.pid == std::process::id() {
                *own = Slot::default();
            }
            Ok(())
        })?;
        self.closed = true;
        Ok(())
    }

    /// Called once per rendered Minecraft frame, including inactive frames.
    pub fn send_position(&mut self, mut sample: position::Sample) -> io::Result<position::Sample> {
        if self.closed || self.role != Role::Minecraft || !sample.valid() {
            return Err(io::Error::other("invalid position or sender role"));
        }
        sample.sender = self.session;
        sample.sent_ms = windows::now_ms();
        self.memory.update(|record| {
            if record.slots[1].session != self.session {
                return Err(io::Error::other("position sender lost ownership"));
            }
            if record.position.sender == self.session && sample.frame <= record.position.frame {
                return Err(io::Error::other("frame numbers must increase"));
            }
            record.position = sample;
            Ok(sample)
        })
    }

    /// Latest complete sample; no second interpolation is performed here.
    pub fn receive_position(&mut self) -> io::Result<Option<position::Sample>> {
        if self.closed || self.role != Role::Skyrim {
            return Err(io::Error::other("invalid position receiver role"));
        }
        self.memory.update(|record| {
            let own = record.slots[0];
            let peer = record.slots[1];
            let sample = record.position;
            let now = windows::now_ms();
            let ready = own.session == self.session && peer.pid != 0
                && own.acknowledged_session == peer.session
                && peer.acknowledged_session == self.session
                && own.protocol == PROTOCOL_VERSION && peer.protocol == PROTOCOL_VERSION
                && sample.sender == peer.session && sample.active == 1 && sample.valid()
                && sample.sent_ms <= now && now - sample.sent_ms <= position::MAX_SAMPLE_AGE_MS
                && peer.heartbeat_ms <= now && now - peer.heartbeat_ms <= HEARTBEAT_TIMEOUT_MS;
            Ok(ready.then_some(sample))
        })
    }

    pub(crate) fn send_world(&mut self, mut snapshot: world::Snapshot) -> io::Result<world::Snapshot> {
        if self.closed || self.role != Role::Skyrim { return Err(io::Error::other("invalid world sender")); }
        snapshot.sender = self.session;
        snapshot.sent_ms = windows::now_ms();
        let mut bytes = snapshot.encode()?;
        self.world_memory.blob(&mut bytes,true)?;
        Ok(snapshot)
    }

    pub(crate) fn receive_world(&mut self) -> io::Result<Option<world::Snapshot>> {
        if self.closed || self.role != Role::Minecraft { return Err(io::Error::other("invalid world receiver")); }
        let ready = self.memory.update(|record| {
            let own=record.slots[1]; let peer=record.slots[0]; let p=record.position;
            let now=windows::now_ms();
            Ok((own.session==self.session && peer.pid!=0 && own.protocol==PROTOCOL_VERSION
                && peer.protocol==PROTOCOL_VERSION && own.acknowledged_session==peer.session
                && peer.acknowledged_session==own.session && p.sender==self.session && p.active==1
                && p.sent_ms<=now && now-p.sent_ms<=position::MAX_SAMPLE_AGE_MS
                && peer.heartbeat_ms<=now && now-peer.heartbeat_ms<=HEARTBEAT_TIMEOUT_MS)
                .then_some((peer.session,p.world)))
        })?;
        let Some((sender,world))=ready else { return Ok(None); };
        let mut bytes=vec![0;world::SIZE]; self.world_memory.blob(&mut bytes,false)?;
        let snapshot=world::Snapshot::decode(&bytes)?;
        let now=windows::now_ms();
        Ok(snapshot.filter(|s| s.active && s.sender==sender && s.recipient==self.session
            && s.world==world && s.sent_ms<=now && now-s.sent_ms<=world::MAX_AGE_MS))
    }
}

impl Drop for Endpoint {
    fn drop(&mut self) { let _ = self.close(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_bytes_have_documented_offsets() {
        let mut record = Record::default();
        record.slots[1] = Slot { pid: 0x11223344, protocol: 7,
            session: 8, heartbeat_ms: 9, acknowledged_session: 10, heartbeat_count: 11 };
        let bytes = record.encode();
        assert_eq!(&bytes[0..16], b"PASSLNK2\x02\0\0\0\0\x01\0\0");
        assert_eq!(&bytes[80..84], &[0x44, 0x33, 0x22, 0x11]);
        assert_eq!(read_u64(&bytes, 104), 10);
        assert_eq!(read_u64(&bytes, 112), 11);
        let decoded = Record::decode(&bytes).unwrap();
        assert_eq!(decoded.slots[0].pid, 0);
        assert_eq!(decoded.slots[1].session, 8);
    }

    #[test]
    fn corrupt_or_unknown_layout_is_rejected() {
        for offset in [0, 8, 12] {
            let mut bytes = Record::default().encode();
            bytes[offset] ^= 0xff;
            assert!(Record::decode(&bytes).is_err());
        }
    }
}
