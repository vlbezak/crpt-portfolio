use std::collections::HashMap;

use async_trait::async_trait;
use coinapi::CoinAPIPriceProvider;
use coingecko::CoinGeckoAthProvider;
use cryptocompare::CryptoComparePriceProvider;

use crate::config::coins::{AthProviderEnum, CoinDef, PriceProviderEnum};
use crate::model::{AthInfo, Currency, PriceInfo};
use crate::Result;

mod coinapi;
mod cryptocompare;
mod coingecko;

#[async_trait]
pub trait PriceProvider: Send + Sync  {
    async fn get_price(&self, symbol: &str, additional_data: &Option<HashMap<String,String>>, currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>>;  

    async fn get_prices(&self, coins_definitions: &Vec<CoinDef>, currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>> {
        println!("Default get prices");
        println!("coins: {:?}", coins_definitions.len());
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

#[async_trait]
pub trait AthProvider {
    async fn get_ath(&self, symbol: &str, additional_data: Option<&HashMap<String,String>>, currencies: &Vec<Currency>) -> Result<Vec<AthInfo>>;
}

pub fn get_price_provider(price_provider_id: &PriceProviderEnum) -> Box<dyn PriceProvider> {
    match price_provider_id {
        PriceProviderEnum::CoinAPI => Box::new(CoinAPIPriceProvider {}),
        PriceProviderEnum::CoinGecko => unimplemented!(),
        PriceProviderEnum::CryptoCompare => Box::new(CryptoComparePriceProvider {})
    }
}

pub fn get_ath_provider(ath_provider_id: &AthProviderEnum) -> Box<dyn AthProvider> {
    match ath_provider_id {
        AthProviderEnum::CoinGecko => Box::new(CoinGeckoAthProvider {})
    }
}
