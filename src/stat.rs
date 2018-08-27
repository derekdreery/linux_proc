use {util, Error};
use std::{
    io,
    fs::File
};

macro_rules! parse_single {
    ($name:expr) => {
        |input| {
            let (input, name) = util::parse_token(input)
                .ok_or(Error::from("cannot read name"))?;
            if name != $name {
                return Err(Error::from("incorrect name"));
            }
            let (input, value) = util::parse_u64(input)
                .ok_or(Error::from("cannot read value"))?;
            let input = util::consume_space(input);
            if ! input.is_empty() {
                return Err(Error::from("trailing content"));
            }
            Ok(value)
        }
    }
}

/// The stats from `/proc/stat`.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Stat {
    /// Total stats, sum of all cpus.
    pub cpu_totals: StatCpu,
    /// For each cpu, the number of *units* spent in different contexts.
    pub cpus: Vec<StatCpu>,
    /// Number of context switches since the system booted.
    pub context_switches: u64,
    /// Timestamp (in seconds since epoch) that system booted.
    pub boot_time: u64,
    /// The total number of processes and threads created since system booted.
    pub processes: u64,
    /// The total number of processes running on the cpu.
    pub procs_running: u64,
    /// The total number of processes waiting to run on the cpu.
    pub procs_blocked: u64,
    // todo `softirq`
}

impl Stat {
    pub fn from_system() {
        Stat::from_reader(File::open("/proc/stat")?)
    }
    pub fn from_reader(reader: impl io::Read) -> io::Result<Self> {
        let mut reader = util::LineParser::new(reader)?);
        let cpu_totals = reader.parse_line(
            |s| StatCpu::from_str(s).ok_or_else(|| Error::from("reading totals line"))
        )?;
        let mut cpus = Vec::new();
        loop {
            if let Ok(cpu_info) = reader.parse_line(
                |s| StatCpu::from_str(s).ok_or_else(|| Error::from(String::new()))
            ) {
                cpus.push(cpu_info);
            } else {
                break;
            }
        }
        let context_switches = reader.parse_line(parse_single!("ctxt"))?;
        let boot_time = reader.parse_line(parse_single!("btime"))?;
        let processes = reader.parse_line(parse_single!("processes"))?;
        let procs_running = reader.parse_line(parse_single!("procs_running"))?;
        let procs_blocked = reader.parse_line(parse_single!("procs_blocked"))?;
        // todo softirq
        Ok(Stat {
            cpu_totals,
            cpus,
            context_switches,
            boot_time,
            processes,
            procs_running,
            procs_blocked
        })
    }
}

/// Info about the number of *units* in the various cpu contexts.
///
/// *units* could be anything, for example cpu cycles, or hundredths of a second. The numbers only
/// really make sense as a proportion of the total.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StatCpu {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    pub guest: u64,
}

impl StatCpu {
    fn from_str(input: &str) -> Option<StatCpu> {
        let (input, cpunum) = util::parse_token(input)?;
        if ! cpunum.starts_with("cpu") {
            return None;
        }

        let (input, user) = util::parse_u64(input)?;
        let (input, nice) = util::parse_u64(input)?;
        let (input, system) = util::parse_u64(input)?;
        let (input, idle) = util::parse_u64(input)?;
        let (input, iowait) = util::parse_u64(input)?;
        let (input, irq) = util::parse_u64(input)?;
        let (input, softirq) = util::parse_u64(input)?;
        let (input, steal) = util::parse_u64(input)?;
        let (input, guest) = util::parse_u64(input)?;
        let input = util::consume_space(input);
        if ! input.is_empty() {
            return None;
        }
        Some(StatCpu { user, nice, system, idle, iowait, irq, softirq, steal, guest })
    }

    pub fn total(&self) -> u64 {
        self.user
            .checked_add(self.nice).unwrap()
            .checked_add(self.system).unwrap()
            .checked_add(self.idle).unwrap()
            .checked_add(self.iowait).unwrap()
            .checked_add(self.irq).unwrap()
            .checked_add(self.softirq).unwrap()
            .checked_add(self.steal).unwrap()
            .checked_add(self.guest).unwrap()
    }

}

#[test]
fn test_stat() {
    let raw = "\
cpu  17501 2 6293 8212469 20141 1955 805 0 0 0
cpu0 4713 0 1720 2049410 8036 260 255 0 0 0
cpu1 3866 0 1325 2054893 3673 928 307 0 0 0
cpu2 4966 1 1988 2051243 5596 516 141 0 0 0
cpu3 3955 0 1258 2056922 2835 250 100 0 0 0
intr 1015182 8 8252 0 0 0 0 0 0 1 113449 0 0 198907 0 0 0 18494 0 0 1 0 0 0 29 22 7171 46413 13 0 413 167 528 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
ctxt 2238717
btime 1535128607
processes 2453
procs_running 1
procs_blocked 0
softirq 4257581 64 299604 69 2986 36581 0 3497229 283111 0 137937
";
    let _stat = Stat::from_reader(io::Cursor::new(raw).lines()).unwrap();
}
