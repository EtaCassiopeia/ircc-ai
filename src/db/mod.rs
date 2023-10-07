use async_trait::async_trait;

use crate::embeddings::Embeddings;
use crate::fs::FileEmbeddings;
use crate::prelude::*;

pub mod qdrant;

#[async_trait]
pub trait RepositoryEmbeddingsDB {
	async fn insert_embeddings(&self, embeddings: Vec<FileEmbeddings>) -> Result<()>;
	async fn get_relevant_files(&self, query_embeddings: Embeddings, limit: f32) -> Result<Vec<String>>;
	async fn get_file_paths(&self) -> Result<Vec<String>>;
	async fn is_indexed(&self) -> Result<bool>;
}
