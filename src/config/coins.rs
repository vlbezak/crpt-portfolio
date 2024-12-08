use serde::Deserialize;
use std::collections::HashMap;
use crate::Result;

use super::read_json_config;

#[derive(Debug, Deserialize)]
pub struct CoinsData {
    pub coins: Vec<CoinDef>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinDef {
    pub code: String,
    #[serde(default)]
    pub price_provider: PriceProviderEnum,
    //pub all_time_high_provider: Provider,
    #[serde(default)]
    pub price_provider_data: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum PriceProviderEnum {
    CoinAPI,
    CoinGecko,
    CryptoCompare,
    // Add more providers here if needed
}

impl Default for PriceProviderEnum {
    fn default() -> Self {
        PriceProviderEnum::CoinAPI
    }
}

pub fn read_default_coins_config() -> Result<CoinsData> {
    Ok(read_json_config("conf/coins.json")?)
}