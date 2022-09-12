fn main() {
    use smartcore::dataset::iris::load_dataset;
    use smartcore::linalg::naive::dense_matrix::DenseMatrix;
    use smartcore::neighbors::knn_classifier::KNNClassifier;
    use smartcore::metrics::accuracy;
    // Load Iris dataset
    let iris_data = load_dataset();
    // Turn Iris dataset into NxM matrix
    let x = DenseMatrix::from_array(
        iris_data.num_samples,
        iris_data.num_features,
        &iris_data.data,
    );
    // These are our target class labels
    let y = iris_data.target;
    // Fit KNN classifier to Iris dataset
    let knn = KNNClassifier::fit(
        &x,
        &y,        
        Default::default(),
    ).unwrap();
    let y_hat = knn.predict(&x).unwrap(); // Predict class labels
    // Calculate training error
    println!("accuracy: {}", accuracy(&y, &y_hat)); // Prints 0.96
}
