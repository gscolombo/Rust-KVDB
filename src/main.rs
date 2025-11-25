pub mod btree;
pub use btree::BTree;
mod records;

use records::{create_record, serialize_record};
use std::env;
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let key = &args[1];
    let path = &args[2];

    println!("Creating record for file in {path}...");
    let rec = create_record(key, path);

    let fname = key.clone() + ".kvdb";
    let mut db = fs::File::create(fname).expect("Error creating database file.");
    
    db.write_all(&serialize_record(rec))?;
    Ok(())
}
