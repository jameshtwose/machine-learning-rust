use color_eyre::Result;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::*;
use smartcore::metrics::*;

pub fn run() -> Result<()> {
    //Iris data
    let x = DenseMatrix::from_2d_array(&[
        &[5.1, 3.5, 1.4, 0.2],
        &[4.9, 3.1, 1.4, 0.2],
        &[4.7, 3.2, 1.3, 0.2],
        &[4.6, 3.1, 1.1, 0.2],
        &[5.0, 3.6, 1.4, 0.2],
        &[5.4, 6.0, 1.0, 0.4],
        &[4.6, 3.4, 1.4, 0.3],
        &[5.0, 3.4, 1.5, 0.2],
        &[4.4, 2.9, 1.4, 0.2],
        &[4.9, 3.1, 1.5, 0.1],
        &[7.0, 3.2, 4.7, 1.4],
        &[6.4, 3.2, 4.5, 1.5],
        &[6.9, 3.1, 4.9, 1.5],
        &[5.5, 2.3, 4.0, 1.3],
        &[6.5, 2.8, 4.6, 1.5],
        &[5.7, 2.8, 4.5, 1.3],
        &[6.3, 3.3, 4.7, 1.6],
        &[4.9, 2.4, 3.3, 1.0],
        &[6.6, 2.9, 4.6, 1.3],
        &[5.2, 2.7, 3.9, 1.4],
        ]);
    let y: Vec<i32> = vec![
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1,
    ];

    // Fit model
    let lr = LogisticRegression::fit(&x, &y, Default::default()).unwrap();

    // Predict
    let y_hat = lr.predict(&x).unwrap();

    // Print results
    println!("y_hat: {:?}", y_hat);

    let acc = accuracy(&y, &y_hat);
    println!("accuracy: {:?}", acc);

    Ok(())
}
