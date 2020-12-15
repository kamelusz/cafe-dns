use crate::stream::{SeekError, SeekOrigin, Input as InputStream};

pub struct BinaryReader<'a: 'b, 'b> {
    stream: &'b mut InputStream<'a>,
}

impl<'a, 'b> BinaryReader<'a, 'b> {
    pub fn new(stream: &'b mut InputStream<'a>) -> Self {
        Self { 
            stream,
        }
    }

    pub fn seek(&mut self, loc: SeekOrigin, offset: i64) -> Result<usize, SeekError> {
        self.stream.seek(loc, offset)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.stream.read_byte()
    }

    pub fn read_i8(&mut self) -> Option<i8> {
        match self.stream.read_byte() {
            Some(v) => Some(v as i8),
            _ => None
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        match (self.read_u8(), self.read_u8()) {
            (Some(i), Some(j)) => return Some(i as u16 | (j as u16) << 8),
            _ => return None
        };
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        match self.read_u16() {
            Some(v) => Some(v as i16),
            _ => None
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        match (self.read_u16(), self.read_u16()) {
            (Some(i), Some(j)) => return Some(i as u32 | (j as u32) << 16),
            _ => return None
        };
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        match self.read_u32() {
            Some(v) => Some(v as i32),
            _ => None
        }
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        match (self.read_u32(), self.read_u32()) {
            (Some(i), Some(j)) => return Some(i as u64 | (j as u64) << 32),
            _ => return None
        };
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        match self.read_u64() {
            Some(v) => Some(v as i64),
            _ => None
        }
    }
}
