use std::time::Instant;

use serde::{Deserialize, Serialize};
use tokio::fs;
use anyhow::Result;
use tokio::io::AsyncBufReadExt;
use uuid::Uuid;
use rusqlite::{Connection, params};

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub id: Uuid,
    pub lang: String,
    pub name: String,
}

pub async fn convert_to_sqlite(path: &str) -> Result<()> {
    let start = Instant::now();

    // Open the input file
    let file = fs::File::open(path).await?;
    let reader = tokio::io::BufReader::new(file);

    let mut lines = reader.lines();
    let mut cnt = 0;

    // Create or open the SQLite database
    let mut conn = Connection::open("cards.db")?;

    // Create the cards table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            id TEXT PRIMARY KEY,
            lang TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        [],
    )?;

    // Start a transaction
    let tx = conn.transaction()?;

    while let Some(line) = lines.next_line().await? {
        let line = line.trim_end_matches(",");
        if line.len() == 1 {
            continue;
        }
        match serde_json::from_str::<Card>(&line) {
            Ok(card) => {
                // Insert the card into the database
                tx.execute(
                    "INSERT INTO cards (id, lang, name) VALUES (?, ?, ?)",
                    params![card.id.to_string(), card.lang, card.name],
                )?;
                cnt += 1;
            }
            Err(e) => {
                eprintln!("Error parsing card: {}. Line: {}", e, line);
                // Optionally, you might want to return this error or handle it differently
            }
        }
    }

    // Commit the transaction
    tx.commit()?;

    let elapsed = start.elapsed();
    println!("Inserted {} cards in {:.2} seconds", cnt, elapsed.as_secs_f64());

    Ok(())
}
