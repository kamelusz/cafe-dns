use crate::stream::{SeekError, SeekOrigin};

pub struct Output<'a> {
    buf: &'a mut Vec<u8>,
    position: usize,
}

impl<'a> Output<'a> {
    #[must_use]
    pub fn new(buf: &'a mut Vec<u8>) -> Output {
        Output { buf, position: 0 }
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

    pub fn write(&mut self, buffer: &[u8], offset: usize, count: usize) {
        self.buf.extend_from_slice(&buffer[offset .. count]);
        self.position += count
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.buf.push(byte);
        self.position += 1
    }
}
