use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Arc;

use clap::Parser;
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
	let file_embeddings = embed_path(Arc::clone(model), dir.to_path_buf()).await?;
	db.insert_embeddings(file_embeddings).await
}
