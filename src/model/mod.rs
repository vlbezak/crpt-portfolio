use std::str::FromStr;
use serde::{ Deserialize, Serialize };

use clap::ValueEnum;

//use core::Result;

#[derive(Debug, PartialEq, Clone, ValueEnum, Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
            "eur" | "EUR" => Ok(Currency::EUR),
            "usd" | "USD" => Ok(Currency::USD),
            _ => Err(format!("Cannot convert {} into currency", val).into()),
        }
    }
}

impl Currency {
    fn as_str_pair(&self) -> (&'static str, &'static str) {
        match self {
            Currency::EUR => ("EUR", "eur"),
            Currency::USD => ("USD", "usd"),
        }
    }

    pub fn to_uppercase_str(&self) -> &'static str {
        self.as_str_pair().0
    }

//    pub fn to_lowercase_str(&self) -> &'static str {
//        self.as_str_pair().1
//    }

}



#[derive(Debug, Serialize, Deserialize)]
pub struct PriceInfo {
    pub coin: String,
    pub currency: Currency,
    pub value: f64,
    pub market_cap: f64,
    pub change_24h: f64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AthInfo {
    pub coin : String,
    pub ath: f64,
    pub ath_date: String,
    pub ath_change_percentage: f64,
    pub atl: f64,
    pub atl_date: String,
    pub atl_change_percentage: f64,

}


#[derive(Debug, ValueEnum, Clone)]
pub enum ReportSortBy {
    Token,
    Amount,
    Value,
    MktCap,
    Change24h,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum ReportOrder {
    Asc,
    Desc,
}


