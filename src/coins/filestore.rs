use std::fs::{ self, File };
use std::path;

use chrono::Local;
use reqwest::header::Entry;
use serde::Serialize;
use serde_json::to_writer_pretty;

use crate::config::read_json_config;
use crate::model::{AthInfo, PriceInfo};
use crate::Result;
use super::{CoinPriceStore, DataStore};

pub struct CoinPriceFileStore {
    pub dir_name: String,
}

impl CoinPriceStore for CoinPriceFileStore {
    fn write_prices(&self, prices: &Vec<PriceInfo>) -> Result<String> {
        let now = Local::now(); // Get the current local date and time
        let formatted_time = now.format("%Y%m%d%H%M"); // Format as "yyyymmddhhMM"
        let filename = format!("prices-{}.json", formatted_time); // Construct the filename
        let dir_and_filename = format!("{}/{}", &self.dir_name, filename);

        if !std::path::Path::new(&self.dir_name).exists() {
            println!("Creating folder: {}", &self.dir_name);
            fs::create_dir(&self.dir_name)?;
        }

        write_data_json_to_file(&dir_and_filename, &prices)?;

        Ok(filename)
    }

    fn read_latest_prices(&self) -> Result<Option<Vec<PriceInfo>>> {
        if let Some(filename) = get_latest_prices_filename(&self.dir_name)? {
            return Ok(Some(read_json_config(&filename)?))
        }
        Ok(None)
    }
}

pub fn write_data_json_to_file<T>(filename: &str, data: &T) -> Result<()>
where T: Serialize {
    let file = File::create(filename)?;
    to_writer_pretty(file, &data)?;
    Ok(())
}

fn get_latest_prices_filename(dir_name: &str) -> Result<Option<String>> {
    let mut entries: Vec<_> = fs::read_dir(dir_name)?
        .filter_map(|entry| entry.ok())
        .filter(|entry|
            entry
                .file_type()
                .map(|t| t.is_file())
                .unwrap_or(false)
        )
        .map(|entry| entry.file_name().to_string_lossy()
        .into_owned())
        .collect();

    entries.sort_by(|a,b| b.cmp(a));

    if let Some(latest_file) = entries.first() {
        let file_path = format!("{}/{}", dir_name, latest_file);
        Ok(Some(file_path))
    }
    else {
        Ok(None)
    }
}


const DIR_PREFIX_COIN_INFO: &str = "data/coins/";

pub struct AdditionalDataStore {
}

impl AdditionalDataStore {
}

impl DataStore<Vec<AthInfo>> for AdditionalDataStore {
    fn write_data(&self, data: &Vec<AthInfo>) -> Result<Vec<String>> {
        
        let now = Local::now(); // Get the current local date and time
        let formatted_time = now.format("%Y%m%d%H%M"); // Format as "yyyymmddhhMM"

        let root_dir = path::Path::new(DIR_PREFIX_COIN_INFO);
        let mut filenames = Vec::new();

        for coin_data in data {
            let dirname = root_dir.join(&coin_data.coin);
            if !std::path::Path::new(&dirname).exists() {
                println!("Creating folder: {:?}", &dirname);
                fs::create_dir_all(&dirname)?;
            }
            else {
                println!("Folder exists: {:?}", &dirname);
            }

            let filename = format!("{}.json", formatted_time); // Construct the filename
            println!("Writing to file: {:?}", filename);
            let dir_and_filename = dirname.join(filename);
            let dir_and_filename_str = dir_and_filename.to_str().ok_or("Failed to create filename")?;
            write_data_json_to_file(&dir_and_filename_str, &coin_data)?;
            filenames.push(dir_and_filename_str.to_string());
        }

        Ok(filenames)
    }

    fn read_last_data_all(&self) -> Result<Option<Vec<AthInfo>>> {
        let dir = path::Path::new(DIR_PREFIX_COIN_INFO);
        if !dir.exists() {
            return Ok(None);
        }

        let mut data = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                
            }

            if path.is_file() {
                let data_str = std::fs::read_to_string(&path)?;
                let coin_data: AthInfo = serde_json::from_str(&data_str)?;
                data.push(coin_data);
            }
        }

        Ok(Some(data))
    }

    fn read_last_data_for_coin(&self, coin: &str) -> Result<Option<Vec<AthInfo>>> {
        let Ok(dirname)  = crate::utils::file_utils::find_subdir_with_name(path::Path::new(DIR_PREFIX_COIN_INFO), coin) else {
            return Ok(None);
        };
        let Some(dirname) = dirname else {
            return Ok(None);
        };
        let Some(filename) = crate::utils::file_utils::get_latest_filename(&dirname)? else {
            return Ok(None);
        };
        Ok(Some(read_json_config(filename.to_str().unwrap())?))
    }
}


