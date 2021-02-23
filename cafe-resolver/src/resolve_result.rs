use std::fmt;
use std::net::IpAddr;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct ARecord {
    ip: IpAddr,
    time_to_die: Instant,
}

#[derive(Debug, Clone)]
pub struct SRVRecord {
    target: String,
    port: u16,
    time_to_die: Instant,
    ips: Vec<ARecord>,
}

#[derive(Debug, Clone)]
pub enum Record {
    A(ARecord),
    SRV(SRVRecord),
}

impl Record {
    pub fn make_a(ip: IpAddr, time_to_die: Instant) -> Self {
        Record::A(ARecord { ip, time_to_die })
    }

    pub fn is_outdated(&self, time: Instant) -> bool {
        let time_to_die = match self {
            Record::A(r) => r.time_to_die,
            Record::SRV(r) => r.time_to_die,
        };

        time_to_die <= time
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Record::A(r) => write!(f, "{}", r.ip),
            Record::SRV(r) => write!(f, "{:?}; port: {}", r.ips, r.port),
        }
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
