use std::fs;
use std::path::Path;

pub enum DataType {
    JSON,
    TXT,
}

impl DataType {
    pub fn to_string(&self) -> &str {
        match self {
            Self::JSON => "json",
            Self::TXT => "txt",
        }
    }
}

pub struct Record {
    header: RecordHeader,
    data: Vec<u8>, // Byte string
}

pub struct RecordHeader {
    key: String,
    data_type: DataType,
    size: u32,
}

pub fn create_record(key: &String, path: &String) -> Record {
    // Get file extension
    let ext: &str = Path::new(path)
        .extension()
        .and_then(|oss| oss.to_str())
        .expect("File should have a format.");

    // Detect file format
    let data_type: DataType = match ext {
        "txt" => DataType::TXT,
        "json" => DataType::JSON,
        _ => DataType::TXT,
    };

    // Get file size
    let file: fs::File = fs::File::open(path).expect("File should have opened.");

    let file_size: u32 = file.metadata().unwrap().len() as u32;

    // Create record header
    let record_header = RecordHeader {
        key: key.to_string() + ";",
        data_type: data_type,
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

pub fn serialize_record(rec: Record) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(rec.header.key.as_bytes());
    data.extend_from_slice(rec.header.data_type.to_string().as_bytes());
    data.extend_from_slice(&rec.header.size.to_be_bytes()); // Store byte string in big-endian order
    data.extend_from_slice(&rec.data);

    return data;
}
