use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
    rating: f64,
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let products: Vec<Product> = serde_json::from_str(&data).expect("Failed to parse json");

    let mut sorted_products = products;
    sorted_products.sort_unstable_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap());

    for p in sorted_products {
        println!("{}: {}", p.name, p.rating);
    }
}
