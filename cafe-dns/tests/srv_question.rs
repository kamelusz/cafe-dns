use cafe_common::stream::Output as OutputStream;
use cafe_dns::{QType, QClass, Request as DnsRequest};

/*
Domain Name System (query)
    Transaction ID: 0x0001
    Flags: 0x0100 Standard query
        0... .... .... .... = Response: Message is a query
        .000 0... .... .... = Opcode: Standard query (0)
        .... ..0. .... .... = Truncated: Message is not truncated
        .... ...1 .... .... = Recursion desired: Do query recursively
        .... .... .0.. .... = Z: reserved (0)
        .... .... ...0 .... = Non-authenticated data: Unacceptable
    Questions: 1
    Answer RRs: 0
    Authority RRs: 0
    Additional RRs: 0
    Queries
        _xmpp-client._tcp.jabber.ru: type SRV, class IN
            Name: _xmpp-client._tcp.jabber.ru
            [Name Length: 27]
            [Label Count: 4]
            Type: SRV (Server Selection) (33)
            Class: IN (0x0001)
*/
const REQUEST: [u8; 45] = [
    0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x0c, 0x5f, 0x78, 0x6d,
    0x70, 0x70, 0x2d, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x04, 0x5f, 0x74, 0x63, 0x70, 0x06, 0x6a,
    0x61, 0x62, 0x62, 0x65, 0x72, 0x02, 0x72, 0x75, 
    0x00, 0x00, 0x21, 0x00, 0x01    
];

#[test]
fn encode_request() {
    let mut request = DnsRequest::new(1);
    request.header_mut().set_rd(true);
    request.add_question("_xmpp-client._tcp.jabber.ru", QType::SRV, QClass::IN);

    let mut result: Vec<u8> = Vec::new();
    let mut stream = OutputStream::new(&mut result);
    request.encode(&mut stream);
    assert_eq!(result.len(), REQUEST.len());
    assert_eq!(&result[..], &REQUEST);
}