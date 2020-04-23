use std::io;
use std::process::Command;
use std::{thread, time};

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run_app() -> io::Result<()> {
    let _ = run_cmd("smartctl", vec!["--test=short", "/dev/sdb"])?;
    let _ = run_cmd("smartctl", vec!["--test=short", "/dev/sdc"])?;

    thread::sleep(time::Duration::from_secs(80));

    let _ = run_cmd("smartctl", vec!["-q", "errorsonly", "-a", "/dev/sdb"])?;
    let _ = run_cmd("smartctl", vec!["-q", "errorsonly", "-a", "/dev/sdc"])?;

    Ok(())
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> io::Result<()> {
    println!("running `{} {}`", cmd, args.join(" "));

    let output = Command::new(cmd).args(args.clone()).output()?;

    assert!(output.status.success());

    Ok(())
}
