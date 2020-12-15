use crate::stream::{SeekError, SeekOrigin Input, Output};

#[test]
fn new_stream() {
    let mut buffer: Vec<u8> = vec![1, 2, 3];
    let mut stream = Output::new(&mut buffer);

    assert_eq!(stream.position(), 0);
    assert_eq!(stream.length(), 3);
}

#[test]
fn write_byte() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stream = Output::new(&mut buffer);
    assert_eq!(stream.length(), 0);
    assert_eq!(stream.position(), 0);

    stream.write_byte(1);

    assert_eq!(stream.length(), 1);
    assert_eq!(stream.position(), 1);
}

#[test]
fn write_read_byte() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stream = Output::new(&mut buffer);
        
    stream.write_byte(1);
    stream.write_byte(2);
    
    assert_eq!(stream.length(), 2);
    assert_eq!(stream.position(), 2);

    let pos = stream.seek(SeekOrigin::Begin, 0).unwrap();
    assert_eq!(pos, 0);
    assert_eq!(stream.position(), 0);

    let mut stream = Input::new(&mut buffer);
    let byte = stream.read_byte().unwrap();
    assert_eq!(byte, 1);
    assert_eq!(stream.position(), 1);

    let byte = stream.read_byte().unwrap();
    assert_eq!(byte, 2);
    assert_eq!(stream.position(), 2);

    let byte = stream.read_byte();
    assert!(byte.is_none());
}

#[test]
fn write_read_chunk() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stream = Output::new(&mut buffer);
    
    let data = &[1, 2, 3, 4, 5];
    stream.write(data, 0, data.len());
    assert_eq!(stream.length(), data.len());

    let pos = stream.seek(SeekOrigin::Begin, 0).unwrap();
    assert_eq!(pos, 0);
    assert_eq!(stream.position(), 0);

    const SIZE: usize = 3;
    let mut dst: [u8; SIZE] = [1; SIZE];
    let mut stream = Input::new(&mut buffer);

    stream.read(&mut dst, 0, SIZE);
    assert_eq!(dst[0], 1);
    assert_eq!(dst[1], 2);
    assert_eq!(dst[2], 3);
}
