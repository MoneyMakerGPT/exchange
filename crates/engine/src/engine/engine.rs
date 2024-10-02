use crate::engine::db::DbUpdates;
use crate::engine::orderbook::OrderBook;
use crate::engine::ws_stream::WsStreamUpdates;
use crate::types::engine::{
    Asset, AssetPair, CancelOrder, CreateOrder, GetOpenOrders, Order, OrderSide, OrderStatus,
    OrderType, ProcessOrderResult,
};
use redis::RedisManager;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

pub enum AmountType {
    AVAILABLE,
    LOCKED,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    available: Decimal,
    locked: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBalances {
    user_id: String,
    balance: HashMap<Asset, Amount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    pub orderbooks: Vec<OrderBook>,
    pub balances: HashMap<String, Mutex<UserBalances>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            orderbooks: vec![],
            balances: HashMap::new(),
        }
    }

    pub fn init_engine(&mut self) {
        let orderbook = OrderBook::new(AssetPair {
            base: Asset::BTC,
            quote: Asset::USDT,
        });

        self.orderbooks.push(orderbook);
    }

    pub fn init_user_balance(&mut self, user_id: &str) {
        let initial_balances = UserBalances {
            user_id: user_id.to_string(),
            balance: HashMap::new(),
        };

        // Add dummy values for USDT and BTC
        let usdt_balance = Amount {
            available: Decimal::new(1000, 2), // Dummy value: 1000 USDT (2 decimal places)
            locked: Decimal::new(0, 2),       // 0 locked
        };

        let btc_balance = Amount {
            available: Decimal::new(2, 1), // Dummy value: 2 BTC (8 decimal places)
            locked: Decimal::new(0, 1),    // 0 locked
        };

        // Initialize the balance HashMap for the user
        let mut balances_map = initial_balances.balance;
        balances_map.insert(Asset::USDT, usdt_balance);
        balances_map.insert(Asset::BTC, btc_balance);

        // Add the initialized UserBalances to the Engine's balances map
        self.balances.insert(
            user_id.to_string(),
            Mutex::new(UserBalances {
                user_id: user_id.to_string(),
                balance: balances_map,
            }),
        );
    }

    pub async fn create_order(
        &mut self,
        input_order: CreateOrder,
        redis_conn: &RedisManager,
    ) -> Result<(), &str> {
        self.check_and_lock_funds(&input_order)
            .expect("Funds check failed");

        let orderbook = self
            .orderbooks
            .iter_mut()
            .find(|orderbook| orderbook.ticker() == input_order.market)
            .expect("No matching orderbook found!");

        let assets: Vec<&str> = input_order.market.split('_').collect();
        let base_asset = Asset::from_str(assets[0]).unwrap();
        let quote_asset = Asset::from_str(assets[1]).unwrap();

        let order = Order {
            price: input_order.price,
            quantity: input_order.quantity,
            filled_quantity: dec!(0),
            order_id: uuid::Uuid::new_v4().to_string(),
            user_id: input_order.user_id.clone(),
            side: input_order.side,
            order_type: OrderType::MARKET,
            order_status: OrderStatus::Pending,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        let order_result: ProcessOrderResult = orderbook.process_order(order.clone());
        println!("Current orderbook bids {:?}", orderbook.bids);
        println!("Current orderbook asks {:?}", orderbook.asks);

        let _ = self.update_user_balance(base_asset, quote_asset, order.clone(), &order_result);
        let _ = self
            .update_db_orders(
                order.clone(),
                order_result.executed_quantity,
                &order_result.fills,
                redis_conn,
            )
            .await;

        let _ = self
            .create_db_trades(
                input_order.user_id.clone(),
                input_order.market.clone(),
                &order_result.fills,
                redis_conn,
            )
            .await;

        let _ = self
            .publish_ws_trades(
                input_order.market.clone(),
                input_order.user_id.clone(),
                &order_result.fills,
                redis_conn,
            )
            .await;

        let _ = self
            .publish_ws_depth_updates(
                input_order.market.clone(),
                order.price,
                order.side,
                &order_result.fills,
                redis_conn,
            )
            .await;

        Ok(())
    }

    pub fn cancel_order(&mut self, cancel_order: CancelOrder) -> Result<(), &str> {
        let orderbook = self
            .orderbooks
            .iter_mut()
            .find(|orderbook| orderbook.ticker() == cancel_order.market)
            .expect("No matching orderbook found!");

        let assets: Vec<&str> = cancel_order.market.split('_').collect();
        let base_asset_str = assets[0];
        let quote_asset_str = assets[1];
        let base_asset = Asset::from_str(base_asset_str)?;
        let quote_asset = Asset::from_str(quote_asset_str)?;

        let result = orderbook.cancel_order(cancel_order);

        match result {
            Ok(order) => {
                let quantity = match order.side {
                    OrderSide::BUY => (order.quantity - order.filled_quantity) * order.price,
                    OrderSide::SELL => order.quantity - order.filled_quantity,
                };

                match order.side {
                    OrderSide::BUY => {
                        self.update_balance_with_lock(
                            order.user_id.clone(),
                            quote_asset.clone(),
                            quantity,
                            AmountType::AVAILABLE,
                        )?;

                        self.update_balance_with_lock(
                            order.user_id.clone(),
                            quote_asset.clone(),
                            -quantity,
                            AmountType::LOCKED,
                        )?;
                    }

                    OrderSide::SELL => {
                        self.update_balance_with_lock(
                            order.user_id.clone(),
                            base_asset.clone(),
                            quantity,
                            AmountType::AVAILABLE,
                        )?;

                        self.update_balance_with_lock(
                            order.user_id.clone(),
                            base_asset.clone(),
                            -quantity,
                            AmountType::LOCKED,
                        )?;
                    }
                }
            }

            Err(()) => {
                println!("Failed to cancel order")
            }
        }

        Ok(())
    }

    pub fn get_open_orders(&mut self, open_orders: GetOpenOrders) -> Vec<&Order> {
        let orderbook = self
            .orderbooks
            .iter_mut()
            .find(|orderbook| orderbook.ticker() == open_orders.market)
            .expect("No matching orderbook found!");

        let open_orders: Vec<&Order> = orderbook.get_open_orders(open_orders.user_id);

        open_orders
    }

    pub fn check_and_lock_funds(&mut self, order: &CreateOrder) -> Result<(), &str> {
        let assets: Vec<&str> = order.market.split('_').collect();
        let base_asset_str = assets[0];
        let quote_asset_str = assets[1];

        // Convert string assets to Asset enum
        let base_asset = Asset::from_str(base_asset_str)?;
        let quote_asset = Asset::from_str(quote_asset_str)?;

        let user_id = &order.user_id;

        let user_balance_mutex = self
            .balances
            .get_mut(user_id)
            .ok_or("No matching user found")?;

        // Lock the Mutex to safely access the user's balances
        let mut user_balance = user_balance_mutex.lock().map_err(|_| "Mutex lock failed")?;

        match order.side {
            OrderSide::BUY => {
                let balance = user_balance
                    .balance
                    .get_mut(&quote_asset)
                    .ok_or("No balance for asset found")?;

                let total_cost = order.price * order.quantity;
                if balance.available >= total_cost {
                    balance.available -= total_cost;
                    balance.locked += total_cost;
                } else {
                    return Err("Insufficient funds");
                }
            }

            OrderSide::SELL => {
                // User must have order.quantity of base_asset
                let balance = user_balance
                    .balance
                    .get_mut(&base_asset)
                    .ok_or("No balance for asset found")?;

                if balance.available >= order.quantity {
                    balance.available -= order.quantity;
                    balance.locked += order.quantity;
                } else {
                    return Err("Insufficient asset quantity");
                }
            }
        }

        Ok(())
    }

    pub fn update_user_balance(
        &mut self,
        base_asset: Asset,
        quote_asset: Asset,
        order: Order,
        order_result: &ProcessOrderResult,
    ) -> Result<(), &str> {
        match order.side {
            OrderSide::BUY => {
                for fill in &order_result.fills {
                    // Update buyer's balances (current user)
                    self.update_balance_with_lock(
                        order.user_id.clone(),
                        base_asset.clone(),
                        fill.quantity,
                        AmountType::AVAILABLE,
                    )?;
                    self.update_balance_with_lock(
                        order.user_id.clone(),
                        quote_asset.clone(),
                        -(fill.price * fill.quantity),
                        AmountType::LOCKED,
                    )?;

                    // Update seller's balances (other user)
                    self.update_balance_with_lock(
                        fill.other_user_id.clone(),
                        quote_asset.clone(),
                        fill.price * fill.quantity,
                        AmountType::AVAILABLE,
                    )?;
                    self.update_balance_with_lock(
                        fill.other_user_id.clone(),
                        base_asset.clone(),
                        -fill.quantity,
                        AmountType::LOCKED,
                    )?;
                }
            }
            OrderSide::SELL => {
                for fill in &order_result.fills {
                    // Update seller's balances (current user)
                    self.update_balance_with_lock(
                        order.user_id.clone(),
                        base_asset.clone(),
                        -fill.quantity,
                        AmountType::LOCKED,
                    )?;
                    self.update_balance_with_lock(
                        order.user_id.clone(),
                        quote_asset.clone(),
                        fill.price * fill.quantity,
                        AmountType::AVAILABLE,
                    )?;

                    // Update buyer's balances (other user)
                    self.update_balance_with_lock(
                        fill.other_user_id.clone(),
                        base_asset.clone(),
                        fill.quantity,
                        AmountType::AVAILABLE,
                    )?;
                    self.update_balance_with_lock(
                        fill.other_user_id.clone(),
                        quote_asset.clone(),
                        -(fill.price * fill.quantity),
                        AmountType::LOCKED,
                    )?;
                }
            }
        }
        Ok(())
    }

    // Helper function to update balance with lock
    fn update_balance_with_lock(
        &self,
        user_id: String,
        asset: Asset,
        amount: Decimal,
        amount_type: AmountType,
    ) -> Result<(), &str> {
        // Access the user's balance via the Mutex
        let balances = &self.balances;
        let user_balance_mutex = balances.get(&user_id).ok_or("No matching user found")?;

        // Lock the Mutex to safely access the user's balances
        let mut user_balance = user_balance_mutex.lock().map_err(|_| "Mutex lock failed")?;

        let balance = user_balance
            .balance
            .get_mut(&asset)
            .ok_or("No balance for asset found")?;

        match amount_type {
            AmountType::AVAILABLE => balance.available += amount,
            AmountType::LOCKED => balance.locked += amount,
        }

        Ok(())
    }
}
