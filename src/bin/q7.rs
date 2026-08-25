use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let products: Vec<Product> = serde_json::from_str(&data).expect("Failed to parse json");

    let budget = 5000;
    let mut seen = HashMap::new();

    for p in &products {
        if p.price <= budget {
            let needed = budget - p.price;

            if let Some(other_name) = seen.get(&needed) {
                println!("Found pair for budget {}:", budget);
                println!("1. {} (${})", other_name, needed);
                println!("2. {} (${})", p.name, p.price);
                return;
            }

            seen.insert(p.price, &p.name);
        }
    }

    println!("No pair found for budget {}", budget);
}
