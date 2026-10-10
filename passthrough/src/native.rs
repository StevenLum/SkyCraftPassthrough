//! Stable C entry points used by Fabric's Java foreign-function API.
use crate::{Endpoint, Role, State, PROTOCOL_VERSION, position::Sample};
use std::{cell::RefCell, fs::{self, File}, io::{self, Write}, path::PathBuf,
    sync::{Mutex, OnceLock}};

static MC_LOG: OnceLock<Mutex<File>> = OnceLock::new();
static SKY_LOG: OnceLock<Mutex<File>> = OnceLock::new();

pub(crate) fn write_log(role: Role, message: impl AsRef<str>) -> io::Result<()> {
    let slot = match role { Role::Minecraft => &MC_LOG, Role::Skyrim => &SKY_LOG };
    if slot.get().is_none() {
        let fixture = cfg!(test) || std::env::var("PASSTHROUGH_TEST_MODE").as_deref() == Ok("1");
        let folder = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(if fixture { "../logs/step2-fixtures" } else { "../logs/step2" });
        fs::create_dir_all(&folder)?;
        let name = match role { Role::Minecraft => "minecraft", Role::Skyrim => "skyrim" };
        let file = File::create(folder.join(format!("{name}-{}-{}.log", std::process::id(), crate::windows::now_ms())))?;
        let _ = slot.set(Mutex::new(file));
    }
    let mut file = slot.get().unwrap().lock().map_err(|_| io::Error::other("log lock poisoned"))?;
    let now=crate::windows::now_ms();
    for line in message.as_ref().lines() {
        writeln!(file, "time_ms={now} pid={} {line}", std::process::id())?;
    }
    file.flush()
}

pub(crate) struct GameLink { pub endpoint: Endpoint, role: Role, last_state: Option<State> }
impl GameLink {
    pub fn open(role: Role) -> io::Result<Self> {
        let session = std::env::var("PASSTHROUGH_SESSION").unwrap_or_else(|_| "game".into());
        Self::open_named(role, &session)
    }
    pub fn open_named(role: Role, session: &str) -> io::Result<Self> {
        let endpoint = Endpoint::open(session, role, PROTOCOL_VERSION)?;
        write_log(role, format!("event=NATIVE_READY protocol={PROTOCOL_VERSION}"))?;
        Ok(Self { endpoint, role, last_state: None })
    }
    pub fn poll(&mut self) -> io::Result<()> {
        let state = self.endpoint.poll()?;
        if self.last_state != Some(state.state) {
            write_log(self.role, format!("event=LINK state={:?} peer_pid={}", state.state, state.peer_pid))?;
            self.last_state = Some(state.state);
        }
        Ok(())
    }
}

thread_local! { static MENU_REQUEST:RefCell<(u64,u64,u64)>=const {RefCell::new((0,0,0))}; }
thread_local! { static INTERACT_REQUEST:RefCell<(u64,u64,u64)>=const {RefCell::new((0,0,0))}; }
thread_local! { static MINECRAFT: RefCell<Option<GameLink>> = const { RefCell::new(None) }; }
thread_local! { static LAST_WORLD: RefCell<Option<(u64,u64,u64)>> = const { RefCell::new(None) }; }
thread_local! { static OVERLAY: RefCell<Option<crate::overlay::Channel>> = const { RefCell::new(None) }; }

#[unsafe(no_mangle)]
pub extern "C" fn pt_mc_overlay_ready()->u64 {
    let result=std::panic::catch_unwind(||MINECRAFT.with(|state| {
        state.borrow().as_ref().is_some_and(|link|link.endpoint.overlay_ready().unwrap_or(false))
    }));
    if matches!(result,Ok(true)) { crate::windows::now_ms() } else { 0 }
}
thread_local! { static SCENE:RefCell<Option<(crate::scene::Mailbox,crate::scene::Mailbox)>>=const { RefCell::new(None) }; }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pt_mc_scene(bytes:*const u8,length:u32)->i32 {
    let result=std::panic::catch_unwind(||->io::Result<i32> {
        if bytes.is_null() || length<128 || length as usize>128+crate::scene::ATLAS_MAX { return Err(io::Error::other("invalid scene pointer/length")); }
        let sender=MINECRAFT.with(|s|s.borrow().as_ref().map(|l|l.endpoint.generation()));
        let Some(sender)=sender else { return Ok(0); };
        let p=crate::scene::Packet{bytes:unsafe { std::slice::from_raw_parts(bytes,length as usize) }.to_vec()};
        p.validate()?;
        SCENE.with(|state| {
            let mut state=state.borrow_mut();
            if state.is_none() { *state=Some((crate::scene::Mailbox::open(false)?,crate::scene::Mailbox::open(true)?)); }
            let (mesh,atlas)=state.as_ref().unwrap();
            if p.kind()==1 { mesh.send(p,sender)?; } else { atlas.send(p,sender)?; }
            Ok(1)
        })
    });
    match result {
        Ok(Ok(n))=>n,
        Ok(Err(e)) if e.kind()==io::ErrorKind::TimedOut=>0,
        failure=> { let _=write_log(Role::Minecraft,format!("event=SCENE_ERROR detail={failure:?}")); -1 }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pt_mc_overlay(world:u64,frame:u64,captured:u64,width:u32,height:u32,pixels:*const u8,length:u32)->i32 {
    let result=std::panic::catch_unwind(||->io::Result<i32> {
        if pixels.is_null() || length as usize>crate::overlay::MAX_BYTES { return Err(io::Error::other("invalid overlay buffer")); }
        let now=crate::windows::now_ms();
        if captured>now || now-captured>500 { return Ok(0); }
        let sender=MINECRAFT.with(|state|state.borrow().as_ref().map(|l|l.endpoint.generation()));
        let Some(sender)=sender else { return Ok(0); };
        OVERLAY.with(|state|->io::Result<i32> {
            let mut state=state.borrow_mut();
            if state.is_none() { *state=Some(crate::overlay::Channel::open()?); }
            state.as_ref().unwrap().publish(sender,world,frame,captured,width,height,
                unsafe { std::slice::from_raw_parts(pixels,length as usize) })?;
            Ok(1)
        })
    });
    match result {
        Ok(Ok(n))=>n,
        Ok(Err(e)) if e.kind()==io::ErrorKind::TimedOut=>0,
        failure=> { let _=write_log(Role::Minecraft,format!("event=OVERLAY_ERROR detail={failure:?}")); -1 }
    }
}

/// Called only on the render thread; Java publishes an immutable copy to physics threads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pt_mc_world(destination:*mut u8,capacity:u32)->i32 {
    mc_guard(|| MINECRAFT.with(|state| {
        if destination.is_null() || capacity as usize!=crate::world::SIZE { return Err(io::Error::other("invalid world destination")); }
        let mut state=state.borrow_mut();
        let link=state.as_mut().ok_or_else(||io::Error::other("world receiver not initialized"))?;
        let Some(snapshot)=link.endpoint.receive_world()? else { return Ok(0); };
        let key=(snapshot.sender,snapshot.epoch,snapshot.sequence);
        LAST_WORLD.with(|last| -> io::Result<()> {
            if *last.borrow()!=Some(key) { snapshot.log(Role::Minecraft,"RECV")?; *last.borrow_mut()=Some(key); }
            Ok(())
        })?;
        let mut bytes=snapshot.encode()?;
        let remaining=crate::world::MAX_AGE_MS.saturating_sub(crate::windows::now_ms().saturating_sub(snapshot.sent_ms));
        bytes[72..80].copy_from_slice(&remaining.to_le_bytes());
        unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(),destination,bytes.len()); }
        Ok(1)
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn pt_mc_init() -> i32 {
    mc_guard(|| MINECRAFT.with(|state| {
        let mut state = state.borrow_mut();
        if state.is_none() { *state = Some(GameLink::open(Role::Minecraft)?); }
        Ok(1)
    }))
}

#[unsafe(no_mangle)]
pub extern "C" fn pt_mc_frame14(world: u64, frame: u64, partial: f64,
    x: f64, y: f64, z: f64, yaw:f64, pitch:f64, active: u32, id_lo:u64, id_hi:u64, mc_lo:u64, mc_hi:u64, interact_seq:u64, menu_seq:u64, menu_action:u32, menu_context:u64, recovery_ack:u64,damage_epoch:u64,damage_ack:u64,damage_ready:u32,attack_seq:u64,attack_handle:u32,attack_damage:f32,cursor_context:u64,cursor_x:f32,cursor_y:f32,death_seq:u64,death_age_ms:u32) -> i32
{
    mc_guard(|| MINECRAFT.with(|state| {
        let mut state = state.borrow_mut();
        let link = state.as_mut().ok_or_else(|| io::Error::other("Minecraft bridge not initialized on this thread"))?;
        link.poll()?;
        let mut save_id=[0;16]; save_id[..8].copy_from_slice(&id_lo.to_le_bytes()); save_id[8..].copy_from_slice(&id_hi.to_le_bytes());
        let mut mc_id=[0;16]; mc_id[..8].copy_from_slice(&mc_lo.to_le_bytes()); mc_id[8..].copy_from_slice(&mc_hi.to_le_bytes());
        let interact_seq=if active==1 {interact_seq} else {0};
        let interact_ms=INTERACT_REQUEST.with(|request| {
            let mut request=request.borrow_mut();
            if request.0!=world || request.1!=interact_seq {
                *request=(world,interact_seq,if interact_seq==0 {0} else {crate::windows::now_ms()});
            }
            request.2
        });
        let menu_seq=if active==1 {menu_seq} else {0};
        let menu_ms=MENU_REQUEST.with(|request| {
            let mut request=request.borrow_mut();
            if request.0!=world || request.1!=menu_seq {
                *request=(world,menu_seq,if menu_seq==0 {0} else {crate::windows::now_ms()});
            }
            request.2
        });
        let attack_seq=if active==1 {attack_seq} else {0};
        let attack_ms=ATTACK_REQUEST.with(|r| {
            let mut r=r.borrow_mut();
            if r.0!=world || r.1!=attack_seq {*r=(world,attack_seq,if attack_seq==0 {0} else {crate::windows::now_ms()});}
            r.2
        });
        // Pointer motion only counts in an active frame; never forward a nonfinite total.
        let (cursor_context,cursor)=if active==1 && cursor_x.is_finite() && cursor_y.is_finite() {(cursor_context,[cursor_x,cursor_y])} else {(0,[0.0;2])};
        // Java reports how long ago the player died; both processes share this boot-relative clock.
        let death_seq=if active==1 {death_seq} else {0};
        let death_ms=if death_seq==0 {0} else {crate::windows::now_ms().saturating_sub(death_age_ms as u64).max(1)};
        let sample = link.endpoint.send_position(Sample { death_seq,death_ms,cursor_context,cursor,attack_seq,attack_handle,attack_damage,attack_ms,world, frame, partial, mc: [x,y,z], yaw, pitch, active, save_id, mc_id, interact_seq, interact_ms, menu_seq,menu_action,menu_context,menu_ms,damage_epoch,damage_ack,damage_ready:if active==1 {damage_ready} else {0},recovery_ack:if active==1 {recovery_ack} else {0}, ..Sample::default() })?;
        write_log(Role::Minecraft, format!("event=SEND active={active} {}", sample.log_fields()))?;
        Ok(1)
    }))
}

fn mc_guard(action: impl FnOnce() -> io::Result<i32>) -> i32 {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(action)) {
        Ok(Ok(value)) => value,
        Ok(Err(error)) if error.kind() == io::ErrorKind::TimedOut => 0, // Skip a contended frame.
        other => {
            let message = match other { Ok(Err(e)) => e.to_string(), _ => "native bridge panic".into() };
            let _ = write_log(Role::Minecraft, format!("event=ERROR message={message}"));
            MINECRAFT.with(|state| { *state.borrow_mut() = None; });
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pt_mc_menu(destination:*mut u8)->i32 {
    mc_guard(|| MINECRAFT.with(|state| {
        if destination.is_null() {return Err(io::Error::other("null menu status"));}
        let state=state.borrow();
        let link=state.as_ref().ok_or_else(||io::Error::other("bridge not initialized"))?;
        let status=link.endpoint.menu_status()?;
        let mut bytes=[0;32];
        bytes[..4].copy_from_slice(&(status.active as u32).to_le_bytes());
        bytes[8..16].copy_from_slice(&status.context.to_le_bytes());
        bytes[16..24].copy_from_slice(&status.world.to_le_bytes());
        unsafe {std::ptr::copy_nonoverlapping(bytes.as_ptr(),destination,32);}
        Ok(1)
    }))
}

thread_local! {static ATTACK_REQUEST:RefCell<(u64,u64,u64)>=const {RefCell::new((0,0,0))};}
