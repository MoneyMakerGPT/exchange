use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    BUY,
    SELL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderInput {
    market: String,
    price: Decimal,
    quantity: Decimal,
    side: OrderSide,
    user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderInput {
    order_id: String,
    user_id: String,
    price: Decimal,
    side: OrderSide,
    market: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenOrdersInput {
    user_id: String,
    market: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderRequests {
    CreateOrder(CreateOrderInput),
    CancelOrder(CancelOrderInput),
    GetOpenOrders(GetOpenOrdersInput),
}
