#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

use clap::Clap;
use prometheus::Gauge;
use std::io;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

#[derive(Clap)]
struct Opts {
    #[clap(short = "p", long = "prometheus-push-addr", required = true)]
    metrics_addr: String,
    #[clap(short = "d", long = "drive", multiple = true, required = true)]
    drives: Vec<String>,
}

lazy_static! {
    static ref LAST_COMPLETED_GAUGE: Gauge = register_gauge!(
        "pecker_last_completed_epoch_seconds",
        "The time of last completion"
    )
    .unwrap();
}

fn main() {
    let opts: Opts = Opts::parse();

    std::process::exit(match run_app(opts.drives) {
        Ok(_) => {
            let _ = meter_completion(&opts.metrics_addr, true);
            0
        }
        Err(err) => {
            let _ = meter_completion(&opts.metrics_addr, false);
            eprintln!("error: {:?}", err);
            1
        }
    });
}

fn run_app(drives: Vec<String>) -> io::Result<()> {
    for d in &drives {
        let _ = run_cmd("smartctl", vec!["--test=short", &d])?;
    }

    thread::sleep(time::Duration::from_secs(80));

    for d in &drives {
        let _ = run_cmd("smartctl", vec!["-q", "errorsonly", "-a", &d])?;
    }

    Ok(())
}

fn run_cmd(cmd: &str, args: Vec<&str>) -> io::Result<()> {
    println!("running `{} {}`", cmd, args.join(" "));

    let output = Command::new(cmd).args(args.clone()).output()?;

    assert!(output.status.success());

    Ok(())
}

fn meter_completion(address: &str, success: bool) -> Result<(), prometheus::Error> {
    LAST_COMPLETED_GAUGE.set(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as f64,
    );
    let success_str = if success { "1" } else { "0" };
    prometheus::push_metrics(
        "pecker",
        labels! {"success".to_owned() => success_str.to_owned(),},
        address,
        prometheus::gather(),
        None,
    )
}
