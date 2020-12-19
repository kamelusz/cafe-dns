pub mod types;
pub mod classes;

pub use self::types::QType;
pub use self::classes::QClass;

use cafe_common::{BinaryReader, BinaryWriter, BitVector64};
use cafe_common::stream::{SeekOrigin, Output as OutputStream, Input as InputStream};
use std::convert::TryInto;

fn to_u64(value: bool) -> u64 {
    match value {
        true => return 1,
        false => return 0
    };
}

fn to_bool(value: u64) -> bool {
    value != 0
}

fn encode_qname(qname: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(qname.len());
    for part in qname.split('.') {
        result.push(part.len() as u8);
        result.extend_from_slice(part.as_bytes());
    }

    result
}

fn decode_qname(bytes: &[u8]) -> Option<String> {
    if bytes.len() == 0 {
        return None
    }

    let mut current = bytes;
    let mut result = String::new();

    loop {
        let len = current[0] as usize;
        if len > current.len() {
            return None;
        }

        current = &current[1 ..];
        if !current[.. len].is_ascii() {
            return None;
        }

        for ch in current[.. len].iter().map(|byte| *byte as char) {
            result.push(ch);
        }

        current = &current[len ..];
        if current.is_empty() {
            return Some(result);
        }

        if current[0] as usize == 0 {
            return Some(result);
        }

        result.push('.')
    }
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

    pub fn decode(stream: &mut InputStream) -> Option<Header> {
        let mut reader = BinaryReader::new(stream);
        let id = reader.read_u16()?;

        let flags = reader.read_u8()?;
        let bits = BitVector64::from(flags as u64);
        let rd = bits.get_part(0, 1) as u64;
        let tc = bits.get_part(1, 1) as u64;
        let aa = bits.get_part(2, 1) as u64;
        let opcode = bits.get_part(3, 4) as u8;
        let qr = bits.get_part(7, 1) as u64;

        let flags = reader.read_u8()?;
        let bits = BitVector64::from(flags as u64);
        let rcode = bits.get_part(0, 4) as u8;
        let z  = bits.get_part(4, 3) as u8;
        let ra = bits.get_part(7, 1) as u64;

        let qdcount = reader.read_u16()?;
        let ancount = reader.read_u16()?;
        let nscount = reader.read_u16()?;
        let arcount = reader.read_u16()?;

        Some(
            Header {
                id: u16::from_be(id),
                qdcount: u16::from_be(qdcount),
                ancount: u16::from_be(ancount),
                nscount: u16::from_be(nscount),
                arcount: u16::from_be(arcount),
                qr: to_bool(qr),
                aa: to_bool(aa),
                tc: to_bool(tc),
                rd: to_bool(rd),
                ra: to_bool(ra),
                opcode,
                rcode,
                z
            }
        )
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
    qtype: QType,
    /// a two octet code that specifies the class of the query.
    /// For example, the QCLASS field is IN for the Internet.
    qclass: QClass
}

impl Question {
    pub fn new(name: &str, qtype: QType, qclass: QClass) -> Self {
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
        writer.write_u16((self.qtype as u16).to_be());
        writer.write_u16((self.qclass as u16).to_be());
    }

    pub fn decode(stream: &mut InputStream) -> Option<Question> {
        let qname = decode_qname(stream.data()?)?;
        if let Err(_) = stream.seek(SeekOrigin::Current, (qname.as_bytes().len() + 2) as i64) {
            return None
        }

        let mut reader = BinaryReader::new(stream);
        let qtype = reader.read_u16()?;
        let qtype = match u16::from_be(qtype).try_into() {
            Ok(QType::A) => QType::A,
            Ok(QType::SRV) => QType::SRV,
            Err(_) => return None
        };

        let qclass = reader.read_u16()?;
        let qclass = match u16::from_be(qclass).try_into() {
            Ok(QClass::IN) => QClass::IN,
            Err(_) => return None
        };

        Some(
            Question {
                qname,
                qtype,
                qclass
            }
        )
    }
}

#[derive(Debug)]
/// The answer, authority, and additional sections all share the same
/// format: a variable number of resource records, where the number of
/// records is specified in the corresponding count field in the header.
///                               1  1  1  1  1  1
/// 0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                                               /
/// /                      NAME                     /
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     CLASS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TTL                      |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                   RDLENGTH                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/// /                     RDATA                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
pub struct ResourceRecord {
    /// a domain name to which this resource record pertains.
    name: String,
    /// two octets containing one of the RR type codes.  This
    /// field specifies the meaning of the data in the RDATA
    /// field.
    ttype: u16,
    /// two octets which specify the class of the data in the
    /// RDATA field.
    class: u16,
    /// a 32 bit unsigned integer that specifies the time
    /// interval (in seconds) that the resource record may be
    /// cached before it should be discarded.  Zero values are
    /// interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.
    ttl: u32,
    /// an unsigned 16 bit integer that specifies the length in
    /// octets of the RDATA field.
    rdlength: u16,
    /// a variable length string of octets that describes the
    /// resource.  The format of this information varies
    /// according to the TYPE and CLASS of the resource record.
    /// For example, the if the TYPE is A and the CLASS is IN,
    /// the RDATA field is a 4 octet ARPA Internet address.
    rdata: Vec<u8>
}

impl ResourceRecord {
    pub fn decode(stream: &mut InputStream) -> Option<ResourceRecord> {
        let first_byte = stream.read_byte()?;
        let name = match ResourceRecord::is_compressed_name(first_byte) {
            true => {
                if let Err(_) = stream.seek(SeekOrigin::Current, -1) {
                    return None;
                }
                ResourceRecord::extract_name(stream)?
            }
            _ => panic!("not implemented yet")
        };

        let mut reader = BinaryReader::new(stream);
        let ttype = reader.read_u16()?;
        let class = reader.read_u16()?;
        let ttl = reader.read_u32()?;
        let rdlength_be = u16::from_be(reader.read_u16()?);

        let mut rdata: Vec<u8> = Vec::with_capacity(rdlength_be as usize);
        rdata.resize(rdlength_be as usize, 0);
        stream.read(&mut rdata, 0, rdlength_be as usize);

        Some(
            ResourceRecord {
                name,
                ttype: u16::from_be(ttype),
                class: u16::from_be(class),
                ttl: u32::from_be(ttl),
                rdlength: rdlength_be,
                rdata
            }
        )
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ttl(&self) -> u32 {
        self.ttl
    }

    pub fn ttype(&self) -> u16 {
        self.ttype
    }

    pub fn class(&self) -> u16 {
        self.class
    }

    pub fn data(&self) -> &[u8] {
        &self.rdata
    }

    fn is_compressed_name(byte: u8) -> bool {
        match byte == 0xC0 {
            true => true,
            _ => false
        }
    }

    fn extract_name(stream: &mut InputStream) -> Option<String> {
        let current = stream.position();

        let mut reader = BinaryReader::new(stream);
        let mut bits = BitVector64::from(reader.read_u16()? as u64);
        bits.set_part(6, 2, 0);
        let offset = u16::from_be(bits.get_part(0, 14) as u16);
        if let Err(_) = stream.seek(SeekOrigin::Begin, offset as i64) {
            return None;
        }

        let qname = decode_qname(stream.data()?)?;

        if let Err(_) = stream.seek(SeekOrigin::Begin, (current + 2) as i64) {
            return None;
        }

        Some(qname)
    }
}

pub struct Response {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<ResourceRecord>
}

impl Response {
    pub fn decode(data: &[u8]) -> Option<Response> {
        let mut stream = InputStream::new(data);
        let header = Header::decode(&mut stream)?;

        let mut questions = Vec::new();
        for _ in 0 .. header.qdcount() {
            questions.push(Question::decode(&mut stream)?);
        }

        if questions.len() != header.qdcount() as usize {
            return None;
        }

        let mut answers = Vec::new();
        for _ in 0 .. header.ancount() {
            answers.push(ResourceRecord::decode(&mut stream)?);
        }
        
        if answers.len() != header.ancount() as usize {
            return None;
        }

        Some(
            Response {
                header,
                questions,
                answers
        })
    }

    pub fn id(&self) -> u16 {
        self.header.id()
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn questions(&self) -> &[Question] {
        &self.questions
    }

    pub fn answers(&self) -> &[ResourceRecord] {
        &self.answers
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

    pub fn add_question(&mut self, qname: &str, qtype: QType, qclass: QClass) {
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

    #[test]
    fn decode_addreass() {
        assert_eq!( 
            decode_qname(&[3, 119, 119, 119, 7, 101, 120, 97, 109, 112, 108, 101, 3, 99, 111, 109]).unwrap(),
            "www.example.com");

        assert_eq!(
            decode_qname(&[4, 109, 97, 105, 108, 2, 114, 117]).unwrap(),
            "mail.ru");

        // zero end
        assert_eq!(
            decode_qname(&[4, 109, 97, 105, 108, 2, 114, 117, 0]).unwrap(),
            "mail.ru");

        assert_eq!(
            decode_qname(&[4, 109, 97, 105, 108, 2, 114, 117, 0, 23, 32, 99]).unwrap(),
            "mail.ru");
    }
}
