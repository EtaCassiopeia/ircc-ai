use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use ircc_ai::fs::list_files_recursively;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	path: String
}

#[tokio::main]
async fn main() -> Result<()> {
	dotenv::dotenv().ok();

	env_logger::init_from_env(Env::default().default_filter_or("info"));

	let args = Args::parse();

	let dir = PathBuf::from(args.path);
	let file = list_files_recursively(dir).await?;

	for f in file {
		println!("{}", f.display());
	}

	Ok(())
}
