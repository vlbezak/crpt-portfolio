use async_trait::async_trait;

use crate::{ client::coingecko::CoinGeckoClient, model::{AthInfo, Currency, PriceInfo} };

use crate::Result;

use super::AthProvider;

pub struct CoinGeckoAthProvider {}

#[async_trait]
impl AthProvider for CoinGeckoAthProvider {
    async fn get_ath(
        &self,
        symbol: &str,
        additional_data: Option<&std::collections::HashMap<String, String>>,
        currencies: &Vec<Currency>
    ) -> Result<Vec<AthInfo>> {
        let client = CoinGeckoClient::new();

        let token_id: String = additional_data.map_or_else(
            || String::from(symbol),
            |map| map.get("token_id").map_or(String::from(symbol), |val| val.clone())
        );

        let mut token_ids = vec![token_id];
        //:TODO - currency
        let currency = "usd";    

        let market_data = client.get_coins_markets(token_ids, &currency).await?;

        let mut result = Vec::new();

        for record in market_data.iter() {
            let ath_info = AthInfo {
                coin: symbol.to_string(),
                ath: record.ath,
                ath_change_percentage: record.ath_change_percentage,
                ath_date: record.ath_date.clone(),
                atl: record.atl,
                atl_change_percentage: record.atl_change_percentage,    
                atl_date: record.atl_date.clone(),
            };
            result.push(ath_info);
        }

        Ok(result)
    }
}
