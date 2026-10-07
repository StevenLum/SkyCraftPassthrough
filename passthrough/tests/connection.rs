//! Real Windows processes and named shared memory, without either game.
use std::{fs, path::{Path, PathBuf}, process::{Child, Command, Stdio},
    sync::atomic::{AtomicU64, Ordering}, thread, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct Probe { child: Child, log: PathBuf }
impl Probe {
    fn text(&self) -> String { fs::read_to_string(&self.log).unwrap_or_default() }
    fn wait_for(&mut self, text: &str) {
        self.wait_for_count(text, 1);
    }
    fn wait_for_count(&mut self, text: &str, count: usize) {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let log = self.text();
            if log.matches(text).count() >= count { return; }
            assert!(self.child.try_wait().unwrap().is_none(), "probe exited early: {}\n{log}", self.log.display());
            assert!(Instant::now() < deadline, "missing {text:?} ({count} times) in {}\n{log}", self.log.display());
            thread::sleep(Duration::from_millis(25));
        }
    }
    fn kill(&mut self) {
        if self.child.try_wait().unwrap().is_none() { self.child.kill().unwrap(); }
        self.child.wait().unwrap();
    }
    fn wait_exit(&mut self) -> bool {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if let Some(status) = self.child.try_wait().unwrap() { return status.success(); }
            assert!(Instant::now() < deadline, "probe did not exit: {}", self.log.display());
            thread::sleep(Duration::from_millis(25));
        }
    }
}
impl Drop for Probe {
    fn drop(&mut self) { let _ = self.child.kill(); let _ = self.child.wait(); }
}

struct Pair { session: String, logs: PathBuf }
impl Pair {
    fn new(label: &str) -> Self {
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let session = format!("test_{}_{}_{}", std::process::id(), time, NEXT.fetch_add(1, Ordering::Relaxed));
        let logs = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/link-tests").join(format!("{label}_{session}"));
        fs::create_dir_all(&logs).unwrap();
        Self { session, logs }
    }
    fn spawn(&self, role: &str, label: &str, extra: &[&str]) -> Probe {
        let log = self.logs.join(format!("{label}.log"));
        let child = Command::new(env!("CARGO_BIN_EXE_link-probe"))
            .args(["--role", role, "--session", &self.session, "--seconds", "30", "--log"])
            .arg(&log).args(extra).stdout(Stdio::null()).stderr(Stdio::null())
            .spawn().unwrap();
        Probe { child, log }
    }
}

#[test]
fn either_role_can_start_first_and_both_receive_heartbeats() {
    for (first, second) in [("skyrim", "minecraft"), ("minecraft", "skyrim")] {
        let pair = Pair::new(first);
        let mut a = pair.spawn(first, "first", &[]);
        a.wait_for("state=Waiting");
        let mut b = pair.spawn(second, "second", &[]);
        a.wait_for("event=STATE state=Connected");
        b.wait_for("event=STATE state=Connected");
        a.wait_for("event=HEARTBEAT state=Connected");
        b.wait_for("event=HEARTBEAT state=Connected");
        for probe in [&a, &b] {
            let log = probe.text();
            let counts: Vec<u64> = log.lines().filter_map(|line|
                line.split_whitespace().find_map(|field| field.strip_prefix("peer_heartbeat="))
                    .and_then(|value| value.parse().ok())).collect();
            assert!(counts.last().unwrap() > counts.first().unwrap(), "peer heartbeat must advance\n{log}");
        }
    }
}

#[test]
fn incompatible_versions_never_connect() {
    let pair = Pair::new("version");
    let mut a = pair.spawn("skyrim", "skyrim", &["--seconds", "2"]);
    let mut b = pair.spawn("minecraft", "minecraft", &["--protocol-version", "999", "--seconds", "2"]);
    a.wait_for("state=Incompatible");
    b.wait_for("state=Incompatible");
    assert!(a.wait_exit());
    assert!(b.wait_exit());
    assert!(!a.text().contains("state=Connected"));
    assert!(!b.text().contains("state=Connected"));
}

#[test]
fn stalled_peer_times_out_cannot_be_replaced_while_alive_and_reconnects_after_exit() {
    let pair = Pair::new("recovery");
    let mut skyrim = pair.spawn("skyrim", "skyrim", &[]);
    let mut minecraft = pair.spawn("minecraft", "minecraft", &["--pause-after-ms", "1500"]);
    skyrim.wait_for("event=STATE state=Connected");
    minecraft.wait_for("event=STATE state=Connected");
    minecraft.wait_for("event=PAUSED");
    skyrim.wait_for("event=STATE state=TimedOut");
    let mut duplicate = pair.spawn("minecraft", "duplicate", &[]);
    assert!(!duplicate.wait_exit());
    assert!(duplicate.text().contains("duplicate role Minecraft"));
    // The probe is paused outside its mutex, so this abrupt exit tests stale-slot
    // recovery without randomly hitting the separate abandoned-lock failure case.
    minecraft.kill();
    let mut replacement = pair.spawn("minecraft", "replacement", &[]);
    replacement.wait_for("event=STATE state=Connected");
    skyrim.wait_for_count("event=STATE state=Connected", 2);
    let log = skyrim.text();
    let sessions: Vec<&str> = log.lines().filter(|line| line.contains("event=STATE state=Connected"))
        .filter_map(|line| line.split_whitespace().find(|s| s.starts_with("peer_session="))).collect();
    assert_ne!(sessions[0], sessions[1], "restart must establish a fresh session");
}

#[test]
fn orderly_exit_returns_survivor_to_waiting() {
    let pair = Pair::new("shutdown");
    let mut skyrim = pair.spawn("skyrim", "skyrim", &[]);
    skyrim.wait_for("event=STATE state=Waiting");
    let mut minecraft = pair.spawn("minecraft", "minecraft", &["--seconds", "1"]);
    skyrim.wait_for("event=STATE state=Connected");
    assert!(minecraft.wait_exit());
    assert!(minecraft.text().contains("event=STOPPED"));
    skyrim.wait_for_count("event=STATE state=Waiting", 2);
}

#[test]
fn separate_session_names_do_not_connect() {
    let first = Pair::new("isolated_a");
    let second = Pair::new("isolated_b");
    let mut skyrim = first.spawn("skyrim", "skyrim", &["--seconds", "1"]);
    let mut minecraft = second.spawn("minecraft", "minecraft", &["--seconds", "1"]);
    assert!(skyrim.wait_exit());
    assert!(minecraft.wait_exit());
    for probe in [&skyrim, &minecraft] {
        assert!(probe.text().contains("state=Waiting"));
        assert!(!probe.text().contains("state=Connected"));
    }
}
