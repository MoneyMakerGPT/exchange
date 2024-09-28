use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubsub_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderInput {
    order_id: String,
    user_id: String,
    price: Decimal,
    side: OrderSide,
    market: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubsub_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOpenOrdersInput {
    user_id: String,
    market: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pubsub_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderRequests {
    CreateOrder(CreateOrderInput),
    CancelOrder(CancelOrderInput),
    GetOpenOrders(GetOpenOrdersInput),
}
