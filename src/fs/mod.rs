use crate::prelude::Result;
use async_recursion::async_recursion;

use log::{debug, info};
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[async_recursion(?Send)]
pub async fn list_files_recursively(dir: PathBuf) -> Result<()> {
    let mut entries = fs::read_dir(dir).await?;

    while let Some(res) = entries.next_entry().await? {
        let path = res.path();
        if path.is_dir() {
            list_files_recursively(path.clone()).await?;
        } else {
            info!("Processing: {}", path.display());

            let mut file = File::open(path).await?;

            let mut buffer = String::new();
            file.read_to_string(&mut buffer).await?;
            debug!(
                "Contents: {:?}",
                buffer.chars().take(10).collect::<Vec<char>>()
            );
        }
    }
    Ok(())
}
