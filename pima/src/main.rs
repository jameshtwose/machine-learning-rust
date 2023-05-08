use std::process;

// mod iris_agg;
mod pima_eda;
mod pima_classification_lr;
mod pima_classification_general;
// mod smartcore_example;
// mod linfa_example;

fn main() {
    if let Err(err) = pima_eda::run() {
        println!("{}", err);
        process::exit(1);
    }
    // if let Err(err) = pima_classification_lr::run() {
    //     println!("{}", err);
    //     process::exit(1);
    // }
    if let Err(err) = pima_classification_general::run() {
        println!("{}", err);
        process::exit(1);
    }
    // if let Err(err) = smartcore_example::run() {
    //     println!("{}", err);
    //     process::exit(1);
    // }
    // if let Err(err) = linfa_example::run() {
    //     println!("{}", err);
    //     process::exit(1);
    // }
}