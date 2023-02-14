use std::process;

// mod iris_agg;
mod pima_eda;
mod pima_classification;

fn main() {
    if let Err(err) = pima_eda::run() {
        println!("{}", err);
        process::exit(1);
    }
    // if let Err(err) = pima_classification::run() {
    //     println!("{}", err);
    //     process::exit(1);
    // }
}