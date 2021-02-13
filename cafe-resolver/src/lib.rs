use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

use cafe_common::stream::Output as OutputStream;
use cafe_dns::{QClass, QType, Request as DnsRequest, Response as DnsResponse, ResponseCode, Type};

#[derive(Debug)]
pub enum AddressVariant {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    SRV {
        target: String,
        port: u16,
        priority: u16,
        weight: u16,
    },
}

impl fmt::Display for AddressVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressVariant::V4(ip) => write!(f, "{}", ip),
            AddressVariant::V6(ip) => write!(f, "{}", ip),
            AddressVariant::SRV {
                target,
                port,
                priority,
                weight,
            } => write!(f, "{}:{}; p:{}; w:{}", target, port, priority, weight),
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
}

type ResolveResult = Result<Vec<AddressVariant>, ResolveError>;

impl Resolver {
    pub fn new() -> Self {
        return Self {
            id_count: 0,
            buffer: [0; 65_535],
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

    fn get_records(&mut self, socket: &UdpSocket, qtype: QType, host: &str) -> ResolveResult {
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
            match answer.ttype() {
                Type::A { ip } => result.push(AddressVariant::V4(*ip)),
                Type::SRV {
                    priority,
                    weight,
                    port,
                    target,
                } => result.push(AddressVariant::SRV {
                    target: target.to_string(),
                    port: *port,
                    priority: *priority,
                    weight: *weight,
                }),
            }
        }

        return Ok(result);
    }

    pub fn get_srv_records(&mut self, host: &str) -> ResolveResult {
        let socket = self.connect_to_server()?;
        return self.get_records(&socket, QType::SRV, host);
    }

    pub fn get_a_records(&mut self, host: &str) -> ResolveResult {
        let socket = self.connect_to_server()?;
        return self.get_records(&socket, QType::A, host);
    }
}
