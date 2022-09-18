use std::process;

// mod iris_agg;
mod pima_eda;

fn main() {
    if let Err(err) = pima_eda::run() {
        println!("{}", err);
        process::exit(1);
    }
}