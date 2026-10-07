use passthrough_link::{Endpoint, POLL_INTERVAL_MS, PROTOCOL_VERSION, Role};
use std::{env, fs::{self, File}, io::{self, Write}, path::PathBuf,
    thread, time::{Duration, Instant}};

struct Options {
    role: Role,
    session: String,
    log: PathBuf,
    seconds: u64,
    protocol: u32,
    pause_after_ms: Option<u64>,
}

fn options() -> Result<Options, String> {
    let mut role = None;
    let mut session = "practice".to_owned();
    let mut log = None;
    let mut seconds = 15;
    let mut protocol = PROTOCOL_VERSION;
    let mut pause_after_ms = None;
    let mut args = env::args().skip(1);
    while let Some(key) = args.next() {
        let value = args.next().ok_or_else(|| format!("missing value for {key}"))?;
        match key.as_str() {
            "--role" => role = Some(match value.as_str() {
                "skyrim" => Role::Skyrim, "minecraft" => Role::Minecraft,
                _ => return Err("role must be skyrim or minecraft".into()),
            }),
            "--session" => session = value,
            "--log" => log = Some(PathBuf::from(value)),
            "--seconds" => seconds = value.parse::<u64>().map_err(|e| e.to_string())?,
            "--protocol-version" => protocol = value.parse::<u32>().map_err(|e| e.to_string())?,
            "--pause-after-ms" => pause_after_ms = Some(value.parse::<u64>().map_err(|e| e.to_string())?),
            _ => return Err(format!("unknown option {key}")),
        }
    }
    if !(1..=600).contains(&seconds) { return Err("seconds must be in 1..600".into()); }
    Ok(Options {
        role: role.ok_or("--role is required")?, session,
        log: log.ok_or("--log is required")?, seconds, protocol, pause_after_ms,
    })
}

fn log(file: &mut File, started: Instant, message: impl AsRef<str>) -> io::Result<()> {
    let line = format!("elapsed_ms={} {}", started.elapsed().as_millis(), message.as_ref());
    writeln!(file, "{line}")?;
    file.flush()?;
    println!("{line}");
    Ok(())
}

fn run(options: Options) -> io::Result<()> {
    if let Some(parent) = options.log.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(&options.log)?;
    let started = Instant::now();
    log(&mut file, started, format!("event=START role={:?} pid={} session={} protocol={}",
        options.role, std::process::id(), options.session, options.protocol))?;
    let result = (|| {
        let mut endpoint = Endpoint::open(&options.session, options.role, options.protocol)?;
        let mut previous = None;
        let mut heartbeat_log = Instant::now();
        while started.elapsed() < Duration::from_secs(options.seconds) {
            // Diagnostic fault injection: stop heartbeats while retaining ownership.
            // Log after releasing all locks so a test can safely kill this process.
            if options.pause_after_ms.is_some_and(|ms| started.elapsed().as_millis() >= ms as u128) {
                log(&mut file, started, "event=PAUSED")?;
                while started.elapsed() < Duration::from_secs(options.seconds) {
                    thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
                }
                break;
            }
            let observation = endpoint.poll()?;
            let key = (observation.state, observation.peer_session);
            let changed = previous != Some(key);
            if changed || heartbeat_log.elapsed() >= Duration::from_secs(1) {
                log(&mut file, started, format!(
                    "event={} state={:?} peer_pid={} peer_session={} peer_heartbeat={} peer_age_ms={}",
                    if changed { "STATE" } else { "HEARTBEAT" }, observation.state,
                    observation.peer_pid, observation.peer_session, observation.peer_heartbeat,
                    observation.peer_age_ms))?;
                previous = Some(key);
                heartbeat_log = Instant::now();
            }
            thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
        }
        endpoint.close()?;
        log(&mut file, started, "event=STOPPED")
    })();
    if let Err(ref error) = result {
        let _ = log(&mut file, started, format!("event=ERROR message={error}"));
    }
    result
}

fn main() {
    let result = options().map_err(io::Error::other).and_then(run);
    if let Err(error) = result {
        eprintln!("link-probe: {error}");
        std::process::exit(1);
    }
}
