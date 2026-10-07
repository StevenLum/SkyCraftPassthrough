//! Test receiver for the Java/native smoke check. It never claims Skyrim ran.
use passthrough_link::{Endpoint,Role,PROTOCOL_VERSION};
use std::{env,fs::{self,File},io::{self,Write},path::Path,thread,time::{Duration,Instant}};
fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len()!=2 { return Err(io::Error::other("usage: position-receiver SESSION LOG")); }
    let log = Path::new(&args[1]);
    if let Some(parent) = log.parent() { fs::create_dir_all(parent)?; }
    let mut file = File::create(log)?;
    writeln!(file,"event=FIXTURE_READY mode=standalone_receiver")?; file.flush()?;
    let mut endpoint = Endpoint::open(&args[0],Role::Skyrim,PROTOCOL_VERSION)?;
    let start = Instant::now(); let mut last = 0; let mut count = 0;
    while start.elapsed() < Duration::from_secs(15) {
        endpoint.poll()?;
        if let Some(sample) = endpoint.receive_position()? {
            if sample.frame != last {
                writeln!(file,"event=FIXTURE_RECV {}",sample.log_fields())?; file.flush()?;
                last=sample.frame; count+=1;
                if count==100 { return Ok(()); }
            }
        }
        thread::sleep(Duration::from_millis(2));
    }
    Err(io::Error::other(format!("received only {count} frames")))
}
fn main() { if let Err(error)=run() { eprintln!("{error}"); std::process::exit(1); } }
