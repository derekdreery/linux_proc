//! Bindings to `/proc/uptime`.
use std::fs::File;
use std::io;
use std::time::Duration;

use crate::{util, Error};

pub struct Uptime {
    /// The time the system has been up for.
    pub up: Duration,
    /// The time any core has been idle for. This may be more than the uptime if the system has
    /// multiple cores.
    pub idle: Duration,
}

impl Uptime {
    const PATH: &'static str = "/proc/uptime";
    /// Parse the contents of `/proc/uptime`.
    pub fn from_system() -> io::Result<Self> {
        Uptime::from_reader(File::open(Self::PATH)?)
    }

    pub fn from_reader(reader: impl io::Read) -> io::Result<Self> {
        let mut reader = util::LineParser::new(reader);
        let uptime = reader.parse_line(Self::from_str)?;
        Ok(uptime)
    }

    pub fn from_str(input: &str) -> Result<Self, Error> {
        let (input, up_secs) = util::parse_u64(input).ok_or("expected number")?;
        let input = util::expect_bytes(".", input).ok_or("expected \".\"")?;
        let (input, up_nanos) = util::parse_nanos(input).ok_or("expected number")?;
        let (input, idle_secs) = util::parse_u64(input).ok_or("expected number")?;
        let input = util::expect_bytes(".", input).ok_or("expected \".\"")?;
        let (_input, idle_nanos) = util::parse_nanos(input).ok_or("expected number")?;
        Ok(Uptime {
            up: Duration::new(up_secs, up_nanos),
            idle: Duration::new(idle_secs, idle_nanos),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Uptime;
    use std::io;

    #[test]
    fn proc_uptime() {
        let raw = "\
            1640919.14 2328903.47
";
        let _stat = Uptime::from_reader(io::Cursor::new(raw)).unwrap();
    }
}
