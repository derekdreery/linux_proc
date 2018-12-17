//! lsproc - a simple program to inspect system status.
#[macro_use]
extern crate quicli;
extern crate linux_proc;

use quicli::prelude::*;

use linux_proc::diskstats::DiskStat;

/// Carriage return
const CR_CODE: &'static str = "\x1b[G";
/// Clear to end of line
const CLEAR_CODE: &'static str = "\x1b[K";

/// Sampling interval length
const INTERVAL_NANOS: u64 = 400_000_000;
/// 1_000_000_000 nanoseconds in a second
const NANOS_IN_SEC: u64 = 1_000_000_000;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, StructOpt)]
enum Command {
    /// Present the contents of `/proc/stat`.
    #[structopt(name = "stat")]
    Stat,
    /// Present the contents of `/proc/diskstats`.
    #[structopt(name = "diskstats")]
    DiskStats {
        /// The disk devices for which information should be printed
        device: String,
    },
    /// Present the contents of `/proc/uptime`.
    #[structopt(name = "uptime")]
    Uptime,
}

main!(|args: Cli, log_level: verbosity| match args.command {
    Command::Stat => {
        let mut prev_stat = linux_proc::stat::Stat::from_system()?;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(400));
            let stat = linux_proc::stat::Stat::from_system()?;
            let cpu_sum = (stat.cpu_totals.total() - prev_stat.cpu_totals.total()) as f64;
            let idle = (stat.cpu_totals.idle - prev_stat.cpu_totals.idle) as f64;
            print!("{}", CR_CODE);
            print!("cpu: {:3.0}% ", (cpu_sum - idle) * 100.0 / cpu_sum);
            print!("{}", CLEAR_CODE);
            std::io::Write::flush(&mut std::io::stdout())?;
            prev_stat = stat;
        }
    }
    Command::DiskStats { device } => {
        let mut prev_stat = linux_proc::diskstats::DiskStats::from_system()?;
        loop {
            std::thread::sleep(std::time::Duration::from_nanos(INTERVAL_NANOS));
            let curr_stat = linux_proc::diskstats::DiskStats::from_system()?;
            let reading = time_reading(
                prev_stat
                    .get(&device)
                    .expect(&format!("cannot find device \"{}\"", &device)),
                curr_stat.get(&device).unwrap(),
            );
            let read_ratio = (reading as f64) / (INTERVAL_NANOS as f64);

            print!("{}", CR_CODE);
            print!("read: {:3.3}% ", read_ratio * 100.0);
            print!("{}", CLEAR_CODE);
            std::io::Write::flush(&mut std::io::stdout())?;
            prev_stat = curr_stat;
        }
    }
    Command::Uptime => {
        let uptime = linux_proc::uptime::Uptime::from_system()?;
        println!("system has been up for {:?}", uptime.up);
        println!("cores have been idle for {:?}", uptime.idle);
    }
});

fn time_reading(prev: &DiskStat, current: &DiskStat) -> u64 {
    let read_time = current.time_reading - prev.time_reading;
    let read_time = read_time
        .as_secs()
        .checked_mul(NANOS_IN_SEC)
        .expect("overflow")
        .checked_add(read_time.subsec_nanos().into())
        .expect("overflow");
    read_time
}
