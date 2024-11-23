use config::wallets;
use dotenv::dotenv;
use model::Currency;
use service::{report_holdings, SumFilter};

pub type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

mod client;
mod model;
mod service;
mod coins;
mod config;
mod provider;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Portfolio");

    dotenv().ok();
    
    //let coin_filter = Some(String::from("ETH"));
    let coin_filter = None;


    let prices = coins::get_all_coins_prices(&coin_filter).await?;
    let wallets = wallets::read_default_wallets_config()?;
    
//    let sum_filter : SumFilter = SumFilter::new(Some(String::from("ETH")), None, None, None, Currency::USD);
//    let sum_filter : SumFilter = SumFilter::new(None, None, None, None, Currency::USD);
    let sum_filter : SumFilter = SumFilter::new(coin_filter, None, None, None, Currency::USD);


    println!("Getting report for {:?}", sum_filter);
    let holdings_value = report_holdings(wallets, &prices, &sum_filter);

    Ok(())
}