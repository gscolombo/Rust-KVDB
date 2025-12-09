use std::fs::{File, OpenOptions, create_dir};
use std::io::{Read, Result, Seek, Write};
use std::path::Path;

use crate::btree::BTree;

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

    let mut header: Vec<u8> = Vec::new();
    header.extend_from_slice(&(0 as u64).to_le_bytes()); // Number of keys in database

    db.write_all(&header)?;
    return Ok(db);
}

pub fn open_database(name: &str) -> File {
    assert_eq!(name.is_empty(), false, "Database name must be non-empty.");

    let pathstr = &format!("./databases/{name}.kvdb");
    let path = Path::new(pathstr);

    return OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .expect("Arquivo do banco de dados deveria ter sido aberto.");
}

pub fn load_database(db: &mut File, index: &mut BTree) {
    let mut num_keys_bytes= [0u8;8];
    let mut offset: u64;
    let mut key: String = String::new();
    let mut char = [0u8; 1];
    let mut value_length = [0u8; 8];

    db.read_exact(&mut num_keys_bytes)
        .expect("Não foi possível ler a quantidade de chaves no banco de dados");

    let num_keys = u64::from_be_bytes(num_keys_bytes);

    for _ in 1..=num_keys {
        key.clear();

        loop {
            db.read_exact(&mut char)
                .expect("Erro lendo caractere da chave");

            if char[0] == b';' {
                break;
            }
            key.push(char[0] as char);
        }

        offset = db
            .stream_position()
            .expect("Erro durante leitura de arquivo do banco de dados");

        db.read_exact(&mut value_length)
            .expect("Erro durante leitura de arquivo do banco de dados");

        db.seek(std::io::SeekFrom::Current(i64::from_be_bytes(value_length)))
            .expect("Erro durante leitura de arquivo do banco de dados");

        index
            .insert(key.clone(), offset)
            .expect("Erro durante leitura de arquivo do banco de dados");
    }
}
