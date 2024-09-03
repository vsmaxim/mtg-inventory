use anyhow::Result;
use curl::easy::Easy;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// TODO: Better visual data
pub fn download_file(url: &str, output_path: &str) -> Result<()> {
    let mut file = File::create(Path::new(output_path))?;
    let mut easy = Easy::new();

    easy.url(url)?;
    easy.progress(true)?;
    easy.follow_location(true)?;

    easy.progress_function(|total_download, downloaded, _, _| {
        if total_download > 0.0 {
            print!(
                "\rDownload progress: {:.2}%",
                (downloaded / total_download) * 100.0
            );
            std::io::stdout().flush().unwrap();
        }
        true
    })?;

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    })?;

    transfer.perform()?;
    println!("\nDownload completed!");

    Ok(())
}
