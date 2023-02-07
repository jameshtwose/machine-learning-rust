use std::process;

mod scatter;
mod heatmap;

fn main() {
    if let Err(err) = scatter::run() {
        println!("{}", err);
        process::exit(1);
    }
    if let Err(err) = heatmap::run() {
        println!("{}", err);
        process::exit(1);
    }
}