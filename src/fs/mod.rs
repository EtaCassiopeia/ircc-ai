use std::path::PathBuf;

use async_recursion::async_recursion;
use log::{debug, info};
use tokio::fs;

use crate::prelude::Result;

pub async fn list_files_recursively(dir: PathBuf) -> Result<Vec<PathBuf>> {
	#[async_recursion(?Send)]
	async fn helper(dir: PathBuf, files: &mut Vec<PathBuf>) -> Result<()> {
		info!("Processing: {}", dir.display());
		let mut entries = fs::read_dir(dir).await?;
		while let Some(res) = entries.next_entry().await? {
			let path = res.path();
			if path.is_dir() {
				helper(path.clone(), files).await?;
			} else {
				debug!("File: {}", path.display());
				files.push(path.clone());
			}
		}
		Ok(())
	}

	let mut files = Vec::new();
	helper(dir, &mut files).await?;
	Ok(files)
}
