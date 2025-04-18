use crate::model::PriceInfo;
use crate::Result;

pub mod coingecko;
pub mod coinapi;
pub mod cryptocompare;

pub trait CurrentPriceApi {
     async fn get_prices(&self, symbol: &str) -> Result<Vec<PriceInfo>>;
 }