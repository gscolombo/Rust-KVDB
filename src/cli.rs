mod db;

fn show_help() {
    println!(
        "
Key-Value Binary File Database Manager

USAGE:
    kvdb [OPTIONS] <DATABASE> <COMMAND> [ARGS...]

OPTIONS:
    -h, --help      Print this help message

DATABASE:
    Name of the database file (e.g., 'mydb' creates/uses 'mydb.kvdb')

COMMANDS:
    insert <KEY> <FILE>     Insert a file into the database
    search <KEY>            Search and output a file to stdout
    delete <KEY>            Delete a key from the database
    list                    List all keys in the database
    info                    Show database statistics

EXAMPLES:
    db mydb insert config settings.json
    db mydb search config > restored.json
    db mydb delete old_config
    db mydb list
    db mydb info

NOTES:
    - Database files are stored with .kvdb extension
    - Values are automatically compressed to save space
    "
    );
}

pub fn init(input: &Vec<String>) {
    match &*input[1] {
        "-h" | "--help" => show_help(),
        _ => match &*input[2] {
            "insert" => {
                assert_eq!(
                    input.len(),
                    5,
                    "Invalid number of arguments\nUse \"-h\" or \"--help\" for usage instructions."
                );

                println!(
                    "Operation: INSERT RECORD {} INTO {}\n",
                    &input[4], &input[1]
                );

                match db::insert_record(&input[1], &input[3], &input[4]) {
                    Ok(..) => println!("Operation finished successfully"),
                    Err(e) => panic!("Error during operation: {e:?}"),
                };
            }
            _ => {
                println!("Invalid option");
                show_help();
            }
        },
    };
}
