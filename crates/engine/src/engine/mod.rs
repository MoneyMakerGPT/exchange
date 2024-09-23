pub mod engine;
pub mod orderbook;
pub mod error;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Asset {
    USDT,
    BTC,
    ETH,
    SOL,
}

impl Asset {
    pub fn from_str(asset_str: &str) -> Result<Asset, &'static str> {
        // static lifetime because Err str slice is static
        match asset_str {
            "USDT" => Ok(Asset::USDT),
            "BTC" => Ok(Asset::BTC),
            "ETH" => Ok(Asset::ETH),
            "SOL" => Ok(Asset::SOL),
            _ => Err("Unsupported asset"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AssetPair {
    base: Asset,
    quote: Asset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    BUY,
    SELL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    LIMIT,
    MARKET,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Pending,
    Filled,
    PartiallyFilled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    price: Decimal,
    quantity: Decimal,
    filled_quantity: Decimal,
    order_id: String,
    user_id: String,
    side: OrderSide,
    order_type: OrderType,
    order_status: OrderStatus,
    timestamp: i64, // chrono::Utc::now().timestamp_millis();
}
