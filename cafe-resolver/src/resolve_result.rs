use std::fmt;
use std::net::IpAddr;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Record {
    host: String,
    ip: IpAddr,
    port: Option<u16>,
    time_to_die: Instant,
}

impl Record {
    pub fn new(host: &str, ip: IpAddr, port: Option<u16>, time_to_die: Instant) -> Self {
        Self {
            host: String::from(host),
            ip,
            port,
            time_to_die,
        }
    }

    pub fn is_outdated(&self, time: Instant) -> bool {
        time >= self.time_to_die
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip)
    }
}

pub struct Result {
    state: Vec<Record>,
}

impl Result {
    pub fn new(records: &Vec<Record>) -> Self {
        Self {
            state: records.to_vec(),
        }
    }
}

impl IntoIterator for Result {
    type Item = Record;
    type IntoIter = std::vec::IntoIter<Record>;

    fn into_iter(self) -> Self::IntoIter {
        self.state.into_iter()
    }
}

impl<'a> IntoIterator for &'a Result {
    type Item = &'a Record;
    type IntoIter = std::slice::Iter<'a, Record>;

    fn into_iter(self) -> Self::IntoIter {
        self.state.iter()
    }
}
