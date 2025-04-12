use chrono::Local;
use filestore::CoinPriceFileStore;

use crate::model::{AthInfo, PriceInfo};
use crate::provider::get_price_provider;
use crate::service::ReportFilter;
use crate::Result;
use crate::config::coins::{ read_default_coins_config, CoinsData };

pub(crate) mod filestore;
pub mod update_prices;
pub(crate) mod update_ath;

use std::fs::{ File };

//:TODO - asi by sme mali ignorovat filter, lebo potom sa nevratia vsetky coins
pub async fn get_coins_prices(filter: &ReportFilter) -> Result<Vec<PriceInfo>> {

    let coin_price_store = CoinPriceFileStore {
        dir_name: String::from("data"),
    };
    if let Some(latest_prices) = coin_price_store.read_latest_prices()? {
        println!("Found latest prices");
        return Ok(latest_prices);
    }

    //TODO -check if some coins are not missing
    println!("Getting coins config");
    let coins_data: CoinsData = read_default_coins_config()?;
    let price_info = get_coins_prices_for_coins_data(&filter, &coins_data).await?;

    println!("Storing prices");
    coin_price_store.write_prices(&price_info)?;

    Ok(price_info)

}


async fn get_coins_prices_for_coins_data(filter: &ReportFilter, coins_data: &CoinsData) -> Result<Vec<PriceInfo>> {
    let mut result_prices: Vec<PriceInfo> = Vec::new();
    for coin_def in coins_data.coins.iter() {
        if let Some(coin_filter) = filter.coin.as_ref() {
            if *coin_filter != coin_def.code {
                continue;
            }
        }

        //println!("getting info for coin: {}", coin_def.code);
        let price_provider = get_price_provider(&coin_def.price_provider);
        let coin_result = price_provider.get_price(
            &coin_def.code,
            &coin_def.price_provider_data,
            &Vec::new()
        ).await;

        if coin_result.is_err() {
            println!("Error reading coin: {} skipping", coin_def.code);
            continue;
        }

        let mut coin_result = coin_result.unwrap();
        println!("Price: {}:{:?}", coin_def.code, coin_result);
        result_prices.append(&mut coin_result);
    }

    Ok(result_prices)
}

trait CoinPriceStore {
    fn write_prices(&self, prices: &Vec<PriceInfo>) -> Result<String>;
    fn read_latest_prices(&self) -> Result<Option<Vec<PriceInfo>>>;
}

trait DataStore<T> {
    fn write_data(&self, data: &T ) -> Result<Vec<String>>;
    fn read_last_data_for_coin(&self, coin: &str) -> Result<Option<T>>;
    fn read_last_data_all(&self) -> Result<Option<T>>;
}

