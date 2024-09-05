mod download;
mod scryfall;
mod database;

use download::download_file;
use scryfall::get_all_cards_download_link;
use database::convert_to_sqlite;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::time::Duration;
use std::time::SystemTime;


fn get_creation_date(path: &str) -> Result<SystemTime, std::io::Error> {
    let metadata = fs::metadata(path)?;
    metadata.modified()
}


fn is_stale(path: &str, expiry_duration: Duration) -> Result<bool> {
    // File is stale if it doesn't exist
    if !Path::new(path).exists() {
        return Ok(true);
    }

    let creation_date = get_creation_date(path)?;
    let now = SystemTime::now();
    let creation_duration = now.duration_since(creation_date)?;

    println!("Created {}s ago", creation_duration.as_secs());
    println!("Expiration {}s", expiry_duration.as_secs());

    Ok(creation_duration > expiry_duration)
}


fn main() -> Result<()> {
    fs::create_dir_all("data/")?;
    let cards_json = "data/all-cards.json";
    let expiration = Duration::from_secs(60 * 60 * 24);

    if is_stale(cards_json, expiration)? {
        let link = get_all_cards_download_link()?;
        println!("Downloading {} from {}", cards_json, link);
        download_file(&link, cards_json)?;
    }

    convert_to_sqlite(cards_json)?;

    Ok(())
}

