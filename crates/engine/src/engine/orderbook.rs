use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::engine::{AssetPair, Order, OrderSide};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub price: Decimal,
    pub quantity: Decimal,
    pub trade_id: u64,
    pub other_user_id: String,
    pub order_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrder {
    order_id: String,
    user_id: String,
    price: Decimal,
    side: OrderSide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOrderResult {
    pub executed_quantity: Decimal,
    pub fills: Vec<Fill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    bids: BTreeMap<Decimal, Vec<Order>>,
    asks: BTreeMap<Decimal, Vec<Order>>,
    asset_pair: AssetPair,
    trade_id: u64,
    last_update_id: u64,
}

impl OrderBook {
    pub fn new(asset_pair: AssetPair) -> OrderBook {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
            asset_pair,
            trade_id: 0,
            last_update_id: 0,
        }
    }

    pub fn ticker(&self) -> String {
        format!("{:?}_{:?}", self.asset_pair.base, self.asset_pair.quote)
    }

    pub fn process_order(&mut self, mut order: Order) -> ProcessOrderResult {
        let order_result: ProcessOrderResult;

        match order.side {
            OrderSide::BUY => {
                order_result = self.match_asks(&order);
                order.filled_quantity = order_result.executed_quantity;
                if order_result.executed_quantity < order.quantity {
                    self.bids
                        .entry(order.price)
                        .and_modify(|orders| orders.push(order.clone())) // If the price exists, append the order
                        .or_insert(vec![order]);
                }
                order_result
            }
            OrderSide::SELL => {
                order_result = self.match_bids(&order);

                if order_result.executed_quantity < order.quantity {
                    self.asks
                        .entry(order.price)
                        .and_modify(|orders| orders.push(order.clone())) // If the price exists, append the order
                        .or_insert(vec![order]);
                }
                order_result
            }
        }
    }

    pub fn match_asks(&mut self, order: &Order) -> ProcessOrderResult {
        let mut fills: Vec<Fill> = vec![];
        let mut executed_quantity: Decimal = dec!(0);

        for (_price, asks) in self.asks.iter_mut() {
            for ask in asks.iter_mut() {
                if order.price >= ask.price && executed_quantity < ask.quantity {
                    let filled_quantity =
                        std::cmp::min(ask.quantity - executed_quantity, ask.quantity);
                    self.trade_id += 1;

                    executed_quantity += filled_quantity;
                    ask.filled_quantity += filled_quantity;

                    fills.push(Fill {
                        price: ask.price,
                        quantity: filled_quantity,
                        trade_id: 1,
                        other_user_id: ask.user_id.clone(),
                        order_id: ask.order_id.clone(),
                    })
                }
            }

            // Remove asks that have been completely filled
            asks.retain(|ask| ask.filled_quantity < ask.quantity);
        }

        ProcessOrderResult {
            fills,
            executed_quantity,
        }
    }

    pub fn match_bids(&mut self, order: &Order) -> ProcessOrderResult {
        let mut fills: Vec<Fill> = vec![];
        let mut executed_quantity: Decimal = dec!(0);

        for (_price, bids) in self.bids.iter_mut().rev() {
            for bid in bids.iter_mut() {
                if order.price <= bid.price && executed_quantity < bid.quantity {
                    let filled_quantity =
                        std::cmp::min(bid.quantity - executed_quantity, bid.quantity);
                    self.trade_id += 1;

                    executed_quantity += filled_quantity;
                    bid.filled_quantity += filled_quantity;

                    fills.push(Fill {
                        price: bid.price,
                        quantity: filled_quantity,
                        trade_id: 1,
                        other_user_id: bid.user_id.clone(),
                        order_id: bid.order_id.clone(),
                    })
                }
            }

            // Remove bids that have been completely filled
            bids.retain(|bid| bid.filled_quantity < bid.quantity);
        }

        ProcessOrderResult {
            fills,
            executed_quantity,
        }
    }

    pub fn get_open_orders(&mut self, user_id: String) -> Vec<&Order> {
        self.bids
            .values()
            .chain(self.asks.values()) // Combine bids and asks
            .flat_map(|orders| orders.iter()) // Flatten the Vec<Order> for each price level
            .filter(|order| order.user_id == user_id)
            .collect()
    }

    pub fn cancel_order(&mut self, cancel_order: CancelOrder) -> Result<(), ()> {
        let cancel = |orders_map: &mut BTreeMap<Decimal, Vec<Order>>| {
            if let Some(orders) = orders_map.get_mut(&cancel_order.price) {
                if let Some(index) = orders
                    .iter()
                    .position(|order| order.order_id == cancel_order.order_id)
                {
                    orders.remove(index);
                    Ok(())
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        };

        match cancel_order.side {
            OrderSide::BUY => cancel(&mut self.bids),
            OrderSide::SELL => cancel(&mut self.asks),
        }
    }
}
