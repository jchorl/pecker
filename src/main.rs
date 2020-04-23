use clap::Clap;
use std::io;
use std::process::Command;
use std::{thread, time};

#[derive(Clap)]
struct Opts {
    #[clap(short = "p", long = "prometheus-push-addr", required = true)]
    metrics_addr: String,
    #[clap(short = "d", long = "drive", multiple = true, required = true)]
    drives: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    std::process::exit(match run_app(opts.drives, opts.metrics_addr) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run_app(drives: Vec<String>, metrics_addr: String) -> io::Result<()> {
    for d in &drives {
        let _ = run_cmd("smartctl", vec!["--test=short", &d])?;
    }

    thread::sleep(time::Duration::from_secs(80));

    for d in &drives {
        let _ = run_cmd("smartctl", vec!["-q", "errorsonly", "-a", &d])?;
    }

    println!("would emit metrics to {}", metrics_addr);

    Ok(())
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> io::Result<()> {
    println!("running `{} {}`", cmd, args.join(" "));

    let output = Command::new(cmd).args(args.clone()).output()?;

    assert!(output.status.success());

    Ok(())
}
