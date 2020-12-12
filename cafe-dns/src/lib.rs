use cafe_common::{BinaryWriter, BitVector64};
use cafe_common::stream::Output as OutputStream;

fn to_u64(value: bool) -> u64 {
    match value {
        true => return 1,
        false => return 0
    };
}

fn encode_qname(qname: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(qname.len());
    for part in qname.split('.') {
        result.push(part.len() as u8);
        result.extend_from_slice(part.as_bytes());
    }

    result
}

#[derive(Debug)]
/// The header contains the following fields (RFC 1035):
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
pub struct Header {
    /// An identifier assigned by the program that generates any kind of query.
    id: u16,
    /// A field that specifies whether this message is a query (false), or a response (true).
    qr: bool,
    /// A four bit field that specifies kind of query in this message.  
    /// This value is set by the originator of a query
    /// and copied into the response.  The values are:
    /// 0            a standard query (QUERY)
    /// 1            an inverse query (IQUERY)
    /// 2            a server status request (STATUS)
    /// 3-15         reserved for future use
    opcode: u8,
    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    aa: bool,
    /// TrunCation - specifies that this message was truncated
    /// due to length greater than that permitted on the
    /// transmission channel.
    tc: bool,
    /// Recursion Desired - this bit may be set in a query and
    /// is copied into the response.  If RD is set, it directs
    /// the name server to pursue the query recursively.
    /// Recursive query support is optional.
    rd: bool, 
    /// Recursion Available - this be is set or cleared in a
    /// response, and denotes whether recursive query support is
    /// available in the name server.
    ra: bool,
    /// Reserved for future use.  Must be zero in all queries
    /// and responses.
    z: u8,
    /// Response code - this 4 bit field is set as part of
    /// responses.  The values have the following
    /// interpretation:
    /// 0               No error condition
    /// 1               Format error - The name server was
    ///                 unable to interpret the query.
    /// 2               Server failure - The name server was
    ///                 unable to process this query due to a
    ///                 problem with the name server.
    /// 3               Name Error - Meaningful only for
    ///                 responses from an authoritative name
    ///                 server, this code signifies that the
    ///                 domain name referenced in the query does
    ///                 not exist.
    /// 4               Not Implemented - The name server does
    ///                 not support the requested kind of query.
    /// 5               Refused - The name server refuses to
    ///                 perform the specified operation for
    ///                 policy reasons.  For example, a name
    ///                 server may not wish to provide the
    ///                 information to the particular requester,
    ///                 or a name server may not wish to perform
    ///                 a particular operation (e.g., zone
    ///                 transfer) for particular data.
    /// 6-15            Reserved for future use.
    rcode: u8,
    /// an unsigned 16 bit integer specifying the number of
    /// entries in the question section.
    qdcount: u16,
    /// an unsigned 16 bit integer specifying the number of
    /// resource records in the answer section.
    ancount: u16,
    /// an unsigned 16 bit integer specifying the number of name
    /// server resource records in the authority records section.
    nscount: u16,
    /// an unsigned 16 bit integer specifying the number of
    /// resource records in the additional records section.
    arcount: u16
}

impl Header {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            qr: false,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn qr(&self) -> bool {
        self.qr
    }

    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn aa(&self) -> bool {
        self.aa
    }

    pub fn tc(&self) -> bool {
        self.tc
    }
    
    pub fn rd(&self) -> bool {
        self.rd
    }

    pub fn set_rd(&mut self, value: bool) {
        self.rd = value
    }

    pub fn ra(&self) -> bool {
        self.ra
    }

    pub fn z(&self) -> u8 {
        self.z
    }

    pub fn rcode(&self) -> u8 {
        self.rcode
    }

    pub fn qdcount(&self) -> u16 {
        self.qdcount
    }

    pub fn ancount(&self) -> u16 {
        self.ancount
    }

    pub fn nscount(&self) -> u16 {
        self.nscount
    }

    pub fn arcount(&self) -> u16 {
        self.arcount
    }

    pub fn encode(&self, stream: &mut OutputStream) {
        let mut bitfield = BitVector64::new();
        bitfield.set_part(15, 1, to_u64(self.qr()));
        bitfield.set_part(11, 4, self.opcode().into());
        bitfield.set_part(10, 1, to_u64(self.aa()));
        bitfield.set_part(9, 1, to_u64(self.tc()));
        bitfield.set_part(8, 1, to_u64(self.rd()));
        bitfield.set_part(7, 1, to_u64(self.ra()));
        bitfield.set_part(4, 3, self.z().into());
        bitfield.set_part(0, 4, self.rcode().into());

        let mut encoder = BinaryWriter::new(stream);
        encoder.write_u16(self.id().to_be());
        encoder.write_u16((bitfield.data() as u16).to_be());
        encoder.write_u16(self.qdcount().to_be());
        encoder.write_u16(self.ancount().to_be());
        encoder.write_u16(self.nscount().to_be());
        encoder.write_u16(self.arcount().to_be());
    }
}

#[derive(Debug)]
/// The question section is used to carry the "question" in most queries,
/// i.e., the parameters that define what is being asked
///                               1  1  1  1  1  1
/// 0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                     QNAME                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QTYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     QCLASS                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
pub struct Question {
    /// a domain name represented as a sequence of labels, where
    /// each label consists of a length octet followed by that
    /// number of octets.  The domain name terminates with the
    /// zero length octet for the null label of the root.  Note
    /// that this field may be an odd number of octets; no
    /// padding is used.
    qname: String,
    /// a two octet code which specifies the type of the query.
    /// The values for this field include all codes valid for a
    /// TYPE field, together with some more general codes which
    /// can match more than one type of RR.
    qtype: u16,
    /// a two octet code that specifies the class of the query.
    /// For example, the QCLASS field is IN for the Internet.
    qclass: u16
}

impl Question {
    pub fn new(name: &str, qtype: u16, qclass: u16) -> Self {
        Self {
            qname: name.to_string(),
            qtype,
            qclass,
        }
    }

    pub fn host_name(&self) -> &str {
        &self.qname
    }

    pub fn encode(&self, stream: &mut OutputStream) {
        let qname = encode_qname(&self.qname);
        stream.write(&qname, 0, qname.len());

        let mut writer = BinaryWriter::new(stream);
        writer.write_u8(0);
        writer.write_u16(self.qtype.to_be());
        writer.write_u16(self.qclass.to_be());
    }
}

pub struct Request {
    header: Header,
    questions: Vec<Question>
}

impl Request {
    pub fn new(id: u16) -> Self {
        Self {
            header: Header::new(id),
            questions: Vec::new()
        }
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }

    pub fn questions(&self) -> &[Question] {
        &self.questions
    }

    pub fn add_question(&mut self, qname: &str, qtype: u16, qclass: u16) {
        self.questions.push(
            Question {
                qname: qname.to_string(),
                qtype,
                qclass
            }
        );

        self.header.qdcount += 1;
    }

    pub fn encode(&self, stream: &mut OutputStream) {
        self.header.encode(stream);
        for q in &self.questions {
            q.encode(stream);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_address() {
        assert_eq!(
            encode_qname("www.example.com"), 
            [3, 119, 119, 119, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109]);
        
        assert_eq!(
            encode_qname("mail.ru"), 
            [4, 109, 97, 105, 108, 2, 114, 117]);
    }
}
