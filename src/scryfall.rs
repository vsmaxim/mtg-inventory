use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct BulkDataResponse {
    object: String,
    has_more: bool,
    data: Vec<BulkData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BulkData {
    object: String,
    id: String,
    #[serde(rename = "type")]
    bulk_type: String,
    updated_at: DateTime<Utc>,
    uri: String,
    name: String,
    description: String,
    size: u64,
    download_uri: String,
    content_type: String,
    content_encoding: String,
}

async fn get_bulk_data() -> Result<BulkDataResponse> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "MTGInventory/0.1".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    Ok(client
        .get("https://api.scryfall.com/bulk-data")
        .send()
        .await?
        .json::<BulkDataResponse>()
        .await?)
}

pub async fn get_all_cards_download_link() -> Result<String> {
    let bulk_data = get_bulk_data().await?;
    for data in bulk_data.data.iter() {
        if data.bulk_type == "all_cards" {
            return Ok(data.download_uri.clone());
        }
    }

    panic!("No `all_cards` entry returned from scryfall api")
}
