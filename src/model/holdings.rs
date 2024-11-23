use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Holdings {
    pub holdings: Vec<Holding>,
}

#[derive(Deserialize, Debug)]
pub struct Holding {
    pub wallet: Wallet,
    pub coin: String,
    pub amount: f64,
}

#[derive(Deserialize, Debug)]
pub struct Wallet {
    pub name: String,
}


impl Holdings {
    pub fn new () -> Self {
        Self {
          holdings: Vec::new(),
        }
    }
}
