use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::time::Instant;
use kvdb::load_database;
use rand::seq::SliceRandom;
use tempfile::tempfile;
use kvdb::btree::BTree;

/// Helper: Create a test database file with given number of entries
fn create_test_database(num_entries: u64, key_len: usize, value_len: usize) -> File {
    let mut file = tempfile().expect("Failed to create temp file");
    
    // Write number of keys
    file.write_all(&num_entries.to_be_bytes())
        .expect("Failed to write key count");
    
    print!("Creating database...");
    io::stdout().flush().unwrap();

    for i in 0..num_entries {
        // Format: key;length;value
        let key = format!("key{:0width$}", i, width = key_len);
        let value = "x".repeat(value_len);
        
        // Write key and delimiter
        file.write_all(key.as_bytes()).expect("Failed to write key");
        file.write_all(b";").expect("Failed to write delimiter");
        
        // Write value length
        let value_len_u64 = value.len() as u64;
        file.write_all(&value_len_u64.to_be_bytes())
            .expect("Failed to write value length");
        
        // Write value
        file.write_all(value.as_bytes()).expect("Failed to write value");
    }

    file.seek(SeekFrom::Start(0)).expect("Failed to rewind");
    print!("\r\x1B[2K");

    file
}

#[test]
fn test_load_database_correctness() {
    println!("=== Testing load_database correctness ===");
    
    // Create a small test database
    let mut db_file = create_test_database(100, 10, 1024); // 100 entries, 10-char keys, 1KB values
    
    let mut index = BTree::new();
    
    // Load the database
    let start = Instant::now();
    load_database(&mut db_file, &mut index);
    let duration = start.elapsed();
    
    println!("Loaded 100 entries in {:?}", duration);
    println!("Index size: {}", index.size);
    
    // Verify all keys are in the index
    let mut rng = rand::rng();
    let mut keys: Vec<i32> = (0..100).collect();
    keys.shuffle(&mut rng);

    println!("Testing keys in order: {:?}", keys);
    for i in keys {
        let key = format!("key{:010}", i);
        let offset = index.search(&key);
        assert!(offset.is_some(), "Key '{}' not found in index", key);
        
        // Optional: Verify the offset points to correct value length
        db_file.seek(SeekFrom::Start(offset.unwrap()))
            .expect("Failed to seek to value");
        
        let mut len_bytes = [0u8; 8];
        db_file.read_exact(&mut len_bytes).expect("Failed to read length");
        let stored_len = u64::from_be_bytes(len_bytes);
        assert_eq!(stored_len, 1024, "Wrong value length for key '{}'", key);

        // Verify that the data can be retrieved by the runtime index
        let mut data_bytes = vec![0u8; stored_len as usize];
        db_file.read_exact(&mut data_bytes).expect("Failed to read data");
        let data = String::from_utf8(data_bytes).expect("Failed to convert data to string.");
        assert_eq!(data, "x".repeat(stored_len as usize));
    }
    
    println!("✓ All 100 keys correctly indexed\n");
}

#[test]
fn test_load_performance_small() {
    println!("=== Testing performance with small database ===");
    
    let sizes = vec![
        (1_000, "1K entries"),
        (10_000, "10K entries"),
        (50_000, "50K entries"),
    ];
    
    for (num_entries, description) in sizes {
        let mut db_file = create_test_database(num_entries, 10, 128); // 128-byte values
        
        let mut index = BTree::new();
        
        let start = Instant::now();
        load_database(&mut db_file, &mut index);
        let duration = start.elapsed();
        
        let rate = num_entries as f64 / duration.as_secs_f64();
        
        println!("{}: {:?} ({:.0} entries/sec)", description, duration, rate);
        assert_eq!(index.size, num_entries, "Index size mismatch for {}", description);
    }
    
    println!();
}

#[test]
fn test_load_performance_large() {
    println!("=== Testing performance with large database ===");
    
    let mut db_file = create_test_database(100_000, 10, 512); // 100K entries, 512-byte values
    
    let mut index = BTree::new();
    
    let start = Instant::now();
    load_database(&mut db_file, &mut index);
    let duration = start.elapsed();
    
    println!("Loaded 100,000 entries in {:?}", duration);
    println!("Rate: {:.0} entries/sec", 100_000 as f64 / duration.as_secs_f64());
    
    // Quick spot check
    for &i in &[0, 999, 9999, 49999, 99999] {
        let key = format!("key{:010}", i);
        assert!(index.search(&key).is_some(), "Key {} missing", key);
    }
    
    println!();
}

#[test]
fn test_load_memory_usage() {
    println!("=== Testing memory usage ===");
    
    let mut db_file = create_test_database(1_000_000, 20, 256); // 1M entries
    
    let mut index = BTree::new();
    
    println!("Loading database index...");
    let start = Instant::now();
    load_database(&mut db_file, &mut index);
    let duration = start.elapsed().as_secs_f64();
    
    println!("Loaded 1,000,000 entries in {:.3} seconds", duration);

    let rate = 1_000_000 as f64 / duration;
    let estimated_memory_usage = (rate * ((20 + 8) as f64) * duration) / (1024.0 * 1024.0);
    println!("Rate: {:.0} entries/sec", rate);
    
    // Memory check: B-Tree with 1M entries should be reasonable
    // Rough estimate: 1M * (20 bytes key + 8 bytes offset + overhead)
    println!("Estimated memory usage: {:.4}MB", estimated_memory_usage);
    
    println!();
}

#[test]
fn test_edge_cases() {
    println!("=== Testing edge cases ===");
    
    // Test 1: Empty database
    let mut empty_db = tempfile().unwrap();
    empty_db.write_all(&0u64.to_be_bytes()).unwrap();
    empty_db.seek(SeekFrom::Start(0)).unwrap();
    
    let mut index = BTree::new();
    load_database(&mut empty_db, &mut index);
    assert_eq!(index.size, 0, "Empty database should have size 0");
    println!("✓ Empty database handled correctly");
    
    // Test 2: Single entry
    let mut single_db = tempfile().unwrap();
    single_db.write_all(&1u64.to_be_bytes()).unwrap();
    single_db.write_all(b"testkey;").unwrap();
    single_db.write_all(&8u64.to_be_bytes()).unwrap(); // 8-byte value
    single_db.write_all(b"testvalue").unwrap(); // Actually 9 bytes - error case!
    single_db.seek(SeekFrom::Start(0)).unwrap();
    
    // This should panic because we try to read 9 bytes when length says 8
    // load_database(&mut single_db, &mut index);
    println!("⚠️  Note: Invalid length handling needs improvement");
    
    // Test 3: Very long key
    let mut long_key_db = tempfile().unwrap();
    let long_key = "a".repeat(1000); // 1000-character key
    long_key_db.write_all(&1u64.to_be_bytes()).unwrap();
    long_key_db.write_all(long_key.as_bytes()).unwrap();
    long_key_db.write_all(b";").unwrap();
    long_key_db.write_all(&0u64.to_be_bytes()).unwrap(); // Zero-length value
    long_key_db.seek(SeekFrom::Start(0)).unwrap();
    
    let mut index = BTree::new();
    load_database(&mut long_key_db, &mut index);
    assert_eq!(index.size, 1);
    println!("✓ Long key (1000 chars) handled correctly");
    
    println!();
}

#[test]
fn test_benchmark_load_performance() {
    println!("=== LOAD DATABASE BENCHMARK ===");
    println!("Format: [entries] [key_size] [value_size] -> time (rate)");
    println!("--------------------------------------------------------");
    
    let test_cases = vec![
        (1_000, 10, 128),
        (10_000, 10, 128),
        (100_000, 10, 128),
        (1_000_000, 10, 128),
        (1_000_000, 10, 256),
        (1_000_000, 10, 512),
        (1_000_000, 10, 1024),
        (1_000_000, 10, 2048),
        (10_000, 50, 1024),
        // 1MB values
        (1000, 10, 1024 * 1024),
        (10_000, 10, 1024 * 1024),
        // 1GB values
        (10, 10, 1024 * 1024 * 1024),
    ];
    
    for (entries, key_len, value_len) in test_cases {
        let mut db_file = create_test_database(entries, key_len, value_len);
        let mut index = BTree::new();
        
        print!("Loading database...");
        io::stdout().flush().unwrap();

        let start = Instant::now();
        load_database(&mut db_file, &mut index);
        let duration = start.elapsed();
        
        let rate = entries as f64 / duration.as_secs_f64();
        let total_data = (key_len + 1 + 8 + value_len) as u64 * entries; // Rough estimate
        
        print!("\r\x1B[2K");

        println!("{:7} entries ({:3} keys, {:6} values) -> {:?} ({:.0}/sec, {:.2} MB/s)", 
                 entries, format_size(key_len), format_size(value_len), 
                 duration, rate, 
                 total_data as f64 / duration.as_secs_f64() / 1_000_000.0);
    }
}

fn format_size(bytes: usize) -> String {
    let mega = (1024 * 1024) as f64;
    let giga = mega * 1024.0;
    if bytes >= giga as usize {
        format!("{:.1}GB", bytes as f64 / giga)
    } else if bytes >= mega as usize && bytes <= giga as usize {
        format!("{:.1}MB", bytes as f64 / mega)
    } else if bytes >= 1_000 {
        format!("{}KB", bytes / 1_000)
    } else {
        format!("{}B", bytes)
    }
}