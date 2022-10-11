use std::process;

mod scatter;

fn main() {
    if let Err(err) = scatter::run() {
        println!("{}", err);
        process::exit(1);
    }
}