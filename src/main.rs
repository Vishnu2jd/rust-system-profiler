use structopt::StructOpt;
use std::time::Duration;
use std::thread;

mod profiler;
mod storage;
mod report;

use profiler::SystemProfiler;
use storage::Storage;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "5")]
    interval: u64,

    #[structopt(short, long, default_value = "60")]
    duration: u64,
}

fn main() {
    let args = Cli::from_args();
    let profiler = profiler::linux::LinuxProfiler::new();
    let storage = storage::file::FileStorage::new("profile_data.json");

    let iterations = args.duration / args.interval;

    for _ in 0..iterations {
        let snapshot = profiler.collect_metrics();
        storage.store(&snapshot);
        report::print_snapshot(&snapshot);
        thread::sleep(Duration::from_secs(args.interval));
    }

    report::generate_summary(&storage);
}
