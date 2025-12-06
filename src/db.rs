use std::fs::{File, OpenOptions, create_dir};
use std::io::{Result, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path};
use std::time::UNIX_EPOCH;

pub fn list_databases() -> Vec<String> {
    let pathstr = "./databases";
    let path = Path::new(pathstr);
    if !path.exists() {
        return Vec::new();
    }

    let mut dbs: Vec<String> = Vec::new();
    for db in path.read_dir().expect("Error reading database directory") {
        if let Ok(entry) = db {
            let p = entry.path().to_str().unwrap().to_owned();
            dbs.push(p[pathstr.len() + 1..p.len() - 5].to_string());
        }
    }

    dbs
}

pub fn create_database(name: &str) -> Result<File> {
    assert_eq!(name.is_empty(), false, "Database name must be non-empty.");

    let pathstr = &format!("./databases/{name}.kvdb");
    let path = Path::new(pathstr);

    let parent = path.parent().unwrap();
    if !parent.exists() {
        match create_dir(parent) {
            Ok(..) => println!("Databases folder created."),
            Err(e) => panic!("Error creating database folder: {e:?}"),
        }
    }

    let mut db = match File::create_new(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error creating database file: {e:?}")
        }
    };

    let magic: Vec<u8> = vec![0x52, 0x55, 0x53, 0x54]; // Magic number with the ASCII code for "RUST"
    let created_at = db
        .metadata()?
        .created()?
        .duration_since(UNIX_EPOCH)
        .expect("Should have calculated file creation time.")
        .as_secs_f32(); // Timestamp of file creation

    let mut header: Vec<u8> = Vec::new();
    header.extend_from_slice(&magic);
    header.extend_from_slice(&created_at.to_be_bytes());
    header.extend_from_slice(&created_at.to_be_bytes()); // Update time is the same a creation time here  

    db.write_all(&header)?;
    return Ok(db)
}

pub fn open_database(name: &str) -> File {
    assert_eq!(name.is_empty(), false, "Database name must be non-empty.");

    let pathstr = &format!("./databases/{name}.kvdb");
    let path = Path::new(pathstr);

    return OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .expect("Arquivo do banco de dados deveria ter sido aberto.")
}

// pub fn insert_record(db: &str, key: &str, path: &str) -> Result<()> {
//     assert_eq!(
//         db.trim().is_empty(),
//         false,
//         "Database name must be non-empty."
//     );
//     assert_eq!(key.trim().is_empty(), false, "Key must be non-empty.");
//     assert_eq!(
//         path.trim().is_empty(),
//         false,
//         "Path to file must be non-empty."
//     );

//     let db_path = Path::new(&format!("./databases/{db}.kvdb")).to_owned();
//     let opener = OpenOptions::new().append(true).to_owned(); // File descriptor in append mode

//     // Open database file
//     let mut db_file = if !db_path.exists() {
//         println!("Creating new database {db}.");
//         match create_database(db) {
//             Ok(..) => println!("Database created successfully"),
//             Err(e) => panic!("Error creating database file: {e}"),
//         }
//         opener
//             .open(db_path)
//             .expect("Should have opened created database file.")
//     } else {
//         // TODO: Function to update database
//         opener
//             .open(db_path)
//             .expect("Should have opened database file.")
//     };

//     // Prepare record
//     let record = create_record(&key.to_string(), &path.to_string());
//     let record_data = serialize_record(record);

//     // Write to database
//     return db_file.write_all(&record_data);
// }
