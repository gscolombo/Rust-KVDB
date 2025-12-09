use std::fs::{File, OpenOptions, create_dir};
use std::io::{Result, Write};
use std::path::Path;

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

    // Escrever offset da raiz inicial (8 bytes de 0 = árvore vazia)
    db.write_all(&0u64.to_be_bytes())?; // Árvore vazia inicialmente
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
        .expect("Não foi possível abrir o arquivo do banco de dados");
}
