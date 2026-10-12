//! Optional switches: `switches.txt` in the project folder, one `name=off` per line.
//! Read once per process. No file means every step is on.
use std::sync::OnceLock;
use crate::{native::write_log,Role};
/// Every switch, in the bit order Minecraft receives. A new one goes at the end.
pub const NAMES:[&str;25]=["e_precedence","hostility","menu_keys","damage_pool","pause_shield",
    "pointer_monitor","healing","menu_wasd","void_guard","server_trust","pose_free","slow_scan",
    "ranged","shout","seat_guard","stand_up","fast_scan","sitting","safe_arrival","furniture_ground","native_hands","item_click","chair_return","effect_healing","fortify_healing"];
pub const HOSTILITY:usize=1;
pub const PAUSE_SHIELD:usize=4;
pub const HEALING:usize=6;
pub const SLOW_SCAN:usize=11;
pub const RANGED:usize=12;
pub const SHOUT:usize=13;
pub const SEAT_GUARD:usize=14;
pub const STAND_UP:usize=15;
pub const FAST_SCAN:usize=16;
pub const SITTING:usize=17;
pub const SAFE_ARRIVAL:usize=18;
pub const FURNITURE_GROUND:usize=19;
pub const NATIVE_HANDS:usize=20;
pub const ITEM_CLICK:usize=21;
pub const CHAIR_RETURN:usize=22;
pub const EFFECT_HEALING:usize=23;
pub const FORTIFY_HEALING:usize=24;
/// The set of switched-off steps as bits, and the lines that were not understood.
/// `#` starts a comment; names and values ignore case and surrounding spaces.
fn parse(text:&str)->(u64,Vec<String>) {
    let mut off=0_u64;let mut unknown=Vec::new();
    for line in text.lines() {
        let line=line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {continue;}
        let known=line.split_once('=').and_then(|(name,value)| {
            let bit=NAMES.iter().position(|n|*n==name.trim().to_ascii_lowercase())?;
            match value.trim().to_ascii_lowercase().as_str() {"off"=>Some((bit,true)),"on"=>Some((bit,false)),_=>None}
        });
        match known {
            Some((bit,true))=>off|=1<<bit,
            Some((bit,false))=>off&=!(1<<bit),
            None=>unknown.push(line.to_owned()),
        }
    }
    (off,unknown)
}
static OFF:OnceLock<u64>=OnceLock::new();
/// Bit set = that step is switched off. The first call reads the file and logs the result.
pub fn mask(role:Role)->u64 {
    // The unit tests must not depend on a file the user may have edited.
    if cfg!(test) {return 0;}
    *OFF.get_or_init(|| {
        let path=std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../switches.txt");
        let found=path.is_file();
        let (off,unknown)=std::fs::read_to_string(&path).map(|text|parse(&text)).unwrap_or((0,Vec::new()));
        let names:Vec<&str>=NAMES.iter().enumerate().filter(|(bit,_)|off&(1<<bit)!=0).map(|(_,name)|*name).collect();
        let _=write_log(role,format!("event=SCENE_SWITCHES file_found={found} off={names:?} not_understood={unknown:?}"));
        off
    })
}
pub fn on(role:Role,bit:usize)->bool {mask(role)&(1<<bit)==0}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn switches_are_off_only_when_the_file_says_so() {
        assert_eq!(parse(""),(0,vec![]));
        assert_eq!(parse("# nothing but a comment\n\n   \n"),(0,vec![]));
        let (off,unknown)=parse("hostility=off\n  Pause_Shield = OFF   # until the fight test passes\nmenu_keys=on\n");
        assert_eq!(off,(1<<HOSTILITY)|(1<<PAUSE_SHIELD));
        assert!(unknown.is_empty());
        // A later line wins, so a step can be turned back on without deleting the earlier line.
        assert_eq!(parse("healing=off\nhealing=on").0,0);
        // A misspelt name or value switches nothing off and is reported.
        let (off,unknown)=parse("hostilty=off\nhealing=maybe\njust words");
        assert_eq!(off,0);
        assert_eq!(unknown,["hostilty=off","healing=maybe","just words"]);
    }
    /// Minecraft uses the same numbers (Switches.java); the order must never change.
    #[test]
    fn switch_numbers_match_their_names() {
        assert_eq!((NAMES[HOSTILITY],NAMES[PAUSE_SHIELD],NAMES[HEALING]),("hostility","pause_shield","healing"));
        assert_eq!((NAMES[SLOW_SCAN],NAMES[RANGED],NAMES[SHOUT]),("slow_scan","ranged","shout"));
        assert_eq!((NAMES[SEAT_GUARD],NAMES[STAND_UP]),("seat_guard","stand_up"));
        assert_eq!(parse("stand_up=off").0,1<<STAND_UP);
        assert_eq!((NAMES[ITEM_CLICK],parse("item_click=off").0),("item_click",1<<ITEM_CLICK));
        assert_eq!((NAMES[CHAIR_RETURN],parse("chair_return=off").0),("chair_return",1<<CHAIR_RETURN));
        assert_eq!((NAMES[EFFECT_HEALING],parse("effect_healing=off").0),("effect_healing",1<<EFFECT_HEALING));
        assert_eq!((NAMES[FORTIFY_HEALING],parse("fortify_healing=off").0),("fortify_healing",1<<FORTIFY_HEALING));
        // The first nine are fixed since 0.10.28; later ones are only ever added at the end.
        assert_eq!(NAMES[..9],["e_precedence","hostility","menu_keys","damage_pool","pause_shield",
            "pointer_monitor","healing","menu_wasd","void_guard"]);
        assert_eq!(NAMES[9..14],["server_trust","pose_free","slow_scan","ranged","shout"]);
        assert_eq!(NAMES[14..],["seat_guard","stand_up","fast_scan","sitting","safe_arrival","furniture_ground","native_hands","item_click","chair_return","effect_healing","fortify_healing"]);
    }
}
