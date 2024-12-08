use std::collections::HashMap;

use async_trait::async_trait;
use coinapi::CoinAPIPriceProvider;
use cryptocompare::CryptoComparePriceProvider;

use crate::config::coins::{CoinDef, PriceProviderEnum};
use crate::model::{Currency, PriceInfo};
use crate::Result;

mod coinapi;
mod cryptocompare;

#[async_trait]
pub trait PriceProvider: Send + Sync  {
    async fn get_price(&self, symbol: &str, additional_data: &Option<HashMap<String,String>>, currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>>;  

    async fn get_prices(&self, coins_definitions: &Vec<CoinDef>, currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>> {
        println!("Default get prices");
        println!("coins: {:#?}", coins_definitions);
        let mut result = Vec::new();
        for coin_def in coins_definitions {
            //:TODO - skusit prepisat stream based
            let Ok(mut res) = self.get_price(&coin_def.code, &coin_def.price_provider_data, &currencies).await else {
                println!("Problem getting price data for: {}", coin_def.code);
                continue;
            };
            result.append(&mut res);
        }
        Ok(result) 
    }

}

pub fn get_price_provider(price_provider_id: &PriceProviderEnum) -> Box<dyn PriceProvider> {
    match price_provider_id {
        PriceProviderEnum::CoinAPI => Box::new(CoinAPIPriceProvider {}),
        PriceProviderEnum::CoinGecko => Box::new(CoinAPIPriceProvider {}),
        PriceProviderEnum::CryptoCompare => Box::new(CryptoComparePriceProvider {})
    }
}
