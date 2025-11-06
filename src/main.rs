use std::env;

mod cli;

fn main() {
    let args: Vec<String> = env::args().collect();

    cli::init(&args);
}
