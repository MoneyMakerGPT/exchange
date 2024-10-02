use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WsMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u32,
}

impl WsMessage {
    pub fn parse_subscription(&self) -> Option<(SubscriptionType, SupportedAssetPairs)> {
        if self.params.is_empty() {
            return None;
        }

        let subscription_id = &self.params[0];
        let parts: Vec<&str> = subscription_id.split('.').collect();

        if parts.len() != 2 {
            return None;
        }

        let subscription_type_str = parts[0];
        let asset_pair_str = parts[1];

        let subscription_type = SubscriptionType::from_str(subscription_type_str)?;
        let asset_pair = SupportedAssetPairs::from_str(asset_pair_str).ok()?;

        Some((subscription_type, asset_pair))
    }
}

#[derive(Debug, Clone)]
pub enum SubscriptionType {
    Depth,
    Trade,
    Ticker,
}

impl SubscriptionType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "depth" => Some(SubscriptionType::Depth),
            "trade" => Some(SubscriptionType::Trade),
            "ticker" => Some(SubscriptionType::Ticker),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SupportedAssetPairs {
    BTCUSDT,
    ETHUSDT,
    SOLUSDT,
}

impl SupportedAssetPairs {
    pub fn from_str(asset_pair_str: &str) -> Result<SupportedAssetPairs, &'static str> {
        match asset_pair_str {
            "BTC_USDT" => Ok(SupportedAssetPairs::BTCUSDT),
            "ETH_USDT" => Ok(SupportedAssetPairs::ETHUSDT),
            "SOL_USDT" => Ok(SupportedAssetPairs::SOLUSDT),
            _ => Err("Unsupported asset pair"),
        }
    }
}
