use serde::Deserialize;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Deserialize)]
struct Product {
    product_id: String,
   
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let products: Vec<Product> = serde_json::from_str(&data).expect("Failed to parse json");

    let mut seen = HashSet::new();
    let mut unique_products = Vec::new();

    for p in products {
        if seen.insert(p.product_id.clone()) {
            unique_products.push(p);
        }
    }

    println!("Original count: {}", 200);
    println!("Deduplicated count: {}", unique_products.len());
}