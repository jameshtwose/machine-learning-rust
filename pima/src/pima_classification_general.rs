use color_eyre::Result;
use eval_metrics::classification::{BinaryConfusionMatrix, PrCurve, RocCurve};
use linfa::prelude::*;
// use linfa::dataset::{Records, Targets};
use linfa_logistic::LogisticRegression;
// use linfa_trees::DecisionTree;
use ndarray::{Array1, ArrayBase, OwnedRepr};
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;
use std::fs::OpenOptions;
use std::io::Write;


pub fn model_fit_predict_evaluate(
    x: ArrayBase<OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>,
    y: Array1<i64>,
) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("model_output.txt")?;

    let y_test = y.clone();

    // convert the data to a linfa dataset
    let data = DatasetBase::new(x, y);
    // fit the model
    let model = LogisticRegression::default().fit(&data).unwrap();
    // let model = DecisionTree::params().fit(&data).unwrap();

    // predict the target
    let y_pred = model.predict(&data);

    // print the intercept
    println!("intercept: {:?}", model.intercept());

    // print the params
    println!("params: {:?}", model.params());

    // print the confusion matrix
    let threshold = 0.5;
    let y_pred_vec: Vec<f64> = y_pred.iter().map(|x| *x as f64).collect();
    let y_test_vec: Vec<bool> = y_test.iter().map(|x| *x == 1).collect();
    let cm = BinaryConfusionMatrix::compute(&y_pred_vec, &y_test_vec, threshold)?;
    println!("confusion matrix:");
    println!("{}", cm);
    writeln!(file, "confusion matrix:").unwrap();
    writeln!(file, "{}", cm).unwrap();

    // print the metrics of interest
    println!("accuracy: {}", cm.accuracy()?);
    println!("precision: {}", cm.precision()?);
    println!("recall: {}", cm.recall()?);
    println!("f1: {}", cm.f1()?);

    // compute the roc curve
    let roc = RocCurve::compute(&y_pred_vec, &y_test_vec)?;

    // print the auc
    println!("auc: {}", roc.auc());

    // inspect the roc curve points
    roc.points.iter().for_each(|point| {
        let _tpr = point.tp_rate;
        let _fpr = point.fp_rate;
        let _thresh = point.threshold;
    });
    // print the roc curve points
    println!("roc curve points: {:?}", roc.points);

    // compute the pr curve
    let pr = PrCurve::compute(&y_pred_vec, &y_test_vec)?;

    // print the average precision
    println!("average precision: {}", pr.ap());

    // inspect the pr curve points
    pr.points.iter().for_each(|point| {
        let _precision = point.precision;
        let _recall = point.recall;
        let _thresh = point.threshold;
    });
    // print the pr curve points
    println!("pr curve points: {:?}", pr.points);

    Ok(())
}

pub fn run() -> Result<()> {
    println!("Classification");
    println!("==============");

    // specify the URL where the csv is located
    let csv_url = "https://raw.githubusercontent.com/jameshtwose/example_deliverables/main/classification_examples/pima_diabetes/diabetes.csv";

    // read in the data as a vector using the reqwest Client
    let data: Vec<u8> = Client::new().get(csv_url).send()?.text()?.bytes().collect();

    // convert the data to a polars data frame
    let df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;

    // specify the features
    let features = ["Glucose", "Insulin"];

    // specify the column name of the target
    let target = "Outcome";

    let x: ArrayBase<OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> =
        df.select(&features)?.to_ndarray::<Float64Type>().unwrap();

    let y: Array1<i64> = df
        .column(target)
        .unwrap()
        .i64()
        .unwrap()
        .into_iter()
        .map(|opt_val| opt_val.unwrap())
        .collect();

    model_fit_predict_evaluate(x, y)?;
    
    Ok(())
}
