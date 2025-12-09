use std::fs;
use std::path::Path;

pub struct Record {
    pub header: RecordHeader,
    data: Vec<u8>, // Byte string
}

pub struct RecordHeader {
    key: String,
    pub size: u64,
}

pub fn create_record(key: &String, path: &Path) -> Record {
    // Get file size
    let file: fs::File = fs::File::open(path).expect("File should have opened.");

    let file_size: u64 = file.metadata().unwrap().len() as u64;

    // Create record header
    let record_header = RecordHeader {
        key: key.to_string() + ";",
        size: file_size,
    };

    // Read file to byte string
    let data: Vec<u8> = fs::read(path).expect("File contents should have been readed.");

    // Create record
    return Record {
        header: record_header,
        data: data,
    };
}

pub fn serialize_record(rec: &Record) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(rec.header.key.as_bytes());
    data.extend_from_slice(&rec.header.size.to_be_bytes()); // Store byte string in big-endian order
    data.extend_from_slice(&rec.data);

    return data;
}
