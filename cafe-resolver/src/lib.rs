pub mod resolve_result;

pub use self::resolve_result::{Record as ResolveRecord, Result as ResolveResult};

use std::collections::BTreeMap;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use cafe_common::stream::Output as OutputStream;
use cafe_dns::{QClass, QType, Request as DnsRequest, Response as DnsResponse, ResponseCode, Type};

#[derive(Debug)]
pub enum RecordVariant {
    A {
        ip: IpAddr,
        ttl: u32,
    },
    SRV {
        target: String,
        port: u16,
        priority: u16,
        weight: u16,
        ttl: u32,
    },
}

type RecordsResult = Result<Vec<RecordVariant>, ResolveError>;

impl fmt::Display for RecordVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecordVariant::A { ip, ttl } => write!(f, "{}; ttl: {}", ip, ttl),
            RecordVariant::SRV {
                target,
                port,
                priority,
                weight,
                ttl,
            } => write!(
                f,
                "{}:{}; priority: {}; weight: {}; ttl: {}",
                target, port, priority, weight, ttl
            ),
        }
    }
}

#[derive(Debug)]
pub enum ResolveError {
    TransportFailed,
    DecodeFailed,
    DnsError(ResponseCode),
}

#[derive(Debug)]
pub struct Resolver {
    id_count: u16,
    buffer: [u8; 65_535],
    cache: BTreeMap<String, Vec<ResolveRecord>>,
}

impl Resolver {
    pub fn new() -> Self {
        return Self {
            id_count: 0,
            buffer: [0; 65_535],
            cache: Default::default(),
        };
    }

    fn connect_to_server(&mut self) -> Result<UdpSocket, ResolveError> {
        let laddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from([0; 4])), 0);
        let socket = match UdpSocket::bind(laddr) {
            Err(_) => return Err(ResolveError::TransportFailed),
            Ok(s) => s,
        };

        let raddr = SocketAddr::from(([8, 8, 8, 8], 53));
        match socket.connect(&raddr) {
            Err(_) => return Err(ResolveError::TransportFailed),
            _ => (),
        };

        return Ok(socket);
    }

    fn get_response(&mut self, socket: &UdpSocket, buf: &mut [u8]) -> Result<(), ResolveError> {
        match socket.send(&buf) {
            Err(_) => return Err(ResolveError::TransportFailed),
            Ok(size) => {
                if size != buf.len() {
                    return Err(ResolveError::TransportFailed);
                }
            }
        };

        match socket.recv(&mut self.buffer[..]) {
            Err(_) => return Err(ResolveError::TransportFailed),
            _ => (),
        };

        return Ok(());
    }

    fn get_records(&mut self, socket: &UdpSocket, qtype: QType, host: &str) -> RecordsResult {
        self.id_count = self.id_count.wrapping_add(1);

        let mut request = DnsRequest::new(self.id_count);
        request.header_mut().set_rd(true);
        request.add_question(&host, qtype, QClass::IN);

        let mut buffer = Vec::with_capacity(512);
        let mut stream = OutputStream::new(&mut buffer);
        request.encode(&mut stream);

        match self.get_response(&socket, &mut buffer) {
            Err(_) => return Err(ResolveError::TransportFailed),
            _ => (),
        }

        let response = match DnsResponse::decode(&self.buffer) {
            None => return Err(ResolveError::DecodeFailed),
            Some(response) => response,
        };

        if response.header().rcode() != ResponseCode::NoError {
            return Err(ResolveError::DnsError(response.header().rcode()));
        }

        let mut result = Vec::new();
        for answer in response.answers() {
            let ttl = answer.ttl();
            match answer.ttype() {
                Type::A { ip } => result.push(RecordVariant::A {
                    ip: IpAddr::V4(*ip),
                    ttl,
                }),
                Type::SRV {
                    priority,
                    weight,
                    port,
                    target,
                } => result.push(RecordVariant::SRV {
                    target: target.to_string(),
                    port: *port,
                    priority: *priority,
                    weight: *weight,
                    ttl,
                }),
            }
        }

        return Ok(result);
    }

    fn get_srv_records_socket(&mut self, socket: &UdpSocket, host: &str) -> RecordsResult {
        let mut records = self.get_records(&socket, QType::SRV, host)?;
        records.sort_unstable_by(|a, b| match (a, b) {
            (
                RecordVariant::SRV {
                    target: _,
                    port: _,
                    priority: p1,
                    weight: _,
                    ttl: _,
                },
                RecordVariant::SRV {
                    target: _,
                    port: _,
                    priority: p2,
                    weight: _,
                    ttl: _,
                },
            ) => return p1.cmp(&p2),
            _ => return std::cmp::Ordering::Equal,
        });

        return Ok(records);
    }

    pub fn get_srv_records(&mut self, host: &str) -> RecordsResult {
        let socket = self.connect_to_server()?;
        return self.get_srv_records_socket(&socket, host);
    }

    fn get_a_records_socket(&mut self, socket: &UdpSocket, host: &str) -> RecordsResult {
        return self.get_records(&socket, QType::A, host);
    }

    pub fn get_a_records(&mut self, host: &str) -> RecordsResult {
        let socket = self.connect_to_server()?;
        return self.get_a_records_socket(&socket, host);
    }

    fn need_to_update_records(&mut self, host: &str) -> bool {
        let record = self.cache.get(host);
        match record {
            Some(rs) => {
                let now = Instant::now();
                for r in rs {
                    if r.is_outdated(now) {
                        return true;
                    }
                }

                return false;
            }
            None => {
                self.cache.insert(String::from(host), Vec::new());
                return true;
            }
        };
    }

    fn resolve_a_host_socket(
        &mut self,
        socket: &UdpSocket,
        host: &str,
    ) -> Result<(), ResolveError> {
        if self.need_to_update_records(host) {
            let records = self.get_a_records_socket(socket, host)?;
            let entry = self.cache.get_mut(host).unwrap();
            entry.clear();

            let now = Instant::now();
            for r in &records {
                match r {
                    RecordVariant::A { ip, ttl } => {
                        let time_to_die = now + Duration::new((*ttl).into(), 0);
                        entry.push(ResolveRecord::make_a(*ip, time_to_die));
                    }
                    _ => (),
                }
            }
        }

        Ok(())
    }

    pub fn resolve_host(&mut self, host: &str) -> Result<ResolveResult, ResolveError> {
        let socket = self.connect_to_server()?;
        self.resolve_a_host_socket(&socket, host)?;

        let entry = self.cache.get_mut(host).unwrap();
        return Ok(ResolveResult::new(entry));
    }

    pub fn resolve_srv_host(&mut self, host: &str) -> Result<ResolveResult, ResolveError> {
        self.need_to_update_records(host);
        let entry = self.cache.get_mut(host).unwrap();
        entry.clear();

        let socket = self.connect_to_server()?;
        let records = self.get_srv_records_socket(&socket, host)?;

        let now = Instant::now();
        for r in &records {
            match r {
                RecordVariant::SRV {
                    target,
                    port,
                    priority,
                    weight,
                    ttl,
                } => {
                    self.resolve_a_host_socket(&socket, target)?;
                    let ips = self.cache.get_mut(target).unwrap().clone();

                    let time_to_die = now + Duration::new((*ttl).into(), 0);
                    let record = ResolveRecord::make_srv(target, port, time_to_die: Instant, ips: &Vec<ARecord>)
                    entry.push()
                }
                _ => return Err(ResolveError::DecodeFailed),
            }
        }

        return Err(ResolveError::DecodeFailed);
    }
}
