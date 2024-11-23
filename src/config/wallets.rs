use serde::Deserialize;
use crate::Result;

use super::read_json_config;

#[derive(Debug, Deserialize)]
pub struct WalletsData {
    pub wallets: Vec<WalletDef>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletDef {
    pub name: String,
    pub kind: String,
    pub address: String,
    pub holdings: Vec<CoinHolding>,
}

#[derive(Debug, Deserialize)]
pub struct CoinHolding {
    pub coin: String,
    pub amount: f64,
}

pub fn read_default_wallets_config() -> Result<WalletsData> {
    Ok(read_json_config("conf/wallets.json")?)
}