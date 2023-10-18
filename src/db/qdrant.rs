use std::collections::HashMap;

use anyhow::Ok;
use async_trait::async_trait;
use qdrant_client::{
	prelude::*,
	qdrant::{vectors_config::Config, ScrollPoints, VectorParams, VectorsConfig}
};
use rayon::prelude::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use super::RepositoryEmbeddingsDB;
use crate::utils::hash::calculate_hash;
use crate::{
	constants::{EMBEDDINGS_DIMENSION, MAX_FILES_COUNT, QDRANT_COLLECTION_NAME, QDRANT_URL_DEFAULT},
	embeddings::Embeddings,
	fs::FileEmbeddings,
	prelude::*
};

pub struct QdrantDB {
	client: QdrantClient
}

#[async_trait]
impl RepositoryEmbeddingsDB for QdrantDB {
	async fn delete_collection(&self) -> Result<()> {
		if self.client.has_collection(QDRANT_COLLECTION_NAME).await? {
			log::info!("Deleting collection {}", QDRANT_COLLECTION_NAME);
			self.client.delete_collection(QDRANT_COLLECTION_NAME).await?;
		}

		Ok(())
	}

	async fn insert_embeddings(&self, embeddings: Vec<FileEmbeddings>) -> Result<()> {
		let collection_exists = self.client.has_collection(QDRANT_COLLECTION_NAME).await?;

		if !collection_exists {
			let collection_details = CreateCollection {
				collection_name: QDRANT_COLLECTION_NAME.to_string(),
				vectors_config: Some(VectorsConfig {
					config: Some(Config::Params(VectorParams {
						size: EMBEDDINGS_DIMENSION as u64,
						distance: Distance::Cosine.into(),
						..Default::default()
					}))
				}),
				..Default::default()
			};

			self.client.create_collection(&collection_details).await?;
			log::info!("Created collection {}", QDRANT_COLLECTION_NAME);
		}

		let points: Vec<PointStruct> = embeddings
			.into_par_iter()
			.map(|file| {
				let FileEmbeddings { path, embeddings } = file;
				let path_hash = calculate_hash(&path);

				let payload: Payload = HashMap::from([("path", path.into())]).into();

				PointStruct::new(path_hash, embeddings, payload)
			})
			.collect();

		let points_len = points.len();

		self.client.upsert_points(QDRANT_COLLECTION_NAME, points, None).await?;
		log::info!("Upserted {} points", points_len);

		Ok(())
	}

	async fn get_relevant_files(&self, query_embeddings: Embeddings, limit: f32) -> Result<Vec<String>> {
		log::info!("Searching for relevant files");
		let search_response = self
			.client
			.search_points(&SearchPoints {
				collection_name: QDRANT_COLLECTION_NAME.to_string(),
				vector: query_embeddings,
				with_payload: Some(true.into()),
				limit: limit as u64,
				..Default::default()
			})
			.await?;

		let paths: Vec<String> = search_response
			.result
			.into_iter()
			.map(|point| point.payload["path"].to_string().replace('\"', ""))
			.collect();

		Ok(paths)
	}

	async fn get_file_paths(&self) -> Result<Vec<String>> {
		let scroll_reponse = self
			.client
			.scroll(&ScrollPoints {
				collection_name: QDRANT_COLLECTION_NAME.to_string(),
				offset: None,
				filter: None,
				limit: Some(MAX_FILES_COUNT as u32),
				with_payload: Some(true.into()),
				with_vectors: None,
				read_consistency: None
			})
			.await?;

		let file_paths: Vec<String> = scroll_reponse
			.result
			.par_iter()
			.map(|point| point.payload["path"].to_string().replace('\"', ""))
			.collect();

		Ok(file_paths)
	}

	async fn is_indexed(&self) -> Result<bool> {
		self.client.has_collection(QDRANT_COLLECTION_NAME).await
	}
}

impl QdrantDB {
	pub fn initialize() -> Result<QdrantDB> {
		let mut qdrant_url = std::env::var("QDRANT_URL").unwrap_or(String::from(QDRANT_URL_DEFAULT));
		dbg!(&qdrant_url);

		if qdrant_url.is_empty() {
			qdrant_url = QDRANT_URL_DEFAULT.to_string();
		}

		let config = QdrantClientConfig::from_url(&qdrant_url);
		let client = QdrantClient::new(Some(config))?;
		Ok(QdrantDB { client })
	}
}
