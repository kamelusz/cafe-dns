use cafe_dns::{Type, ResponseCode as RCode, Response as DnsResponse};

/*
Domain Name System (response)
    Transaction ID: 0x0001
    Flags: 0x8180 Standard query response, No error
        1... .... .... .... = Response: Message is a response
        .000 0... .... .... = Opcode: Standard query (0)
        .... .0.. .... .... = Authoritative: Server is not an authority for domain
        .... ..0. .... .... = Truncated: Message is not truncated
        .... ...1 .... .... = Recursion desired: Do query recursively
        .... .... 1... .... = Recursion available: Server can do recursive queries
        .... .... .0.. .... = Z: reserved (0)
        .... .... ..0. .... = Answer authenticated: Answer/authority portion was not authenticated by the server
        .... .... ...0 .... = Non-authenticated data: Unacceptable
        .... .... .... 0000 = Reply code: No error (0)
    Questions: 1
    Answer RRs: 2
    Authority RRs: 0
    Additional RRs: 0
    Queries
        _xmpp-client._tcp.jabber.ru: type SRV, class IN
            Name: _xmpp-client._tcp.jabber.ru
            [Name Length: 27]
            [Label Count: 4]
            Type: SRV (Server Selection) (33)
            Class: IN (0x0001)
    Answers
        _xmpp-client._tcp.jabber.ru: type SRV, class IN, priority 0, weight 0, port 5222, target jabber.ru
            Service: _xmpp-client
            Protocol: _tcp
            Name: jabber.ru
            Type: SRV (Server Selection) (33)
            Class: IN (0x0001)
            Time to live: 21278 (5 hours, 54 minutes, 38 seconds)
            Data length: 17
            Priority: 0
            Weight: 0
            Port: 5222
            Target: jabber.ru
        _xmpp-client._tcp.jabber.ru: type SRV, class IN, priority 10, weight 0, port 443, target allports.jabber.ru
            Service: _xmpp-client
            Protocol: _tcp
            Name: jabber.ru
            Type: SRV (Server Selection) (33)
            Class: IN (0x0001)
            Time to live: 21278 (5 hours, 54 minutes, 38 seconds)
            Data length: 26
            Priority: 10
            Weight: 0
            Port: 443
            Target: allports.jabber.ru
*/
const RESPONSE: [u8; 112] = [
    0x00, 0x01, 0x81, 0x80, 0x00, 0x01, 0x00, 0x02, 
    0x00, 0x00, 0x00, 0x00, 0x0c, 0x5f, 0x78, 0x6d, 
    0x70, 0x70, 0x2d, 0x63, 0x6c, 0x69, 0x65, 0x6e, 
    0x74, 0x04, 0x5f, 0x74, 0x63, 0x70, 0x06, 0x6a, 
    0x61, 0x62, 0x62, 0x65, 0x72, 0x02, 0x72, 0x75, 
    0x00, 0x00, 0x21, 0x00, 0x01, 0xc0, 0x0c, 0x00, 
    0x21, 0x00, 0x01, 0x00, 0x00, 0x53, 0x1e, 0x00,
    0x11, 0x00, 0x00, 0x00, 0x00, 0x14, 0x66, 0x06, 
    0x6a, 0x61, 0x62, 0x62, 0x65, 0x72, 0x02, 0x72, 
    0x75, 0x00, 0xc0, 0x0c, 0x00, 0x21, 0x00, 0x01, 
    0x00, 0x00, 0x53, 0x1e, 0x00, 0x1a, 0x00, 0x0a, 
    0x00, 0x00, 0x01, 0xbb, 0x08, 0x61, 0x6c, 0x6c, 
    0x70, 0x6f, 0x72, 0x74, 0x73, 0x06, 0x6a, 0x61, 
    0x62, 0x62, 0x65, 0x72, 0x02, 0x72, 0x75, 0x00
];

#[test]
fn decode_response() {
    let response = DnsResponse::decode(&RESPONSE).unwrap();
    assert_eq!(response.id(), 0x0001);

    assert_eq!(response.header().qr(), true);
    assert_eq!(response.header().rd(), true);
    assert_eq!(response.header().ra(), true);
    assert_eq!(response.header().rcode(), RCode::NoError);
    assert_eq!(response.header().qdcount(), 1);
    assert_eq!(response.header().ancount(), 2);
    assert_eq!(response.questions().len(), response.header().qdcount() as usize);
    assert_eq!(response.answers().len(), response.header().ancount() as usize);

    let q = &response.questions()[0];
    assert_eq!(q.host_name(), "_xmpp-client._tcp.jabber.ru");

    let a = &response.answers()[0];
    assert_eq!(a.name(), "_xmpp-client._tcp.jabber.ru");
    assert_eq!(a.class(), 1);
    assert_eq!(a.ttl(), 21278);
    match a.ttype() {
        Type::SRV { priority, weight, port, target } => {
            assert_eq!(*priority, 0);
            assert_eq!(*weight, 0);
            assert_eq!(*port, 5222);
            assert_eq!(*target, "jabber.ru");
        }
        _ => panic!("Unexpected type!")
    }

    let a = &response.answers()[1];
    assert_eq!(a.name(), "_xmpp-client._tcp.jabber.ru");
    assert_eq!(a.class(), 1);
    assert_eq!(a.ttl(), 21278);
    match a.ttype() {
        Type::SRV { priority, weight, port, target } => {
            assert_eq!(*priority, 10);
            assert_eq!(*weight, 0);
            assert_eq!(*port, 443);
            assert_eq!(*target, "allports.jabber.ru");
        }
        _ => panic!("Unexpected type!")
    }
}
