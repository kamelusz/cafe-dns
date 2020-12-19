use std::convert::TryFrom;

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum QType {
    A = 1,
    SRV = 33
}

impl Default for QType {
    fn default() -> Self {
        Self::A
    }
}

impl TryFrom<u16> for QType {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == QType::A as u16 => Ok(QType::A),
            x if x == QType::SRV as u16 => Ok(QType::SRV),
            _ => Err(()),
        }
    }
}
