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

use clap::Parser;

/// Simple portfolio viewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the token in uppercase - for example ETH, BTC
    #[arg(short, long)]
    token: Option<String>,

    /// Wallet name - for example Ethereum MetaMask 1
    #[arg(short = 'n' , long)]
    wallet_name: Option<String>,

    /// Wallet kind - for example Ledger, MetaMask
    #[arg(short = 'k', long)]
    wallet_kind: Option<String>,

    /// Wallet address - address for the wallet
    #[arg(short = 'a', long)]
    wallet_address: Option<String>,

    /// currency 
    #[arg(short, long, default_value = "usd")]
    currency: Currency
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    println!("Portfolio");

    dotenv().ok();
    
    //let coin_filter = Some(String::from("ETH"));
    let coin_filter = args.token;

    let prices = coins::get_all_coins_prices(&coin_filter).await?;
    let wallets = wallets::read_default_wallets_config()?;
    
//    let sum_filter : SumFilter = SumFilter::new(Some(String::from("ETH")), None, None, None, Currency::USD);
//    let sum_filter : SumFilter = SumFilter::new(None, None, None, None, Currency::USD);
    let sum_filter : SumFilter = SumFilter::new(coin_filter, args.wallet_name, args.wallet_kind, args.wallet_address, args.currency);

    println!("Getting report for {:?}", sum_filter);
    let _holdings_value = report_holdings(wallets, &prices, &sum_filter);

    Ok(())
}