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
        www.mail.ru: type A, class IN
            Name: www.mail.ru
            [Name Length: 11]
            [Label Count: 3]
            Type: A (Host Address) (1)
            Class: IN (0x0001)
*/
const REQUEST: [u8; 29] = [
    0x00, 0x01, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 
    0x00, 0x00, 0x00, 0x00, 0x03, 0x77, 0x77, 0x77,
    0x04, 0x6d, 0x61, 0x69, 0x6c, 0x02, 0x72, 0x75, 
    0x00, 0x00, 0x01, 0x00, 0x01
];

#[test]
fn encode_request() {
    let mut request = DnsRequest::new(1);
    request.header_mut().set_rd(true);
    request.add_question("www.mail.ru", QType::A, QClass::IN);

    let mut result: Vec<u8> = Vec::new();
    let mut stream = OutputStream::new(&mut result);
    request.encode(&mut stream);
    assert_eq!(result.len(), REQUEST.len());
    assert_eq!(&result[..], &REQUEST);
}
