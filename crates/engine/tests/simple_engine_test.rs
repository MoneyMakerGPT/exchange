#[cfg(test)]
mod tests {
    use engine::engine::Engine;
    use engine::types::engine::{Asset, CreateOrder, OrderSide};
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_engine_creation() {
        let engine = Engine::new();
        assert_eq!(engine.orderbooks.len(), 0);
        assert_eq!(engine.balances.len(), 0);
    }

    #[test]
    fn test_init_user_balance() {
        let mut engine = Engine::new();
        let user_id = "test_user";
        
        engine.init_user_balance(user_id);
        
        assert!(engine.balances.contains_key(user_id));
    }

    #[test]
    fn test_check_and_lock_funds_insufficient_funds() {
        let mut engine = Engine::new();
        let user_id = "test_user";
        
        // 初始化用户余额
        engine.init_user_balance(user_id);
        
        // 创建一个需要更多资金的买单
        let order = CreateOrder {
            market: "SOL_USDC".to_string(),
            price: dec!(1000000), // 价格过高，资金不足
            quantity: dec!(5),
            side: OrderSide::BUY,
            user_id: user_id.to_string(),
            pubsub_id: None,
        };
        
        // 检查并锁定资金应该失败
        let result = engine.check_and_lock_funds(&order);
        assert!(result.is_err());
    }
}