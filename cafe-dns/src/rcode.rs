use std::convert::TryFrom;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResponseCode {
    /// No error condition
    NoError = 0,
    /// The name server was unable to interpret the query.
    FormatError = 1,
    /// The name server was unable to process this query due to a
    /// problem with the name server.
    ServerFailure = 2,
    /// Meaningful only for responses from an authoritative name
    /// server, this code signifies that the domain name referenced in the query does.
    NameError = 3,
    /// The name server does not support the requested kind of query.
    NotImplemented = 4,
    /// The name server refuses to perform the specified operation for
    /// policy reasons.  For example, a name server may not wish to provide the
    /// information to the particular requester, or a name server may not wish to perform
    /// a particular operation (e.g., zone transfer) for particular data.
    Refused = 5
}

impl Default for ResponseCode {
    fn default() -> Self {
        Self::NoError
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == ResponseCode::NoError as u8 => Ok(ResponseCode::NoError),
            x if x == ResponseCode::FormatError as u8 => Ok(ResponseCode::FormatError),
            x if x == ResponseCode::ServerFailure as u8 => Ok(ResponseCode::ServerFailure),
            x if x == ResponseCode::NameError as u8 => Ok(ResponseCode::NameError),
            x if x == ResponseCode::NotImplemented as u8 => Ok(ResponseCode::NotImplemented),
            x if x == ResponseCode::Refused as u8 => Ok(ResponseCode::Refused),
            _ => Err(()),
        }
    }
}
