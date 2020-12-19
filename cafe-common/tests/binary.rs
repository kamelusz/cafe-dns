use cafe_common::memory_stream::{MemoryStream, SeekOrigin};
use cafe_common::binary_writer::BinaryWriter;
use cafe_common::binary_reader::BinaryReader;

fn run_on_empty_stream<F>(mut action: F) 
where F : FnMut(&mut MemoryStream) {
    let mut buffer: Vec<u8> = Vec::new();
    let mut stream = MemoryStream::new(&mut buffer);
    action(&mut stream);
}


#[test]
fn write_read_u8() {
    let data = 129;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_u8(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let byte = reader.read_u8().unwrap();
            assert_eq!(data, byte);
        }
    });
}

#[test]
fn write_read_i8() {
    let data = -100;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_i8(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let byte = reader.read_i8().unwrap();
            assert_eq!(data, byte);
        }
    });
}

#[test]
fn write_read_u16() {
    let data : u16 = 36401;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_u16(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let short = reader.read_u16().unwrap();
            assert_eq!(data, short);
        }
    });
}

#[test]
fn write_read_i16() {
    let data : i16 = -10000;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_i16(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let short = reader.read_i16().unwrap();
            assert_eq!(data, short);
        }
    });
}

#[test]
fn write_read_u32() {
    let data : u32 = 145306401;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_u32(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let int = reader.read_u32().unwrap();
            assert_eq!(data, int);
        }
    });
}

#[test]
fn write_read_i32() {
    let data : i32 = -145306401;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_i32(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let int = reader.read_i32().unwrap();
            assert_eq!(data, int);
        }
    });
}

#[test]
fn write_read_u64() {
    let data : u64 = 123145306401;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_u64(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let long = reader.read_u64().unwrap();
            assert_eq!(data, long);
        }
    });
}

#[test]
fn write_read_i64() {
    let data : i64 = -14539846106401;
    run_on_empty_stream(|mut stream| {
        {
            let mut writer = BinaryWriter::new(&mut stream);
            writer.write_i64(data);
        }

        stream.seek(SeekOrigin::Begin, 0).unwrap();

        {
            let mut reader = BinaryReader::new(&mut stream);
            let int = reader.read_i64().unwrap();
            assert_eq!(data, int);
        }
    });
}
