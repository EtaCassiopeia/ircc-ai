pub mod events;

use std::sync::Arc;

use actix_web::{
	error::ErrorNotFound,
	post,
	web::{self, Json},
	Responder, Result
};
use actix_web_lab::sse;

use crate::constants::SSE_CHANNEL_BUFFER_SIZE;
use crate::convrsation::data::Query;
use crate::convrsation::Conversation;
use crate::db::qdrant::QdrantDB;
use crate::db::RepositoryEmbeddingsDB;
use crate::embeddings::Onnx;

#[post("/query")]
async fn query(data: Json<Query>, db: web::Data<Arc<QdrantDB>>, model: web::Data<Arc<Onnx>>) -> Result<impl Responder> {
	if db.is_indexed().await.unwrap_or_default() {
		let (sender, rx) = sse::channel(SSE_CHANNEL_BUFFER_SIZE);

		actix_rt::spawn(async move {
			let result = async {
				let mut conversation = Conversation::initiate(data.into_inner(), db.get_ref().clone(), model.get_ref().clone(), sender.clone()).await?;
				conversation.generate().await?;

				Ok::<(), anyhow::Error>(())
			};
			if let Err(e) = result.await {
				eprintln!("/query error: {}", e);
			}
		});

		Ok(rx)
	} else {
		eprintln!("Repository is not indexed");
		Err(ErrorNotFound("Repository is not indexed"))
	}
}
