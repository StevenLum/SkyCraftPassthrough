//! Furniture classification, intentional seating, and recovery of an unowned seated save.
use std::ptr;
type Object=*mut u8;
unsafe fn read<T:Copy>(p:Object,at:usize)->T { unsafe {ptr::read_unaligned(p.add(at).cast())} }

/// A seat: furniture (CommonLib FormTypes.h 0x28) that opens no menu. Using one makes
/// Skyrim's character sit down, lean or work at it, and stay there. A workbench (bench type
/// other than 0) opens a crafting menu and a bed (marker flag bit 31, "can sleep") the
/// sleep menu; those two are not seats.
pub fn is_seat(base_type:u8,bench:u8,markers:u32)->bool {
    base_type==0x28 && bench==0 && markers&(1<<31)==0
}
/// The record type of a loaded reference's base object, and for furniture its bench type
/// and marker flags. CommonLib TESObjectREFR.h: base object at 0x40; TESForm.h: form type
/// at 0x1A; TESFurniture.h: bench type at 0xE0, marker flags at 0xE4. The two furniture
/// fields are only read once the type says the record is furniture.
pub unsafe fn furniture(target:Object)->(u8,u8,u32) {
    if target.is_null() {return (0,0,0);}
    let base:Object=unsafe {read(target,0x40)};
    if base.is_null() {return (0,0,0);}
    let base_type:u8=unsafe {read(base,0x1A)};
    if base_type!=0x28 {return (base_type,0,0);}
    (base_type,unsafe {read(base,0xE0)},unsafe {read(base,0xE4)})
}
/// Whether a loaded reference is a seat.
pub unsafe fn target_is_seat(target:Object)->bool {
    let (base_type,bench,markers)=unsafe {furniture(target)};
    is_seat(base_type,bench,markers)
}
/// Ordinary sit-capable furniture; exclude lean/work markers without the can-sit bit.
pub fn is_chair(base_type:u8,bench:u8,markers:u32)->bool {
    is_seat(base_type,bench,markers) && markers&(1<<30)!=0
}
pub unsafe fn target_is_chair(target:Object)->bool {
    let (kind,bench,markers)=unsafe {furniture(target)};is_chair(kind,bench,markers)
}

/// The sit state in a character's state word (CommonLib ActorState.h: bits 14 to 17 of the
/// word at Actor+0xC8, whose bits 21 to 24 are the life state the bridge already reads).
/// 0 standing, 1 and 2 sitting down, 3 seated or riding, 4 standing up, 5 to 8 the same
/// four for sleeping.
pub fn sit_state(word:u32)->u32 {(word>>14)&0xF}
/// The sit state of a loaded character.
pub unsafe fn of(actor:Object)->u32 {sit_state(unsafe {read(actor,0xC8)})}
pub fn label(state:u32)->&'static str {
    match state {
        0=>"standing",1=>"sitting_down",2=>"waiting_to_sit",3=>"seated_or_riding",4=>"standing_up",
        5=>"lying_down",6=>"waiting_to_sleep",7=>"asleep",8=>"waking",_=>"unknown",
    }
}

/// Intent belongs to the paired Minecraft session/save and survives ordinary suspension.
#[derive(Clone,Copy,PartialEq,Eq)]
struct Owner(u64,u64,[u8;16],[u8;16]);
fn owner(s:crate::position::Sample)->Owner {Owner(s.sender,s.world,s.save_id,s.mc_id)}
#[derive(Debug,PartialEq,Eq)]
pub enum SitDecision {Idle,Held,Stand(u8),Align,Failed,GaveUp}
#[derive(Default)]
pub struct Intent {
    owner:Option<Owner>,started:u64,entered:bool,leaving:bool,native:bool,
    interact:u64,attempt_ms:Option<u64>,tries:u8,gave_up:bool,entry:Option<[f64;3]>,
}
impl Intent {
    pub fn reset(&mut self) {*self=Self::default();}
    pub fn active(&self)->bool {self.owner.is_some()}
    pub fn arm(&mut self,s:crate::position::Sample,now:u64) {
        *self=Self{owner:Some(owner(s)),started:now,interact:s.interact_seq,
            entry:s.mc.iter().all(|v|v.is_finite() && v.abs()<30_000_000.0).then_some(s.mc),..Self::default()};
    }
    /// Bed/crafting animations finish in Skyrim; only align after it is standing.
    pub fn arm_native(&mut self,s:crate::position::Sample,now:u64) {
        self.arm(s,now);self.native=true;self.entered=true;self.entry=None;
    }
    /// Chair-only return point; never belongs to another pair or native bed/crafting.
    pub fn entry(&self,s:crate::position::Sample)->Option<[f64;3]> {
        if self.owner==Some(owner(s)) && !self.native {self.entry} else {None}
    }
    pub fn poll(&mut self,s:crate::position::Sample,state:u32,now:u64)->SitDecision {
        if self.owner.is_none() {return SitDecision::Idle;}
        if self.owner!=Some(owner(s)) || now<self.started || self.attempt_ms.is_some_and(|at|now<at) {self.reset();return SitDecision::Idle;}
        if s.active!=1 {return SitDecision::Held;}
        if self.native {return if state==0 {SitDecision::Align} else {SitDecision::Held};}
        if state==0 {
            if self.entered {return SitDecision::Align;}
            if now-self.started>=4000 {self.reset();return SitDecision::Failed;}
            return SitDecision::Held;
        }
        if !(1..=4).contains(&state) {self.reset();return SitDecision::Failed;}
        self.entered=true;
        let fresh_e=s.interact_seq!=0 && s.interact_seq!=self.interact && s.interact_ms!=0
            && s.interact_ms<=now && now-s.interact_ms<=250;
        if s.interact_seq!=0 {self.interact=s.interact_seq;}
        if s.controls&crate::position::CONTROL_MOVE!=0 || fresh_e {self.leaving=true;}
        if !self.leaving || self.gave_up || self.attempt_ms.is_some_and(|at|now-at<2000) {return SitDecision::Held;}
        if self.tries==5 {self.gave_up=true;return SitDecision::GaveUp;}
        self.tries+=1;self.attempt_ms=Some(now);SitDecision::Stand(self.tries)
    }
}

/// Requests from the seated-character timer; nothing here calls the game.
#[derive(Debug,PartialEq,Eq)]
pub enum Decision { Stand(u8), GaveUp }
#[derive(Default)]
pub struct Watch {
    last_ms:Option<u64>, since_ms:Option<u64>, attempt_ms:Option<u64>, tries:u8, gave_up:bool,
}
impl Watch {
    pub fn reset(&mut self) {*self=Self::default();}
    /// Called only while the character is being steered. A long gap is a menu/load,
    /// not four seconds spent stuck. Any nonzero sit state continues the same watch.
    pub fn look(&mut self,state:u32,now_ms:u64)->Option<Decision> {
        if state==0 {self.reset();return None;}
        if self.last_ms.is_some_and(|last|now_ms<last || now_ms-last>500) {self.reset();}
        self.last_ms=Some(now_ms);
        let since=*self.since_ms.get_or_insert(now_ms);
        if now_ms-since<4000 || self.gave_up {return None;}
        if self.attempt_ms.is_some_and(|last|now_ms-last<2000) {return None;}
        if self.tries==5 {self.gave_up=true;return Some(Decision::GaveUp);}
        self.tries+=1;self.attempt_ms=Some(now_ms);
        Some(Decision::Stand(self.tries))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn chair_entry_survives_a_hold_but_never_another_owner_or_native_furniture() {
        let s=crate::position::Sample{sender:7,world:2,mc:[10.0,128.125,20.0],save_id:[3;16],mc_id:[4;16],active:1,..Default::default()};
        let mut intent=super::Intent::default();intent.arm(s,100);
        assert_eq!(intent.entry(s),Some(s.mc));
        assert_eq!(intent.poll(s,3,200),super::SitDecision::Held);
        assert_eq!(intent.entry(crate::position::Sample{mc:[0.0;3],..s}),Some(s.mc));
        for other in [crate::position::Sample{sender:8,..s},crate::position::Sample{world:3,..s},
            crate::position::Sample{save_id:[5;16],..s},crate::position::Sample{mc_id:[6;16],..s}] {
            assert_eq!(intent.entry(other),None);
        }
        intent.arm_native(s,300);assert_eq!(intent.entry(s),None);
        intent.arm(crate::position::Sample{mc:[f64::NAN,0.0,0.0],..s},400);assert_eq!(intent.entry(s),None);
        intent.arm(s,500);intent.reset();assert_eq!(intent.entry(s),None);
    }

    use super::*;
    fn seated_sample()->crate::position::Sample {
        crate::position::Sample{sender:1,world:2,active:1,save_id:[3;16],mc_id:[4;16],
            interact_seq:7,interact_ms:1000,..crate::position::Sample::default()}
    }
    #[test]
    fn native_furniture_owns_its_animation_until_standing() {
        let s=seated_sample();let mut seat=Intent::default();seat.arm_native(s,1000);
        for state in [1,3,4,5,6,7,8] {assert_eq!(seat.poll(s,state,2000),SitDecision::Held);}
        assert_eq!(seat.poll(s,0,2100),SitDecision::Align);
        assert_eq!(seat.poll(crate::position::Sample{world:99,..s},0,2200),SitDecision::Idle);
    }
    #[test]
    fn an_intentional_seat_last_as_long_as_requested_and_the_entering_e_is_not_replayed() {
        let s=seated_sample();let mut seat=Intent::default();seat.arm(s,1000);
        assert_eq!(seat.poll(s,0,1000),SitDecision::Held);
        assert_eq!(seat.poll(s,1,1100),SitDecision::Held);
        assert_eq!(seat.poll(s,3,5000),SitDecision::Held);
        assert_eq!(seat.poll(s,3,3_600_000),SitDecision::Held);
        assert!(seat.active());
        // Ordinary suspension does not remove ownership; returning without keys stays seated.
        assert_eq!(seat.poll(crate::position::Sample{active:0,controls:1,..s},3,3_600_100),SitDecision::Held);
        assert_eq!(seat.poll(s,3,3_600_200),SitDecision::Held);
    }
    #[test]
    fn either_wasd_or_a_fresh_e_stands_and_requires_alignment_before_walking() {
        let s=seated_sample();
        for request in [crate::position::Sample{controls:1,..s},
            crate::position::Sample{interact_seq:8,interact_ms:2000,..s}] {
            let mut seat=Intent::default();seat.arm(s,1000);
            assert_eq!(seat.poll(s,3,1500),SitDecision::Held);
            assert_eq!(seat.poll(request,3,2000),SitDecision::Stand(1));
            assert_eq!(seat.poll(request,4,2100),SitDecision::Held);
            assert_eq!(seat.poll(request,0,2200),SitDecision::Align);
            assert_eq!(seat.poll(s,0,3000),SitDecision::Align);
            seat.reset();assert_eq!(seat.poll(request,0,3001),SitDecision::Idle);
        }
    }
    #[test]
    fn stale_future_and_repeated_e_are_not_exit_requests() {
        let s=seated_sample();let mut seat=Intent::default();seat.arm(s,1000);
        for (seq,ms) in [(7,2000),(8,1000),(9,0),(10,2001)] {
            assert_eq!(seat.poll(crate::position::Sample{interact_seq:seq,interact_ms:ms,..s},3,2000),SitDecision::Held);
        }
        // A previously rejected future request becoming current is still never replayed.
        assert_eq!(seat.poll(crate::position::Sample{interact_seq:10,interact_ms:2001,..s},3,2100),SitDecision::Held);
        assert_eq!(seat.poll(crate::position::Sample{interact_seq:11,interact_ms:2100,..s},3,2100),SitDecision::Stand(1));
    }
    #[test]
    fn failed_activation_expires_and_a_new_pair_or_load_cannot_inherit_a_seat() {
        let s=seated_sample();let mut seat=Intent::default();seat.arm(s,1000);
        assert_eq!(seat.poll(s,0,4999),SitDecision::Held);
        assert_eq!(seat.poll(s,0,5000),SitDecision::Failed);assert!(!seat.active());
        for other in [crate::position::Sample{sender:5,..s},crate::position::Sample{world:5,..s},
            crate::position::Sample{save_id:[5;16],..s},crate::position::Sample{mc_id:[5;16],..s}] {
            seat.arm(s,1000);assert_eq!(seat.poll(other,3,2000),SitDecision::Idle);assert!(!seat.active());
        }
        seat.arm(s,1000);assert_eq!(seat.poll(s,3,999),SitDecision::Idle);
        seat.arm(s,1000);assert_eq!(seat.poll(s,7,2000),SitDecision::Failed); // not a bed owner
    }
    #[test]
    fn failed_exit_is_bounded_and_still_never_moves_a_seated_actor() {
        let s=seated_sample();let mut seat=Intent::default();seat.arm(s,1000);
        let exit=crate::position::Sample{controls:1,..s};let mut calls=Vec::new();
        for now in 1000..=30_000 {
            match seat.poll(exit,3,now) {
                SitDecision::Stand(n)=>calls.push((now,n)),
                SitDecision::GaveUp=>calls.push((now,0)),
                SitDecision::Held=>{},other=>panic!("unexpected {other:?}"),
            }
        }
        assert_eq!(calls,vec![(1000,1),(3000,2),(5000,3),(7000,4),(9000,5),(11_000,0)]);
        assert!(seat.active());assert_eq!(seat.poll(s,0,30_001),SitDecision::Align);
        assert_eq!(seat.poll(s,3,8000),SitDecision::Idle); // Rollback after a retry.
    }
    #[test]
    fn only_ordinary_sit_capable_furniture_enters_intentional_sitting() {
        let sit=1<<30;let sleep=1<<31;
        assert!(is_chair(0x28,0,0x40000001));
        assert!(!is_chair(0x28,0,0));assert!(!is_chair(0x28,0,1<<29));
        assert!(!is_chair(0x28,0,sit|sleep));assert!(!is_chair(0x28,1,sit));
        assert!(!is_chair(0x2B,0,sit));
    }
    #[test]
    fn standing_never_asks_for_recovery() {
        let mut watch=Watch::default();
        for now in (0..=60_000).step_by(100) {assert_eq!(watch.look(0,now),None);}
    }
    #[test]
    fn a_stuck_character_gets_five_spaced_calls_and_one_gave_up() {
        let mut watch=Watch::default();let mut actions=Vec::new();
        for now in 0..=60_000 {
            if let Some(action)=watch.look(3,now) {actions.push((now,action));}
        }
        assert_eq!(actions,vec![(4000,Decision::Stand(1)),(6000,Decision::Stand(2)),
            (8000,Decision::Stand(3)),(10_000,Decision::Stand(4)),(12_000,Decision::Stand(5)),
            (14_000,Decision::GaveUp)]);
    }
    #[test]
    fn standing_resets_the_wait_and_retry_count_but_other_states_do_not() {
        let mut watch=Watch::default();
        for now in (0..4000).step_by(100) {assert_eq!(watch.look(1+(now/1000) as u32,now),None);}
        assert_eq!(watch.look(4,4000),Some(Decision::Stand(1)));
        assert_eq!(watch.look(0,4100),None);
        for now in (4200..8200).step_by(100) {assert_eq!(watch.look(7,now),None);}
        assert_eq!(watch.look(7,8200),Some(Decision::Stand(1)));
        // Successful standing also restarts a watch that had already given up.
        for now in (8300..=18_200).step_by(100) {let _=watch.look(7,now);}
        assert_eq!(watch.look(0,18_300),None);
        for now in (18_400..22_400).step_by(100) {assert_eq!(watch.look(3,now),None);}
        assert_eq!(watch.look(3,22_400),Some(Decision::Stand(1)));
    }
    #[test]
    fn a_gap_over_half_a_second_or_a_clock_rollback_restarts_the_wait() {
        let mut watch=Watch::default();
        for now in (0..=3500).step_by(500) {assert_eq!(watch.look(3,now),None);}
        assert_eq!(watch.look(3,4000),Some(Decision::Stand(1))); // Exactly 500 is allowed.
        assert_eq!(watch.look(3,4501),None); // 501 resets even after a call.
        for now in (5001..8501).step_by(500) {assert_eq!(watch.look(3,now),None);}
        assert_eq!(watch.look(3,8501),Some(Decision::Stand(1)));
        assert_eq!(watch.look(3,10),None);
        for now in (510..4010).step_by(500) {assert_eq!(watch.look(3,now),None);}
        assert_eq!(watch.look(3,4010),Some(Decision::Stand(1)));
    }
    #[test]
    fn a_workbench_sit_menu_and_stand_never_asks_for_recovery() {
        let mut watch=Watch::default();
        for now in (0..=2500).step_by(100) {assert_eq!(watch.look(1,now),None);}
        // No looks while the crafting menu is open. On return, a 1.5 second stand-up.
        for now in (20_000..=21_500).step_by(100) {assert_eq!(watch.look(4,now),None);}
        for now in (21_600..30_000).step_by(100) {assert_eq!(watch.look(0,now),None);}
        // Explicit release also resets across a menu shorter than 500 ms.
        for now in (30_000..=33_500).step_by(100) {assert_eq!(watch.look(3,now),None);}
        watch.reset();
        for now in (33_700..=35_200).step_by(100) {assert_eq!(watch.look(4,now),None);}
        assert_eq!(watch.look(0,35_300),None);
    }
    /// Numbers read from Skyrim.esm for the meadery of the report: the bar stool and the
    /// chairs have bench type 0 and "can sit" (bit 30); the beds "can sleep" (bit 31).
    #[test]
    fn a_seat_is_furniture_without_a_menu() {
        let (sit,sleep)=(1_u32<<30,1_u32<<31);
        assert!(is_seat(0x28,0,sit));
        // A lean marker and a block to chop wood on carry other marker bits or none.
        assert!(is_seat(0x28,0,1<<29));
        assert!(is_seat(0x28,0,0));
        // Workbenches of every kind open a menu; a bed opens the sleep menu.
        for bench in 1..=7 {assert!(!is_seat(0x28,bench,sit),"{bench}");}
        assert!(!is_seat(0x28,0,sleep));
        assert!(!is_seat(0x28,0,sleep|sit));
        // Nothing but furniture is a seat: a door, a container, an activator, a character.
        for other in [0x1D,0x1C,0x18,0x2B,0x00] {assert!(!is_seat(other,0,sit),"{other:#x}");}
    }
    #[test]
    fn the_sit_state_is_four_bits_of_the_state_word() {
        assert_eq!(sit_state(0),0);
        assert_eq!(sit_state(3<<14),3);
        assert_eq!(sit_state(8<<14),8);
        // The neighbours do not leak in: movement bits below, fly and life state above
        // (life state 1, dying, is bit 21; the death code reads the same word).
        assert_eq!(sit_state(0x3FFF),0);
        assert_eq!(sit_state(!(0xF<<14)),0);
        assert_eq!(sit_state((1<<21)|(4<<14)|0x1FF),4);
        assert_eq!((((1_u32<<21)|(4<<14))>>21)&0xF,1);
        assert_eq!([0,3,4,7,9].map(label),["standing","seated_or_riding","standing_up","asleep","unknown"]);
    }
    #[test]
    fn a_reference_is_read_only_as_far_as_its_type_allows() {
        // A base record that is a door: only its type byte is read.
        let mut door=[0_u8;0x20];door[0x1A]=0x1D;
        let mut reference=[0_u8;0x48];
        reference[0x40..0x48].copy_from_slice(&(door.as_mut_ptr() as usize).to_le_bytes());
        assert_eq!(unsafe {furniture(reference.as_mut_ptr())},(0x1D,0,0));
        assert!(!unsafe {target_is_seat(reference.as_mut_ptr())});
        // A stool, and a forge.
        let mut record=[0_u8;0xF0];record[0x1A]=0x28;
        record[0xE4..0xE8].copy_from_slice(&(1_u32<<30).to_le_bytes());
        let record=record.as_mut_ptr();
        reference[0x40..0x48].copy_from_slice(&(record as usize).to_le_bytes());
        assert!(unsafe {target_is_seat(reference.as_mut_ptr())});
        unsafe {*record.add(0xE0)=1;}
        assert_eq!(unsafe {furniture(reference.as_mut_ptr())},(0x28,1,1<<30));
        assert!(!unsafe {target_is_seat(reference.as_mut_ptr())});
        // No reference, or one without a base record.
        assert!(!unsafe {target_is_seat(ptr::null_mut())});
        let mut bare=[0_u8;0x48];
        assert!(!unsafe {target_is_seat(bare.as_mut_ptr())});
    }
}
