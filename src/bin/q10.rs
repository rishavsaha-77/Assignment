use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
struct Product {
   
    name: String,
    category: String,
    price: u32,
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Failed to read file");
    let products: Vec<Product> = serde_json::from_str(&data).expect("Failed to parse json");

    let mut grouped: HashMap<String, Vec<Product>> = HashMap::new();

    for p in products {
    grouped.entry(p.category.clone()).or_default().push(p);
}
    for (category, mut items) in grouped {
        items.sort_by(|a, b| b.price.cmp(&a.price));
        items.truncate(5);

        println!("Category: {}", category);
        for item in items {
            println!("  {} - ${}", item.name, item.price);
        }
    }
}
