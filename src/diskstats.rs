pub struct DiskStats {
    inner: Vec<DiskStat>
}

impl DiskStats {
    pub fn from_system() -> io::Result<Self> {
        let mut reader = BufReader::new(File::open("/proc/diskstats")?);
        let mut disk_stats = Vec::new();

        unimplemented!()
    }

    pub fn iter(&self) -> impl Iterator<Item=&DiskStat> {
        self.inner.iter()
    }
}

impl IntoIterator for DiskStats {
    type IntoIter = std::vec::IntoIter<DiskStat>;
    type Item = DiskStat;
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

pub struct DiskStat {
    pub major: u64,
    pub minor: u64,
    pub name: String,
    pub reads_completed: u64,
    pub reads_merged: u64,
    pub sectors_read: u64,
    // in ms
    pub time_reading: Duration,
    pub writes_completed: u64,
    pub writes_merged: u64,
    pub sectors_written: u64,
    // in ms
    pub time_writing: Duration,
    pub io_in_progress: u64,
    // in ms
    pub time_io: Duration,
    // in ms
    pub time_io_weighted: Duration,
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufRead};
    use super::Stat;

    #[test]
    fn parse_single() {
        let input = "processes 2453";
        assert_eq!(super::parse_single("processes", input).unwrap(), 2453);
    }

    #[test]
    fn parse_u64() {
        assert!(super::parse_u64("a123").is_err());
        assert_eq!(super::parse_u64("12 "),
                   Result::Ok((" ", 12)));
        assert_eq!(super::parse_u64("12"),
                   Result::Ok(("", 12)));
    }

    #[test]
    fn proc_stat() {
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
        let _stat = Stat::from_iter(io::Cursor::new(raw).lines()).unwrap();
    }
}
