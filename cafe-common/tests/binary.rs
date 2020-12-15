use cafe_common::stream::{Input as InputStream, Output as OutputStream};
use cafe_common::binary_writer::BinaryWriter;
use cafe_common::binary_reader::BinaryReader;

#[test]
fn write_read_u8() {
    let data = 129;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_u8(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let byte = reader.read_u8().unwrap();

    assert_eq!(data, byte);
}

#[test]
fn write_read_i8() {
    let data = -100;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_i8(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let byte = reader.read_i8().unwrap();
    
    assert_eq!(data, byte);
}

#[test]
fn write_read_u16() {
    let data : u16 = 36401;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_u16(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let short = reader.read_u16().unwrap();

    assert_eq!(data, short);
}

#[test]
fn write_read_i16() {
    let data : i16 = -10000;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_i16(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let short = reader.read_i16().unwrap();

    assert_eq!(data, short);
}

#[test]
fn write_read_u32() {
    let data : u32 = 145306401;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_u32(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let int = reader.read_u32().unwrap();

    assert_eq!(data, int);
}

#[test]
fn write_read_i32() {
    let data : i32 = -145306401;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_i32(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let int = reader.read_i32().unwrap();

    assert_eq!(data, int);
}

#[test]
fn write_read_u64() {
    let data : u64 = 123145306401;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_u64(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let long = reader.read_u64().unwrap();
        
    assert_eq!(data, long);
}

#[test]
fn write_read_i64() {
    let data : i64 = -14539846106401;

    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_i64(data);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    let long = reader.read_i64().unwrap();

    assert_eq!(data, long);
}

#[test]
fn write_read_all() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut output = OutputStream::new(&mut buffer);
    let mut writer = BinaryWriter::new(&mut output);
    writer.write_u8(111);
    writer.write_i8(-1);
    writer.write_u16(30000);
    writer.write_i16(-1511);
    writer.write_u32(610000001);
    writer.write_i32(-301123456);
    writer.write_u64(10967811235610000001);
    writer.write_i64(-5773123456301123456);

    let mut input = InputStream::new(&buffer);
    let mut reader = BinaryReader::new(&mut input);
    assert_eq!(reader.read_u8().unwrap(), 111);
    assert_eq!(reader.read_i8().unwrap(), -1);
    assert_eq!(reader.read_u16().unwrap(), 30000);
    assert_eq!(reader.read_i16().unwrap(), -1511);
    assert_eq!(reader.read_u32().unwrap(), 610000001);
    assert_eq!(reader.read_i32().unwrap(), -301123456);
    assert_eq!(reader.read_u64().unwrap(), 10967811235610000001);
    assert_eq!(reader.read_i64().unwrap(), -5773123456301123456);
}
