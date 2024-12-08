use std::fs::{ self, File };

use chrono::Local;
use serde_json::to_writer_pretty;

use crate::config::read_json_config;
use crate::model::PriceInfo;
use crate::Result;
use super::CoinPriceStore;

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

        write_prices_as_json_to_file(&dir_and_filename, &prices)?;

        Ok(filename)
    }

    fn read_latest_prices(&self) -> Result<Option<Vec<PriceInfo>>> {
        if let Some(filename) = get_latest_prices_filename(&self.dir_name)? {
            return Ok(Some(read_json_config(&filename)?))
        }
        Ok(None)
    }
}

fn write_prices_as_json_to_file(filename: &str, data: &Vec<PriceInfo>) -> Result<()> {
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
