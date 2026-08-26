use serde :: {Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product{
    pub product_id: String,
    pub name : String,
    pub price : u64,
    pub category : String,
    pub rating : f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "scraped_products_200.json";
    let chunk_size = 10;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);    

    let products: Vec<Product> = serde_json::from_reader(reader)?;
    let mut total_processed = 0;

    for chunk in products.chunks(chunk_size) {
        for product in chunk {
            total_processed += 1;
        }
    }
    println!("Processed {} products simply and cleanly!", total_processed);
    Ok(())
}
    


