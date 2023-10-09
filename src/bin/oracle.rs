use std::{path::Path, sync::Arc};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use ircc_ai::{
	constants::{HOME_ROUTE_REDIRECT_URL, WEBSERVER_PORT_DEFAULT},
	db::qdrant::QdrantDB,
	embeddings::Onnx
};
use tracing_actix_web::TracingLogger;

#[cfg(feature = "oracle")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use log::info;

	dotenv::dotenv().ok();
	let host = "0.0.0.0";

	env_logger::init_from_env(Env::default().default_filter_or("info"));

	let model: Arc<Onnx> = Arc::new(Onnx::new(Path::new("/model")).unwrap());
	let db: Arc<QdrantDB> = Arc::new(QdrantDB::initialize().unwrap());

	let mut port = std::env::var("WEBSERVER_PORT").unwrap_or(WEBSERVER_PORT_DEFAULT.into());
	if port.is_empty() {
		port = WEBSERVER_PORT_DEFAULT.to_string();
	}
	let port = port.parse::<u16>().expect("Invalid WEBSERVER_PORT");

	let server = HttpServer::new(move || {
		App::new()
			.wrap(Cors::permissive())
			.wrap(TracingLogger::default())
			.service(web::redirect("/", HOME_ROUTE_REDIRECT_URL))
			.service(ircc_ai::routes::query)
			.app_data(web::Data::new(model.clone()))
			.app_data(web::Data::new(db.clone()))
	})
	.bind((host, port))?;

	info!("Server running on {}:{}", host, port);

	server.run().await
}
