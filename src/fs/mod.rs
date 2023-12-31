use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Error;
use async_recursion::async_recursion;
use futures::stream::BoxStream;
use futures::stream::{self, StreamExt};
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

pub async fn embed_path<M: EmbeddingsModel + Send + Sync + 'static>(model: Arc<M>, path: PathBuf) -> BoxStream<'static, Result<FileEmbeddings>> {
	let files = match list_files_recursively(path).await {
		Ok(f) => f,
		Err(e) => return stream::once(futures::future::ready(Err(e))).boxed()
	};

	let file_embeddings_stream = stream::iter(files.into_iter())
		.then(move |path| {
			let model_clone: Arc<M> = Arc::clone(&model);
			async move {
				let file_content = fetch_file_content(path.clone()).await?;
				let embeddings = model_clone.embed(&file_content)?;
				log::info!("Embeddings for {} calculated", path.display());
				Ok(FileEmbeddings {
					path: path.to_str().unwrap().to_string(),
					embeddings
				})
			}
		})
		.boxed();

	file_embeddings_stream
}

pub async fn fetch_file_content(path: PathBuf) -> Result<String> {
	let timeout = Duration::from_secs(60); // Adjust the timeout as needed.
	let result = tokio::time::timeout(timeout, async {
		let file_content = fs::read_to_string(path).await?;
		anyhow::Ok(file_content)
	})
	.await;

	match result {
		Ok(Ok(content)) => Ok(content),
		Ok(Err(err)) => Err(err),
		Err(_) => Err(Error::msg("File content fetching timed out."))
	}
}
