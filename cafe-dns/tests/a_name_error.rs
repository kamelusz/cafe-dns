use cafe_dns::{ResponseCode as RCode, Response as DnsResponse};

/*
Frame 28: 68 bytes on wire (544 bits), 68 bytes captured (544 bits) on interface \Device\NPF_{4CBDB64B-C72C-4452-921C-DB95DBCAECC4}, id 0
Ethernet II, Src: Routerbo_59:27:1b (74:4d:28:59:27:1b), Dst: IntelCor_43:f4:9f (50:76:af:43:f4:9f)
Internet Protocol Version 4, Src: 192.168.88.1, Dst: 192.168.88.250
User Datagram Protocol, Src Port: 53, Dst Port: 58457
Domain Name System (response)
    Transaction ID: 0x0002
    Flags: 0x8183 Standard query response, No such name
        1... .... .... .... = Response: Message is a response
        .000 0... .... .... = Opcode: Standard query (0)
        .... .0.. .... .... = Authoritative: Server is not an authority for domain
        .... ..0. .... .... = Truncated: Message is not truncated
        .... ...1 .... .... = Recursion desired: Do query recursively
        .... .... 1... .... = Recursion available: Server can do recursive queries
        .... .... .0.. .... = Z: reserved (0)
        .... .... ..0. .... = Answer authenticated: Answer/authority portion was not authenticated by the server
        .... .... ...0 .... = Non-authenticated data: Unacceptable
        .... .... .... 0011 = Reply code: No such name (3)
    Questions: 1
    Answer RRs: 0
    Authority RRs: 0
    Additional RRs: 0
    Queries
        mai3l.ru: type A, class IN
            Name: mai3l.ru
            [Name Length: 8]
            [Label Count: 2]
            Type: A (Host Address) (1)
            Class: IN (0x0001)
    [Request In: 27]
    [Time: 0.001523000 seconds]
*/
const RESPONSE: [u8; 26] = [
    0x00, 0x02, 0x81, 0x83, 0x00, 0x01, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x05, 0x6d, 0x61, 0x69, 
    0x33, 0x6c, 0x02, 0x72, 0x75, 0x00, 0x00, 0x01, 
    0x00, 0x01
];

#[test]
fn decode_response() {
    let response = DnsResponse::decode(&RESPONSE).unwrap();
    assert_eq!(response.id(), 0x0002);

    assert_eq!(response.header().qr(), true);
    assert_eq!(response.header().rd(), true);
    assert_eq!(response.header().ra(), true);
    assert_eq!(response.header().rcode(), RCode::NameError);
    assert_eq!(response.header().qdcount(), 1);
    assert_eq!(response.header().ancount(), 0);

    let q = &response.questions()[0];
    assert_eq!(q.host_name(), "mai3l.ru");
}
