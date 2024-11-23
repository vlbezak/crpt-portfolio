use config::wallets;
use dotenv::dotenv;
use model::{Currency, ReportOrder, ReportSortBy};
use service::{ report_holdings, write_report, ReportFilter };

pub type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

mod client;
mod model;
mod service;
mod coins;
mod config;
mod provider;

use clap::{ Parser, Subcommand };

/// Simple portfolio viewer
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Report holdings for wallet or token
    Holdings {
        /// Name of the token in uppercase - for example ETH, BTC
        #[arg(short, long)]
        token: Option<String>,

        /// Wallet name - for example Ethereum MetaMask 1
        #[arg(short = 'n', long)]
        wallet_name: Option<String>,

        /// Wallet kind - for example Ledger, MetaMask
        #[arg(short = 'k', long)]
        wallet_kind: Option<String>,

        /// Wallet address - address for the wallet
        #[arg(short = 'a', long)]
        wallet_address: Option<String>,

        /// currency
        #[arg(short, long, default_value = "usd")]
        currency: Currency,

        /// group by token, when set, the tokens in different wallets are grouped per token
        #[arg(short, long, default_value = "false")]
        group_by_token: bool,

        /// Sort field for report
        #[arg(long, default_value = "value", value_enum)]
        sort_by: ReportSortBy,

        /// Sort order for report
        #[arg(long, default_value = "desc", value_enum)]
        order: ReportOrder,
    },

    ListWallets {
        /// Wallet name - for example Ethereum MetaMask 1
        #[arg(short = 'n', long)]
        wallet_names: bool,
    },
}


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Holdings { .. } => handle_holdings(&cli.command).await?,
        Commands::ListWallets { .. } => handle_list_wallets(&cli.command).await?,
    }

    Ok(())
}

async fn handle_holdings(holdings: &Commands) -> Result<()> {
    if
        let Commands::Holdings {
            token,
            wallet_name,
            wallet_kind,
            wallet_address,
            currency,
            group_by_token,
            sort_by,
            order
        } = holdings
    {
        let coin_filter = token;

        let prices = coins::get_all_coins_prices(&coin_filter).await?;
        let wallets = wallets::read_default_wallets_config()?;

        let report_filter: ReportFilter = ReportFilter::new(
            coin_filter.clone(),
            wallet_name.clone(),
            wallet_kind.clone(),
            wallet_address.clone(),
            currency.clone(),
            group_by_token.clone(),
            sort_by.clone(),
            order.clone(),
        );

        println!("Getting report for {:?}", report_filter);
        let report_lines = report_holdings(&wallets, &prices, &report_filter);
        write_report(&report_lines);
    }

    Ok(())
}

async fn handle_list_wallets(holdings: &Commands) -> Result<()> {
    if let Commands::ListWallets { wallet_names } = holdings {
        todo!("Not implemented");
    }

    Ok(())
}
