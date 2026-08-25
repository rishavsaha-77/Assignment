use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Product {
    
    price: u32,

}
    

fn find_min_max(prices: &[u32]) -> Option<(u32, u32)>{
    if prices.is_empty(){
    return None;
    }


let mut min = prices[0];
let mut max = prices[0];

for &price in prices {
    if price < min {
        min = price;
    }
    if price > max {
        max = price;
    }
}

Some((min, max))
}

fn main() {
    let file_path = "scraped_products_200 (1).json";
    let data = fs::read_to_string(file_path).expect("Fail to read Json");

    let products: Vec<Product> = serde_json::from_str(&data)
        .expect("Failed to parse JSON content");

    let prices: Vec<u32> = products.iter().map(|p| p.price).collect();

    match find_min_max(&prices) {
        Some((min, max)) => {
         
            println!("Lowest Price : {}", min);
            println!("Highest Price: {}", max);
        }
        None => println!("No prices found."),
    }
}






