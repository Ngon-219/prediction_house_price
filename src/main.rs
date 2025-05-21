use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::Array2;
use std::error::Error;
use csv::ReaderBuilder;

fn format_number(n: f64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut _chars = s.chars().rev().collect::<Vec<_>>();

    for (i, c) in _chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.insert(0, ' ');
        }
        result.insert(0, *c);
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path("data.csv")?;
    let mut features: Vec<Vec<f64>> = Vec::new();
    let mut targets: Vec<f64> = Vec::new();

    for result in rdr.records().skip(1) {
        let record = result?;
        let size: f64 = record[0].parse()?;
        let bedrooms: f64 = record[1].parse()?;
        let price: f64 = record[2].parse()?;
        features.push(vec![size, bedrooms]);
        targets.push(price);
    }

    let x = Array2::from_shape_vec((features.len(), 2), features.into_iter().flatten().collect())?;
    let y = ndarray::Array1::from(targets);

    let dataset = Dataset::new(x, y);

    let model = LinearRegression::default().fit(&dataset)?;

    let test = Array2::from_shape_vec((1, 2), vec![2000.0, 3.0])?;
    let prediction = model.predict(&test);
    println!("Dự đoán giá nhà: {} VND", format_number(f64::from(prediction[0]).round()));

    Ok(())
}
