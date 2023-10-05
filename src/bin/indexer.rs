use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use ircc_ai::fs::list_files_recursively;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse();

    let dir = PathBuf::from(args.path);
    list_files_recursively(dir).await?;

    Ok(())
}
