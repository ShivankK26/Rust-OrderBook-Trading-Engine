
pub mod trading_engine;
pub mod models;

// Re-export key items for easier use
pub use models::{Order, Trade, Orderbook, ProcessingResult, OrderSide, OrderType};
pub use trading_engine::process_orders;
