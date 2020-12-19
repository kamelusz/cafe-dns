pub struct MemoryStream<'a> {
    buf: &'a mut Vec<u8>,
    position: usize,
}

#[derive(Debug)]
pub enum SeekOrigin {
    Begin,
    Current,
    End
}

#[derive(Debug)]
pub enum SeekError {
    BeforeBegin,
    AfterEnd
}

impl<'a> MemoryStream<'a> {
    #[must_use]
    pub fn new(buf: &'a mut Vec<u8>) -> Self {
        MemoryStream { buf, position: 0 }
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
        self.position = match loc {
            SeekOrigin::Begin => {
                let new_position = if offset >= 0 {
                    offset as usize
                } else {
                    return Err(SeekError::BeforeBegin)
                };

                if new_position >= self.length() {
                    return Err(SeekError::AfterEnd)
                }

                new_position
            },
            SeekOrigin::Current => {
                let new_position = if offset.is_positive() {
                    self.position + offset as usize
                } else {
                    let u_offset = offset.wrapping_abs() as usize;
                    if self.position < u_offset {
                        return Err(SeekError::BeforeBegin)
                    }
        
                    self.position - u_offset
                };

                if new_position >= self.length() {
                    return Err(SeekError::BeforeBegin)
                }

                new_position
            },
            SeekOrigin::End => {
                let new_position = if offset.is_positive() {
                    return Err(SeekError::AfterEnd)
                } else {
                    let u_offset = offset.wrapping_abs() as usize;

                    self.length() - u_offset
                };

                new_position
            }
        };

        Ok(self.position)
    }

    pub fn write(&mut self, buffer: &[u8], offset: usize, count: usize) {
        self.buf.extend_from_slice(&buffer[offset .. count]);
        self.position += count
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.buf.push(byte);
        self.position += 1
    }

    pub fn read(&mut self, buffer: &mut [u8], offset: usize, count: usize) {
        let src = &self.buf.as_slice()[self.position .. count];
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_on_empty_stream<F>(mut action: F) 
    where F : FnMut(&mut MemoryStream) {
        let mut buffer: Vec<u8> = Vec::new();
        let mut stream = MemoryStream::new(&mut buffer);

        action(&mut stream);
    }

    fn run_on_short_stream<F>(mut action: F) 
    where F : FnMut(&mut MemoryStream) {
        let mut buffer: Vec<u8> = vec![1, 2, 3];
        let mut stream = MemoryStream::new(&mut buffer);

        action(&mut stream);
    }

    #[test]
    fn new_stream() {
        run_on_short_stream(|stream| {
            assert_eq!(stream.position(), 0);
            assert_eq!(stream.length(), 3);
        });
    }

    #[test]
    fn seek_begin() {
        run_on_short_stream(|stream| {
            let pos = stream.seek(SeekOrigin::Begin, 1).unwrap();
            assert_eq!(pos, 1);
            assert_eq!(stream.position(), 1);
        });
    }

    #[test]
    #[should_panic]
    fn seek_before_begin() {
        run_on_short_stream(|stream| {
            stream.seek(SeekOrigin::Begin, -1).unwrap();
        });
    }

    #[test]
    fn seek_current() {
        run_on_short_stream(|stream| {
            let pos = stream.seek(SeekOrigin::Current, 1).unwrap();
            assert_eq!(pos, 1);
            assert_eq!(stream.position(), 1);

            let pos = stream.seek(SeekOrigin::Current, 1).unwrap();
            assert_eq!(pos, 2);
            assert_eq!(stream.position(), 2);

            let pos = stream.seek(SeekOrigin::Current, -2).unwrap();
            assert_eq!(pos, 0);
            assert_eq!(stream.position(), 0);
        });
    }

    #[test]
    #[should_panic]
    fn seek_current_before_begin() {
        run_on_short_stream(|stream| {
            let pos = stream.seek(SeekOrigin::Current, 1).unwrap();
            assert_eq!(pos, 1);
            assert_eq!(stream.position(), 1);

            stream.seek(SeekOrigin::Current, -2).unwrap();
        });
    }

    #[test]
    fn seek_end() {
        run_on_short_stream(|stream| {
            let pos = stream.seek(SeekOrigin::End, -1).unwrap();
            assert_eq!(pos, 2);
            assert_eq!(stream.position(), 2);
        });
    }

    #[test]
    fn write_byte() {
        run_on_empty_stream(|stream| {
            assert_eq!(stream.length(), 0);
            assert_eq!(stream.position(), 0);
    
            stream.write_byte(1);
    
            assert_eq!(stream.length(), 1);
            assert_eq!(stream.position(), 1);
        });
    }

    #[test]
    fn write_read_byte() {
        run_on_empty_stream(|stream| {
            stream.write_byte(1);
            stream.write_byte(2);
            assert_eq!(stream.length(), 2);
            assert_eq!(stream.position(), 2);

            let pos = stream.seek(SeekOrigin::Begin, 0).unwrap();
            assert_eq!(pos, 0);
            assert_eq!(stream.position(), 0);

            let byte = stream.read_byte().unwrap();
            assert_eq!(byte, 1);
            assert_eq!(stream.position(), 1);

            let byte = stream.read_byte().unwrap();
            assert_eq!(byte, 2);
            assert_eq!(stream.position(), 2);

            let byte = stream.read_byte();
            assert!(byte.is_none());
        });
    }

    #[test]
    fn write_read_chunk() {
        run_on_empty_stream(|stream| {
            let data = &[1, 2, 3, 4, 5];

            stream.write(data, 0, data.len());
            assert_eq!(stream.length(), data.len());

            let pos = stream.seek(SeekOrigin::Begin, 0).unwrap();
            assert_eq!(pos, 0);
            assert_eq!(stream.position(), 0);

            const SIZE: usize = 3;
            let mut dst: [u8; SIZE] = [1; SIZE];
            stream.read(&mut dst, 0, SIZE);

            assert_eq!(dst[0], 1);
            assert_eq!(dst[1], 2);
            assert_eq!(dst[2], 3);
        });
    }
}
