
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    Create,
    Delete,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "type_op")]
    pub order_type: OrderType,
    pub account_id: String,
    #[serde(with = "string_decimal")]
    pub amount: Decimal,
    pub order_id: String,
    pub pair: String,
    #[serde(rename = "limit_price", with = "string_decimal")]
    pub price: Decimal,
    pub side: OrderSide,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderbookEntry {
    pub order_id: String,
    pub account_id: String,
    #[serde(with = "string_decimal")]
    pub amount: Decimal,
    #[serde(with = "string_decimal")]
    pub price: Decimal,
    pub side: OrderSide,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orderbook {
    pub buy: BTreeMap<String, Vec<OrderbookEntry>>,
    pub sell: BTreeMap<String, Vec<OrderbookEntry>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub buy_order_id: String,
    pub sell_order_id: String,
    #[serde(with = "string_decimal")]
    pub amount: Decimal,
    #[serde(with = "string_decimal")]
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub orderbook: Orderbook,
    pub trades: Vec<Trade>,
}

impl Orderbook {
    pub fn new() -> Self {
        Orderbook {
            buy: BTreeMap::new(),
            sell: BTreeMap::new(),
        }
    }
}

// Helper module for serializing Decimals as strings
mod string_decimal {
    use rust_decimal::Decimal;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Decimal>().map_err(serde::de::Error::custom)
    }
}
