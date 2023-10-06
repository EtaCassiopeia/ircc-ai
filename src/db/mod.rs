use async_trait::async_trait;

use crate::embeddings::Embeddings;
use crate::fs::FileEmbeddings;
use crate::prelude::*;

#[async_trait]
pub trait RepositoryEmbeddingsDB {
	async fn inserts_embeddings(&self, embeddings: Vec<FileEmbeddings>) -> Result<()>;
	async fn get_relevant_files(&self, query_embeddings: Embeddings, threshold: f32) -> Result<Vec<String>>;
	async fn is_indexed(&self) -> Result<bool>;
}
