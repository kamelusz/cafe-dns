use crate::stream::{SeekError, SeekOrigin, Output as OutputStream};

pub struct BinaryWriter<'a: 'b, 'b> {
    stream: &'b mut OutputStream<'a>,
    buffer: [u8; 8]
}

impl<'a, 'b> BinaryWriter<'a, 'b> {
    pub fn new(stream: &'b mut OutputStream<'a>) -> Self {
        Self { 
            stream,
            buffer: [0; 8]
        }
    }

    pub fn seek(&mut self, loc: SeekOrigin, offset: i64) -> Result<usize, SeekError> {
        self.stream.seek(loc, offset)
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.stream.write_byte(byte)
    }

    pub fn write_i8(&mut self, byte: i8) {
        self.stream.write_byte(byte as u8)
    }

    pub fn write_u16(&mut self, value: u16) {
        self.buffer[0] = value as u8;
        self.buffer[1] = (value >> 8) as u8;
        self.stream.write(&self.buffer, 0, 2);
    }

    pub fn write_i16(&mut self, value: i16) {
        self.write_u16(value as u16);
    }

    pub fn write_u32(&mut self, value: u32) {
        self.buffer[0] = value as u8;
        self.buffer[1] = (value >> 8) as u8;
        self.buffer[2] = (value >> 16) as u8;
        self.buffer[3] = (value >> 24) as u8;
        self.stream.write(&self.buffer, 0, 4);
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write_u32(value as u32);
    }

    pub fn write_u64(&mut self, value: u64) {
        self.buffer[0] = value as u8;
        self.buffer[1] = (value >> 8) as u8;
        self.buffer[2] = (value >> 16) as u8;
        self.buffer[3] = (value >> 24) as u8;
        self.buffer[4] = (value >> 32) as u8;
        self.buffer[5] = (value >> 40) as u8;
        self.buffer[6] = (value >> 48) as u8;
        self.buffer[7] = (value >> 56) as u8;
        self.stream.write(&self.buffer, 0, 8);
    }

    pub fn write_i64(&mut self, value: i64) {
        self.write_u64(value as u64);
    }
}
