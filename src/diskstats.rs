//! Bindings to `/proc/diskstats`.
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::time::Duration;

use crate::{util, Error};

pub struct DiskStats {
    inner: HashMap<String, DiskStat>,
}

impl DiskStats {
    const PATH: &'static str = "/proc/diskstats";
    /// Parse the contents of `/proc/diskstats`.
    pub fn from_system() -> io::Result<Self> {
        DiskStats::from_reader(File::open(Self::PATH)?)
    }

    fn from_reader(reader: impl io::Read) -> io::Result<Self> {
        let mut reader = util::LineParser::new(reader);
        let mut inner = HashMap::new();
        loop {
            match reader.parse_line(DiskStat::from_str) {
                Ok(disk_stat) => {
                    if inner.insert(disk_stat.name.clone(), disk_stat).is_some() {
                        panic!("Duplicate device name in /proc/diskstats");
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }
        Ok(DiskStats { inner })
    }

    pub fn iter(&self) -> impl Iterator<Item = &DiskStat> {
        self.inner.values()
    }
}

impl std::ops::Deref for DiskStats {
    type Target = HashMap<String, DiskStat>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl IntoIterator for DiskStats {
    type IntoIter = std::collections::hash_map::IntoIter<String, DiskStat>;
    type Item = (String, DiskStat);
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Debug, Clone)]
pub struct DiskStat {
    pub major: u64,
    pub minor: u64,
    pub name: String,
    pub reads_completed: u64,
    pub reads_merged: u64,
    pub sectors_read: u64,
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

macro_rules! err_msg {
    ($inner:expr, $msg:expr) => {
        $inner.ok_or_else(|| Error::from($msg))
    };
}

impl DiskStat {
    fn from_str(input: &str) -> Result<DiskStat, Error> {
        let (input, major) = err_msg!(util::parse_u64(input), "major number")?;
        let (input, minor) = err_msg!(util::parse_u64(input), "minor number")?;
        let (input, name) = err_msg!(util::parse_token(input), "device name")?;
        let name = name.to_owned();
        let (input, reads_completed) =
            err_msg!(util::parse_u64(input), "reads completed successfully")?;
        let (input, reads_merged) = err_msg!(util::parse_u64(input), "reads merged")?;
        let (input, sectors_read) = err_msg!(util::parse_u64(input), "sectors read")?;
        let (input, time_reading) = err_msg!(util::parse_u64(input), "time spent reading (ms)")?;
        let time_reading = Duration::from_millis(time_reading);
        let (input, writes_completed) =
            err_msg!(util::parse_u64(input), "writes completed successfully")?;
        let (input, writes_merged) = err_msg!(util::parse_u64(input), "writes merged")?;
        let (input, sectors_written) = err_msg!(util::parse_u64(input), "sectors written")?;
        let (input, time_writing) = err_msg!(util::parse_u64(input), "time writing")?;
        let time_writing = Duration::from_millis(time_writing);
        let (input, io_in_progress) =
            err_msg!(util::parse_u64(input), "I/Os currently in progress")?;
        let (input, time_io) = err_msg!(util::parse_u64(input), "time spent doing I/Os (ms)")?;
        let time_io = Duration::from_millis(time_io);
        let (_input, time_io_weighted) = err_msg!(
            util::parse_u64(input),
            "weighted time spent doing I/Os (ms)"
        )?;
        let time_io_weighted = Duration::from_millis(time_io_weighted);
        // We don't check remaining content as future linux may add extra columns.
        Ok(DiskStat {
            major,
            minor,
            name,
            reads_completed,
            reads_merged,
            sectors_read,
            time_reading,
            writes_completed,
            writes_merged,
            sectors_written,
            time_writing,
            io_in_progress,
            time_io,
            time_io_weighted,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::DiskStats;
    use std::io;

    #[test]
    fn proc_diskstats() {
        let raw = "\
   8      16 sdb 213 0 18712 564 0 0 0 0 0 217 794
   8      17 sdb1 48 0 4688 157 0 0 0 0 0 164 227
   8      18 sdb2 44 0 4656 204 0 0 0 0 0 167 254
   8      19 sdb3 44 0 4656 187 0 0 0 0 0 164 234
   8       0 sda 446866 32893 8168064 20164 339296 376515 86758441 4343530 0 250860 4704740
   8       1 sda1 143 30 11462 24 1 0 8 0 0 50 64
   8       2 sda2 46 0 4992 0 0 0 0 0 0 17 17
   8       3 sda3 6 0 36 0 0 0 0 0 0 4 4
   8       5 sda5 446599 32863 8148758 20140 331949 376515 86758433 4337104 0 233207 4686390
   8      32 sdc 7354 0 1580168 91987 7 0 56 0 0 91374 96127
   8      33 sdc1 7279 0 1575472 91310 7 0 56 0 0 90670 95424
  11       0 sr0 0 0 0 0 0 0 0 0 0 0 0
";
        let _stat = DiskStats::from_reader(io::Cursor::new(raw)).unwrap();
    }
}
