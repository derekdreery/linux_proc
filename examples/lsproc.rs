//! lsproc - a simple program to inspect system status.
#[macro_use]
extern crate quicli;
extern crate linux_proc;

use quicli::prelude::*;

/// Carriage return
const CR_CODE: &'static str = "\x1b[G";
/// Clear to end of line
const CLEAR_CODE: &'static str = "\x1b[K";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    verbosity: Verbosity,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Copy, Clone, StructOpt)]
enum Command {
    /// Present the contents of `/proc/stat`.
    #[structopt(name = "stat")]
    Stat,
}

main!(|args: Cli, log_level: verbosity| {
    match args.command {
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
    }
});




