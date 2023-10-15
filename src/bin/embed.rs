use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use ircc_ai::db::qdrant::QdrantDB;
use ircc_ai::db::RepositoryEmbeddingsDB;
use ircc_ai::embeddings::*;
use ircc_ai::fs::embed_path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	path: String
}

#[cfg(feature = "embed")]
#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv().ok();

	env_logger::init_from_env(Env::default().default_filter_or("info"));

	// The model is copied in the container at build time
	let model: Arc<Onnx> = Arc::new(Onnx::new(Path::new("/model")).unwrap());
	let db: QdrantDB = QdrantDB::initialize().unwrap();

	let args = Args::parse();

	let dir = PathBuf::from(args.path);

	let file_embeddings = embed_path(model, dir).await?;

	db.insert_embeddings(file_embeddings).await?;

	Ok(())
}
