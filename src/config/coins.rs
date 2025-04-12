use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{client::coingecko::Coin, Result};

use super::read_json_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinsData {
    pub coins: Vec<CoinDef>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoinDef {
    pub code: String,
    #[serde(default)]
    pub price_provider: PriceProviderEnum,
    #[serde(default)]
    pub price_provider_data: Option<HashMap<String, String>>,
    #[serde(default)]
    pub ath_provider: AthProviderEnum,
    #[serde(default)]
    pub ath_provider_data: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum PriceProviderEnum {
    CoinAPI,
    CoinGecko,
    CryptoCompare,
    // Add more providers here if needed
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum AthProviderEnum {
    CoinGecko
}

impl Default for PriceProviderEnum {
    fn default() -> Self {
        PriceProviderEnum::CryptoCompare
    }
}

impl Default for AthProviderEnum {
    fn default() -> Self {
        AthProviderEnum::CoinGecko
    }
}

pub fn read_default_coins_config() -> Result<CoinsData> {
    Ok(read_json_config("conf/coins.json")?)
}

impl CoinsData {
    pub fn get_coin_def(&self, code: &str) -> Option<&CoinDef> {
        self.coins.iter().find(|coin_def| coin_def.code == code)
    }
}