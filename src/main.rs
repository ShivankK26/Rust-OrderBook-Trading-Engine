mod trading_engine;
mod models;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::models::{Order};
use crate::trading_engine::process_orders;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading orders from orders.json...");
    
    // Read orders from JSON file
    let file = File::open("orders.json")?;
    let reader = BufReader::new(file);
    let orders: Vec<Order> = serde_json::from_reader(reader)?;
    
    println!("Processing {} orders...", orders.len());
    
    // Process orders
    let result = process_orders(&orders)?;
    
    // Write results to files
    write_to_file("orderbook.json", &result.orderbook)?;
    write_to_file("trades.json", &result.trades)?;
    
    println!("Done! Processed {} orders, generated {} trades.", orders.len(), result.trades.len());
    println!("Results written to orderbook.json and trades.json");
    
    Ok(())
}

fn write_to_file<P: AsRef<Path>, T: serde::Serialize>(
    path: P,
    data: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(path, json)?;
    Ok(())
}
