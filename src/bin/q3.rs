use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Product {
    category: String,
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let products: Vec<Product> = serde_json::from_str(&data).expect("Failed to parse json");

    let mut counts = HashMap::new();

    for p in products {
        let count = counts.entry(p.category).or_insert(0);
        *count += 1;
    }

    for (category, count) in counts {
        println!("{}: {}", category, count);
    }
}