use std::path::PathBuf;
use std::sync::Arc;

use async_recursion::async_recursion;
use log::{debug, info};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::{fs, task};

use crate::{
	embeddings::{Embeddings, EmbeddingsModel},
	prelude::*
};

#[derive(Debug, Clone)]
pub struct FileEmbeddings {
	pub path: String,
	pub embeddings: Embeddings
}

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

pub async fn embed_path<M: EmbeddingsModel + Send + Sync + 'static>(model: Arc<M>, path: PathBuf) -> Result<Vec<FileEmbeddings>> {
	let files = list_files_recursively(path).await?;

	let embedding_futures: Vec<_> = files
		.into_iter()
		.map(|path| {
			let model_clone = Arc::clone(&model);
			task::spawn(async move {
				let file_content = fs::read_to_string(path.clone()).await.unwrap();
				let embeddings = model_clone.embed(&file_content).unwrap();
				FileEmbeddings {
					path: path.to_str().unwrap().to_string(),
					embeddings
				}
			})
		})
		.collect();

	let file_embeddings: Vec<_> = futures::future::join_all(embedding_futures).await.into_iter()
        .map(|res| res.unwrap())  // unwrap the Result returned by task::spawn
        .collect();

	Ok(file_embeddings)
}

pub async fn fetch_file_content(path: &str) -> Result<String> {
	let mut file = File::open(path).await?;

	let mut buffer = String::new();
	file.read_to_string(&mut buffer).await?;
	Ok(buffer)
}
