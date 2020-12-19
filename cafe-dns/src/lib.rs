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
