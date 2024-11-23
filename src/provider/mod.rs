use std::collections::HashMap;

use async_trait::async_trait;
use coinapi::CoinAPIPriceProvider;

use crate::config::coins::PriceProviderEnum;
use crate::model::PriceInfo;
use crate::Result;

mod coinapi;

#[async_trait]
pub trait PriceProvider: Send + Sync  {
    async fn get_price(&self, symbol: &str, additional_data: Option<HashMap<String,String>>) -> Result<Vec<PriceInfo>>;  
}

pub fn get_price_provider(price_provider_id: PriceProviderEnum) -> Box<dyn PriceProvider> {
    match price_provider_id {
        PriceProviderEnum::CoinAPI => Box::new(CoinAPIPriceProvider {}),
        PriceProviderEnum::CoinGecko => Box::new(CoinAPIPriceProvider {})
    }
}
