use crate::stream::{SeekError, SeekOrigin};

pub struct Input<'a> {
    buf: &'a [u8],
    position: usize,
}

impl<'a> Input<'a> {
    #[must_use]
    pub fn new(buf: &[u8]) -> Input {
        Input { buf, position: 0 }
    }

    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.buf.len()
    }

    pub fn seek(&mut self, loc: SeekOrigin, offset: i64) -> Result<usize, SeekError> {
        let position = crate::stream::calculate_position(self.position(), self.length(), loc, offset);
        if let Ok(p) = position {
            self.position = p
        }

        position
    }

    pub fn read(&mut self, buffer: &mut [u8], offset: usize, count: usize) {
        let src = &self.buf[self.position .. self.position + count];
        buffer[offset .. offset + count].clone_from_slice(src);
        self.position += count;
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        if self.position() >= self.length() {
            return None
        } 
        
        let result = self.buf[self.position()];
        self.position += 1;
        Some(result)
    }

    pub fn data(&self) -> Option<&[u8]> {
        return Some(&self.buf[self.position() ..])
    }
}
