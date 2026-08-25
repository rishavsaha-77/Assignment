use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let products: Vec<Product> = serde_json::from_str(&data).expect("Failed to parse json");

    for p in products {
        let clean_name: String = p.name.split_whitespace().collect::<Vec<&str>>().join(" ");
        println!("{}", clean_name);
    }
}
