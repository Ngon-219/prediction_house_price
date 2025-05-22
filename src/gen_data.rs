use std::error::Error;
use csv::Writer;
use rand::Rng;

pub fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data.csv";
    let mut wtr = Writer::from_path(file_path)?;

    wtr.write_record(&["size", "bedrooms", "price"])?;

    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let size = rng.gen_range(500..5000);         
        let bedrooms = rng.gen_range(1..6);          
        let price = rng.gen_range(5000000..100000000);   

        wtr.write_record(&[
            size.to_string(),
            bedrooms.to_string(),
            price.to_string(),
        ])?;
    }

    wtr.flush()?;
    println!("Đã tạo thành công file '{}'", file_path);

    Ok(())
}
