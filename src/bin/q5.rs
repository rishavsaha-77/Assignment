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

    let current_ids: HashSet<String> = products.into_iter().map(|p| p.product_id).collect();

    let old_ids = vec![
        "P001".to_string(),
        "P002".to_string(),
        "P999".to_string(),
        "P888".to_string(),
    ];

    for id in old_ids {
        if !current_ids.contains(&id) {
            println!("Unseen ID: {}", id);
        }
    }
}
