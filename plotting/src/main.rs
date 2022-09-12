use std::process;

mod csv_reader;

fn main() {
    if let Err(err) = csv_reader::run() {
        println!("{}", err);
        process::exit(1);
    }
}