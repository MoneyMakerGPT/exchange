use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WsMessage {
    pub method: String,
    pub params: Vec<String>,
    pub id: u32,
}

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

    pub fn to_asset_pair(&self) -> AssetPair {
        match self {
            SupportedAssetPairs::BTCUSDT => AssetPair {
                base: Asset::BTC,
                quote: Asset::USDT,
            },
            SupportedAssetPairs::ETHUSDT => AssetPair {
                base: Asset::ETH,
                quote: Asset::USDT,
            },
            SupportedAssetPairs::SOLUSDT => AssetPair {
                base: Asset::SOL,
                quote: Asset::USDT,
            },
        }
    }
}

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
    pub base: Asset,
    pub quote: Asset,
}
