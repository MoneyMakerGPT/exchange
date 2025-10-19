#[cfg(test)]
mod tests {
    use engine::engine::{AmountType, Engine};
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
        let user_balance = engine.balances.get(user_id).unwrap().lock().unwrap();
        assert_eq!(user_balance.user_id, user_id);
        assert!(user_balance.balance.contains_key(&Asset::USDC));
        assert!(user_balance.balance.contains_key(&Asset::SOL));
        
        let usdc_balance = user_balance.balance.get(&Asset::USDC).unwrap();
        assert_eq!(usdc_balance.available, dec!(1000000));
        assert_eq!(usdc_balance.locked, dec!(0));
        
        let sol_balance = user_balance.balance.get(&Asset::SOL).unwrap();
        assert_eq!(sol_balance.available, dec!(10000));
        assert_eq!(sol_balance.locked, dec!(0));
    }

    #[test]
    fn test_check_and_lock_funds_buy() {
        let mut engine = Engine::new();
        let user_id = "test_user";
        
        // 初始化用户余额
        engine.init_user_balance(user_id);
        
        // 创建买单
        let order = CreateOrder {
            market: "SOL_USDC".to_string(),
            price: dec!(100),
            quantity: dec!(5),
            side: OrderSide::BUY,
            user_id: user_id.to_string(),
            pubsub_id: None,
        };
        
        // 检查并锁定资金
        let result = engine.check_and_lock_funds(&order);
        assert!(result.is_ok());
        
        // 验证资金是否正确锁定
        let user_balance = engine.balances.get(user_id).unwrap().lock().unwrap();
        let usdc_balance = user_balance.balance.get(&Asset::USDC).unwrap();
        assert_eq!(usdc_balance.available, dec!(999500)); // 1000000 - 100*5
        assert_eq!(usdc_balance.locked, dec!(500)); // 100*5
    }

    #[test]
    fn test_check_and_lock_funds_sell() {
        let mut engine = Engine::new();
        let user_id = "test_user";
        
        // 初始化用户余额
        engine.init_user_balance(user_id);
        
        // 创建卖单
        let order = CreateOrder {
            market: "SOL_USDC".to_string(),
            price: dec!(100),
            quantity: dec!(5),
            side: OrderSide::SELL,
            user_id: user_id.to_string(),
            pubsub_id: None,
        };
        
        // 检查并锁定资金
        let result = engine.check_and_lock_funds(&order);
        assert!(result.is_ok());
        
        // 验证资金是否正确锁定
        let user_balance = engine.balances.get(user_id).unwrap().lock().unwrap();
        let sol_balance = user_balance.balance.get(&Asset::SOL).unwrap();
        assert_eq!(sol_balance.available, dec!(9995)); // 10000 - 5
        assert_eq!(sol_balance.locked, dec!(5)); // 5
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
        assert_eq!(result.err().unwrap(), "Insufficient funds");
    }

    #[test]
    fn test_update_balance_with_lock() {
        let mut engine = Engine::new();
        let user_id = "test_user";
        
        // 初始化用户余额
        engine.init_user_balance(user_id);
        
        // 更新用户余额
        let result = engine.update_balance_with_lock(
            user_id.to_string(),
            Asset::USDC,
            dec!(100),
            AmountType::AVAILABLE,
        );
        
        assert!(result.is_ok());
        
        // 验证余额是否正确更新
        let user_balance = engine.balances.get(user_id).unwrap().lock().unwrap();
        let usdc_balance = user_balance.balance.get(&Asset::USDC).unwrap();
        assert_eq!(usdc_balance.available, dec!(1000100)); // 1000000 + 100
    }
}