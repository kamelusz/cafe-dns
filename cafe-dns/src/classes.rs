use std::convert::TryFrom;

#[repr(u16)]
#[derive(Debug, Copy, Clone)]
pub enum QClass {
    IN = 1
}

impl Default for QClass {
    fn default() -> Self {
        Self::IN
    }
}

impl TryFrom<u16> for QClass {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            x if x == QClass::IN as u16 => Ok(QClass::IN),
            _ => Err(()),
        }
    }
}
