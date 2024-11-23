use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CoinInfo {
    pub code: String,
    pub name: String,
    pub price_provider: PriceInfoProvider,
    pub ath_provider: AthInfoProvider,
    pub market_cap_provider: MarketCapProvider,
}


#[derive(Deserialize, Debug)]
pub enum PriceInfoProvider {
    CoinAPI
}


#[derive(Deserialize, Debug)]
pub enum AthInfoProvider {
    CoinAPI
}


#[derive(Deserialize, Debug)]
pub enum MarketCapProvider {
    CoinAPI
}