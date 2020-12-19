use cafe_dns::{Type, Response as DnsResponse};

/*
Domain Name System (response)
    Transaction ID: 0x0001
    Flags: 0x8180 Standard query response, No error
    Questions: 1
    Answer RRs: 2
    Authority RRs: 0
    Additional RRs: 0
    Queries
        www.mail.ru: type A, class IN
            Name: www.mail.ru
            [Name Length: 11]
            [Label Count: 3]
            Type: A (Host Address) (1)
            Class: IN (0x0001)
    Answers
        www.mail.ru: type A, class IN, addr 217.69.139.70
            Name: www.mail.ru
            Type: A (Host Address) (1)
            Class: IN (0x0001)
            Time to live: 31 (31 seconds)
            Data length: 4
            Address: 217.69.139.70
        www.mail.ru: type A, class IN, addr 94.100.180.70
            Name: www.mail.ru
            Type: A (Host Address) (1)
            Class: IN (0x0001)
            Time to live: 31 (31 seconds)
            Data length: 4
            Address: 94.100.180.70
*/
const RESPONSE: [u8; 61] = [
    0x00, 0x01, 0x81, 0x80, 0x00, 0x01, 0x00, 0x02, 
    0x00, 0x00, 0x00, 0x00, 0x03, 0x77, 0x77, 0x77,
    0x04, 0x6d, 0x61, 0x69, 0x6c, 0x02, 0x72, 0x75, 
    0x00, 0x00, 0x01, 0x00, 0x01, 0xc0, 0x0c, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1f, 0x00,
    0x04, 0xd9, 0x45, 0x8b, 0x46, 0xc0, 0x0c, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1f, 0x00, 
    0x04, 0x5e, 0x64, 0xb4, 0x46
];

#[test]
fn decode_response() {
    let response = DnsResponse::decode(&RESPONSE).unwrap();
    assert_eq!(response.id(), 0x0001);

    assert_eq!(response.header().qr(), true);
    assert_eq!(response.header().rd(), true);
    assert_eq!(response.header().ra(), true);
    assert_eq!(response.header().rcode(), 0);
    assert_eq!(response.header().qdcount(), 1);
    assert_eq!(response.header().ancount(), 2);
    assert_eq!(response.questions().len(), response.header().qdcount() as usize);
    assert_eq!(response.answers().len(), response.header().ancount() as usize);

    let q = &response.questions()[0];
    assert_eq!(q.host_name(), "www.mail.ru");
    
    let a = &response.answers()[0];
    assert_eq!(a.name(), "www.mail.ru");
    assert_eq!(a.class(), 1);
    assert_eq!(a.ttl(), 31);
    match a.ttype() {
        Type::A { ip } => assert_eq!(*ip, std::net::Ipv4Addr::new(217, 69, 139, 70)),
        _ => panic!("Unexpected type!")
    }

    let a = &response.answers()[1];
    assert_eq!(a.name(), "www.mail.ru");
    assert_eq!(a.class(), 1);
    assert_eq!(a.ttl(), 31);
    match a.ttype() {
        Type::A { ip } => assert_eq!(*ip, std::net::Ipv4Addr::new(94, 100, 180, 70)),
        _ => panic!("Unexpected type!")
    }
}