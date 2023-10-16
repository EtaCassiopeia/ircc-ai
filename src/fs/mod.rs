use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Error;
use async_recursion::async_recursion;
use futures::stream::StreamExt;
use log::{debug, info};
use tokio::fs;
use tokio::time::Duration;

use crate::{
	embeddings::{Embeddings, EmbeddingsModel},
	prelude::*
};

#[derive(Debug, Clone)]
pub struct FileEmbeddings {
	pub path: String,
	pub embeddings: Embeddings
}

async fn list_files_recursively(dir: PathBuf) -> Result<Vec<PathBuf>> {
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

pub async fn embed_path<M: EmbeddingsModel + Send + Sync + 'static>(model: Arc<M>, path: PathBuf) -> Result<Vec<FileEmbeddings>> {
	let files = list_files_recursively(path).await?;

	let file_embeddings_stream = futures::stream::iter(files.into_iter()).then(|path| {
		let model_clone = Arc::clone(&model);
		async move {
			let file_content = fetch_file_content(path.clone()).await?;
			let embeddings = model_clone.embed(&file_content)?;
			log::info!("Embeddings for {} calculated", path.display());
			Ok(FileEmbeddings {
				path: path.to_str().unwrap().to_string(),
				embeddings
			})
		}
	});

	let file_embeddings_results: Vec<Result<FileEmbeddings>> = file_embeddings_stream.collect().await;

	let file_embeddings: Result<Vec<FileEmbeddings>> = file_embeddings_results.into_iter().collect();

	log::info!("Calculated embeddings for {} files", file_embeddings.as_ref().map(Vec::len).unwrap_or(0));

	file_embeddings
}

async fn fetch_file_content(path: PathBuf) -> Result<String> {
	let timeout = Duration::from_secs(60); // Adjust the timeout as needed.
	let result = tokio::time::timeout(timeout, async {
		let file_content = fs::read_to_string(path).await?;
		anyhow::Ok(file_content)
	})
	.await;

	match result {
		Ok(Ok(content)) => Ok(content),
		Ok(Err(err)) => Err(err.into()),
		Err(_) => Err(Error::msg("File content fetching timed out."))
	}
}
