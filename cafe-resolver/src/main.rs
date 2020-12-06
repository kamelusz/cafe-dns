use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::process::exit;
use structopt::StructOpt;

use cafe_common::stream::Output as OutputStream;
use cafe_dns::{Request as DnsRequest, Response as DnsResponse};

#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Args {
    #[structopt(short, long)]
    host: String,
}

fn main() {
    let args = Args::from_args();

    let laddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from([0; 4])), 0);
    let socket = match UdpSocket::bind(laddr) {
        Err(e) => {
            eprintln!("Unable to bind UDP socket: {}", e);
            exit(1)
        }
        Ok(s) => s,
    };

    let mut request = DnsRequest::new(1);
    request.header_mut().set_rd(true);
    request.add_question(&args.host, 1, 1);

    let mut buffer = Vec::with_capacity(512);
    let mut stream = OutputStream::new(&mut buffer);
    request.encode(&mut stream);

    let raddr = SocketAddr::from(([8, 8, 8, 8], 53));
    socket
        .connect(&raddr)
        .expect("Unable to connect UDP socket");
    let sent = socket
        .send(&buffer)
        .expect("Expect sent data");
    if sent != buffer.len() {
        eprintln!("Unable to send all {} bytes of datagram", buffer.len());
        exit(1);
    }

    let buffer = &mut [0u8; 2048];
    match socket.recv(&mut buffer[..]) {
        Err(err) => {
            eprintln!("UDP error: {}", err);
            exit(1)
        },
        _ => { }
    };

    let response = DnsResponse::decode(buffer).expect("Unable to decode response");
    for q in response.answers() {
        let ip = q.data();
        println!("{}: {}.{}.{}.{}", q.name(), ip[0], ip[1], ip[2], ip[3]);
    }
}
