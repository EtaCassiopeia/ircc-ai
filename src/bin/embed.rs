use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Arc;

use clap::Parser;
use futures::stream::StreamExt;
use ircc_ai::db::qdrant::QdrantDB;
use ircc_ai::db::RepositoryEmbeddingsDB;
use ircc_ai::embeddings::*;
use ircc_ai::fs::embed_path;
use ircc_ai::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	path: String
}

#[cfg(feature = "embed")]
#[tokio::main]
async fn main() {
	pretty_env_logger::init();

	dotenv::dotenv().ok();

	// The model is copied in the container at build time
	let model: Arc<Onnx> = Arc::new(Onnx::new(Path::new("/model")).unwrap());
	let db: QdrantDB = QdrantDB::initialize().unwrap();

	let args = Args::parse();

	let dir = PathBuf::from(args.path);

	log::info!("Calculating embeddings for {}", dir.display());

	let result = embed_and_insert_embeddings(&model, &db, &dir).await;

	match result {
		Ok(_) => {
			log::info!("Process completed successfully.");
			exit(0);
		}
		Err(err) => {
			log::error!("Process failed: {}", err);
			exit(1);
		}
	}
}

async fn embed_and_insert_embeddings(model: &Arc<Onnx>, db: &QdrantDB, dir: &Path) -> Result<()> {
	let collection_exists = db.is_indexed().await?;

	if collection_exists {
		db.delete_collection().await?;
	}

	let embeddings_stream = embed_path(Arc::clone(model), dir.to_path_buf()).await;
	let mut chunks_stream = embeddings_stream.chunks(10);
	let mut successfully_inserted = 0;

	while let Some(chunk) = chunks_stream.next().await {
		// Another alternative could be using chunk.into_iter().collect(); to convert Vec<Result<_>> to Result<Vec<_>>
		// But if there's even a single Err(e) value in the Vec, the collection will yield that error, and any subsequent items
		// will be ignored.

		let mut embeddings_chunk = Vec::new();

		for result in chunk {
			match result {
				Ok(embedding) => {
					embeddings_chunk.push(embedding);
				}
				Err(e) => {
					log::error!("Error processing embedding: {:?}", e);
				}
			}
		}

		if !embeddings_chunk.is_empty() {
			db.insert_embeddings(embeddings_chunk.clone()).await?;
			successfully_inserted += embeddings_chunk.len();
		}
	}

	log::info!("Successfully inserted {} embeddings", successfully_inserted);

	Ok(())
}
