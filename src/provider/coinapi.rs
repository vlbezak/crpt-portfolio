use async_trait::async_trait;

use crate::client::coinapi::CoinAPIClient;
use crate::client::CurrentPriceApi;
use crate::Result;
use crate::model::{Currency, PriceInfo};

use super::PriceProvider;

pub struct CoinAPIPriceProvider {

}

#[async_trait]
impl PriceProvider for CoinAPIPriceProvider {
    async fn get_price(&self, symbol: &str, _additional_data: &Option<std::collections::HashMap<String,String>>, _currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>> {
        let client  = CoinAPIClient::new();
        Ok(client.get_prices(symbol).await?)
    }
}