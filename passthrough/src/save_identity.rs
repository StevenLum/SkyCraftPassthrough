//! Save-lineage identity. SKSE owns companion-save disk IO.
use std::{ffi::c_void, sync::Mutex};
use crate::{Role,native::write_log,saved_origin::{self,Origin},position::{Anchor,Sample}};
#[repr(C)]
pub struct Api {
    version:u32,
    unique:unsafe extern "C" fn(u32,u32),
    revert:unsafe extern "C" fn(u32,unsafe extern "C" fn(*const Api)),
    save:unsafe extern "C" fn(u32,unsafe extern "C" fn(*const Api)),
    load:unsafe extern "C" fn(u32,unsafe extern "C" fn(*const Api)),
    form_delete:usize,
    write:unsafe extern "C" fn(u32,u32,*const c_void,u32)->bool,
    open:usize,write_data:usize,
    next:unsafe extern "C" fn(*mut u32,*mut u32,*mut u32)->bool,
    read:unsafe extern "C" fn(*mut c_void,u32)->u32,
    resolve_handle:usize,resolve_form:unsafe extern "C" fn(u32,*mut u32)->bool,
}
#[derive(Default)]
struct State { id:Option<[u8;16]>, invalid:bool, published:bool, origin:Option<Origin>, origin_invalid:bool }
static STATE:Mutex<State>=Mutex::new(State{id:None,invalid:false,published:false,origin:None,origin_invalid:false});
/// Tests stand in for SKSE's load callback: one published lineage and no saved origin yet.
#[cfg(test)]
pub fn publish_for_test(id:[u8;16]) {
    *STATE.lock().unwrap()=State{id:Some(id),published:true,..State::default()};
}
pub fn published_id()->[u8;16] {
    STATE.lock().ok().filter(|s|s.published && !s.invalid).and_then(|s|s.id).unwrap_or([0;16])
}
pub fn origin(sample:Sample,current:[f32;3],cell:u32,worldspace:u32)->Result<Anchor,u32> {
    let mut state=STATE.lock().map_err(|_|6_u32)?;
    if state.invalid || state.origin_invalid || !state.published {return Err(2);}
    if sample.mc_id==[0;16] {return Err(1);}
    if state.id!=Some(sample.save_id) {return Err(2);}
    if let Some(origin)=state.origin {
        let anchor=origin.restore(sample,current,cell,worldspace)?;
        if !origin.same_location(cell,worldspace) {
            // Another building or outdoor world: its own fixed region, nothing saved for it.
            if let Some(place)=origin.for_location(cell,worldspace) {place.log("derived");}
            return Ok(anchor);
        }
        if origin.cell!=cell {
            let _=write_log(Role::Skyrim,format!("event=SCENE_ORIGIN state=outdoor_cell_accepted saved_cell={:08X} current_cell={:08X} worldspace_id={:08X} mapping=preserved",origin.cell,cell,worldspace));
        }
        origin.log("restored");
        return Ok(anchor);
    }
    let origin=Origin::new(sample,current,cell,worldspace);
    if !origin.valid() {return Err(2);}
    origin.log("created");state.origin=Some(origin);
    Ok(Anchor::new(sample,current))
}
static REVISION:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
pub fn recovery_target(sample:Sample,current:[f32;3],cell:u32,space:u32)->Option<[f64;3]> {
    let state=STATE.lock().ok()?;
    if state.invalid || state.origin_invalid || !state.published || state.id!=Some(sample.save_id) {return None;}
    let origin=state.origin?;
    if origin.mc_id!=sample.mc_id {return None;}
    let place=origin.for_location(cell,space)?;
    let target=Anchor::restored(sample,place.mc,place.sky).minecraft(current.map(f64::from));
    target.iter().all(|n|n.is_finite() && n.abs()<=30_000_000.0).then_some(target)
}
pub fn revision()->u64 {REVISION.load(std::sync::atomic::Ordering::Acquire)}
pub fn menu_authorized(sample:Sample,cell:u32,worldspace:u32)->bool {
    STATE.lock().ok().is_some_and(|s|s.published && !s.invalid && !s.origin_invalid
        && s.id==Some(sample.save_id) && s.origin.is_some_and(|o|o.mc_id==sample.mc_id && o.for_location(cell,worldspace).is_some()))
}
const RECORD:u32=u32::from_le_bytes(*b"PBID");
#[repr(C)]
#[derive(Default)]
struct Guid { a:u32,b:u16,c:u16,d:[u8;8] }
#[link(name="ole32")]
unsafe extern "system" { fn CoCreateGuid(guid:*mut Guid)->i32; }
fn log(event:&str,id:Option<[u8;16]>) {
    let text=id.map(|id|id.iter().map(|b|format!("{b:02X}")).collect::<String>()).unwrap_or_else(||"none".into());
    let _=write_log(Role::Skyrim,format!("event=SCENE_SAVE_ID state={event} id={text}"));
}
pub unsafe fn register(api:*const Api,handle:u32)->bool {
    if api.is_null() || unsafe {(*api).version}<4 { return false; }
    unsafe {
        ((*api).unique)(handle,u32::from_le_bytes(*b"PSPT"));
        ((*api).revert)(handle,revert);
        ((*api).save)(handle,save);
        ((*api).load)(handle,load);
    }
    log("registered",None); true
}
unsafe extern "C" fn revert(_: *const Api) {
    REVISION.fetch_add(1,std::sync::atomic::Ordering::AcqRel);
    let _=std::panic::catch_unwind(|| {
        if let Ok(mut state)=STATE.lock() { *state=State::default(); }
        log("reverted",None);
    });
}
unsafe extern "C" fn save(api:*const Api) {
    let _=std::panic::catch_unwind(|| {
        if api.is_null() { return; }
        let Ok(mut state)=STATE.lock() else { log("lock_failed",None);return; };
        if state.invalid { log("write_blocked_invalid_record",None);return; }
        if state.id.is_none() {
            let mut guid=Guid::default();
            if unsafe {CoCreateGuid(&mut guid)}<0 {log("generation_failed",None);return;}
            let mut id=[0;16];
            id[..4].copy_from_slice(&guid.a.to_le_bytes());
            id[4..6].copy_from_slice(&guid.b.to_le_bytes());
            id[6..8].copy_from_slice(&guid.c.to_le_bytes());
            id[8..].copy_from_slice(&guid.d);
            state.id=Some(id);log("created",state.id);
        }
        let id=state.id.unwrap();
        let written=unsafe {((*api).write)(RECORD,1,id.as_ptr().cast(),16)};
        if written {state.published=true;}
        log(if written {"saved"} else {"write_failed"},Some(id));
        if !written {return;}
        if state.origin_invalid {
            // A deliberate rejected marker prevents corrupt data becoming legacy absence.
            let marker=[0_u8];
            let ok=unsafe {((*api).write)(saved_origin::RECORD,1,marker.as_ptr().cast(),1)};
            let _=write_log(Role::Skyrim,format!("event=SCENE_ORIGIN state=invalid_marker_saved success={ok}"));
        } else if let Some(origin)=state.origin {
            let bytes=origin.encode();
            let ok=unsafe {((*api).write)(saved_origin::RECORD,1,bytes.as_ptr().cast(),bytes.len() as u32)};
            origin.log(if ok {"saved"} else {"write_failed"});
        }
    });
}
unsafe extern "C" fn load(api:*const Api) {
    REVISION.fetch_add(1,std::sync::atomic::Ordering::AcqRel);
    let _=std::panic::catch_unwind(|| {
        if api.is_null() { return; }
        let Ok(mut state)=STATE.lock() else { log("lock_failed",None);return; };
        *state=State::default();
        let (mut kind,mut version,mut length)=(0,0,0);
        while unsafe {((*api).next)(&mut kind,&mut version,&mut length)} {
            if kind==saved_origin::RECORD {
                if state.origin.is_some() || version!=1 || length!=saved_origin::SIZE as u32 {
                    state.origin_invalid=true;continue;
                }
                let mut bytes=[0;saved_origin::SIZE];
                if unsafe {((*api).read)(bytes.as_mut_ptr().cast(),length)}!=length {
                    state.origin_invalid=true;continue;
                }
                if let Some(mut origin)=Origin::decode(&bytes) {
                    let mut resolved=0;
                    if !unsafe {((*api).resolve_form)(origin.cell,&mut resolved)} || resolved==0 {
                        state.origin_invalid=true;continue;
                    }
                    origin.cell=resolved;
                    if origin.worldspace!=0 {
                        if !unsafe {((*api).resolve_form)(origin.worldspace,&mut resolved)} || resolved==0 {
                            state.origin_invalid=true;continue;
                        }
                        origin.worldspace=resolved;
                    }
                    state.origin=Some(origin);
                } else {state.origin_invalid=true;}
                continue;
            }
            if kind!=RECORD {continue;}
            if state.id.is_some() || version!=1 || length!=16 {
                state.invalid=true;continue;
            }
            let mut id=[0;16];
            if unsafe {((*api).read)(id.as_mut_ptr().cast(),16)}!=16 || id==[0;16] {
                state.invalid=true;continue;
            }
            state.id=Some(id);
        }
        if state.origin_invalid {
            state.origin=None;
            let _=write_log(Role::Skyrim,"event=SCENE_ORIGIN state=invalid_record");
        } else if let Some(origin)=state.origin {origin.log("loaded");}
        else {let _=write_log(Role::Skyrim,"event=SCENE_ORIGIN state=legacy_absent");}
        if state.invalid {state.id=None;log("invalid_record",None);}
        else {state.published=state.id.is_some();log(if state.id.is_some() {"loaded"} else {"missing_record"},state.id);}
    });
}
