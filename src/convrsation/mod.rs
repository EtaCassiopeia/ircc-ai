#![allow(unused_must_use)]
pub mod data;
mod prompts;

use std::env;
use std::sync::Arc;

use actix_web_lab::sse::Sender;
use openai_api_rs::v1::chat_completion::FinishReason;
use openai_api_rs::v1::{
	api::Client,
	chat_completion::{ChatCompletionMessage, ChatCompletionRequest, ChatCompletionResponse, MessageRole}
};
use prompts::{generate_completion_request, system_message};

use self::prompts::{answer_generation_prompt, sanitize_query_prompt};
use crate::constants::RELEVANT_CHUNKS_LIMIT;
pub use crate::convrsation::data::*;
use crate::prelude::*;
use crate::routes::events::{emit, QueryEvent};
use crate::utils::functions::{paths_to_completion_message, relevant_chunks_to_completion_message, search_documents, search_file, search_path, Function};
use crate::{db::RepositoryEmbeddingsDB, embeddings::EmbeddingsModel};

pub struct Conversation<D: RepositoryEmbeddingsDB, M: EmbeddingsModel> {
	query: data::Query,
	client: Client,
	messages: Vec<ChatCompletionMessage>,
	db: Arc<D>,
	model: Arc<M>,
	sender: Sender
}

impl<D: RepositoryEmbeddingsDB, M: EmbeddingsModel> Conversation<D, M> {
	pub async fn initiate(mut query: data::Query, db: Arc<D>, model: Arc<M>, sender: Sender) -> Result<Self> {
		dbg!("Initiating conversation with query: {}", &query.query);
		emit(&sender, QueryEvent::ProcessQuery(None)).await;

		query.query = sanitize_query(&query.query)?;
		let client = Client::new(env::var("OPENAI_API_KEY").unwrap());
		let messages = vec![
			ChatCompletionMessage {
				name: None,
				function_call: None,
				role: MessageRole::system,
				content: system_message()
			},
			ChatCompletionMessage {
				name: None,
				function_call: None,
				role: MessageRole::user,
				content: query.to_string()
			},
		];
		dbg!("Initiated conversation with sanitized query: {}\n\n Messages: {}", &query.query, &messages);
		Ok(Self {
			query,
			client,
			messages,
			db,
			model,
			sender
		})
	}

	fn append_message(&mut self, message: ChatCompletionMessage) {
		self.messages.push(message);
	}

	fn prepare_final_explanation_message(&mut self) {
		// Update the system prompt using answer_generation_prompt()
		self.messages[0] = ChatCompletionMessage {
			name: None,
			function_call: None,
			role: MessageRole::system,
			content: answer_generation_prompt()
		}
	}

	fn send_request(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse> {
		dbg!("Sending request to OpenAI API: \n{}", &request);
		Ok(self.client.chat_completion(request)?)
	}

	pub async fn generate(&mut self) -> Result<()> {
		#[allow(unused_labels)]
		'conversation: loop {
			// Generate a request with the message history and functions
			let request = generate_completion_request(self.messages.clone(), "auto");

			match self.send_request(request) {
				Ok(response) => {
					dbg!("Response: {}", &response);
					match response.choices[0].finish_reason {
						FinishReason::function_call => {
							dbg!("Finish reason: Function call");
							if let Some(function_call) = response.choices[0].message.function_call.clone() {
								let parsed_function_call = ParsedFunctionCall::try_from(&function_call)?;
								let function_call_message = ChatCompletionMessage {
									name: None,
									function_call: Some(function_call),
									role: MessageRole::assistant,
									content: String::new()
								};
								self.append_message(function_call_message);
								dbg!(parsed_function_call.clone());
								match parsed_function_call.name {
									Function::SearchDocuments => {
										let query: &str = parsed_function_call.args["query"].as_str().unwrap_or_default();
										dbg!("SearchDocuments with params: {}", query);

										emit(&self.sender, QueryEvent::SearchDocuments(Some(parsed_function_call.clone().args))).await;

										let relevant_chunks = search_documents(
											query,
											self.model.as_ref(),
											self.db.as_ref(),
											crate::constants::RELEVANT_FILES_LIMIT,
											RELEVANT_CHUNKS_LIMIT
										)
										.await?;
										let completion_message = relevant_chunks_to_completion_message(parsed_function_call.name, relevant_chunks);
										dbg!("Completion message: {}", &completion_message);
										self.append_message(completion_message);
									}
									Function::SearchFile => {
										let query: &str = parsed_function_call.args["query"].as_str().unwrap_or_default();
										let path: &str = parsed_function_call.args["path"].as_str().unwrap_or_default();

										dbg!("SearchFile at {} with params: {}", path, query);

										emit(&self.sender, QueryEvent::SearchFile(Some(parsed_function_call.clone().args))).await;

										let relevant_chunks = search_file(path, query, self.model.as_ref(), RELEVANT_CHUNKS_LIMIT).await?;
										let completion_message = relevant_chunks_to_completion_message(parsed_function_call.name, relevant_chunks);
										dbg!("Completion message: {}", &completion_message);
										self.append_message(completion_message);
									}
									Function::SearchPath => {
										let path: &str = parsed_function_call.args["path"].as_str().unwrap_or_default();
										dbg!("SearchPath with params: {}", path);

										emit(&self.sender, QueryEvent::SearchPath(Some(parsed_function_call.clone().args))).await;

										let fuzzy_matched_paths = search_path(path, self.db.as_ref(), 1).await?;
										let completion_message = paths_to_completion_message(parsed_function_call.name, fuzzy_matched_paths);
										dbg!("Completion message: {}", &completion_message);
										self.append_message(completion_message);
									}
									Function::Done => {
										dbg!("Generating final response");
										self.prepare_final_explanation_message();

										// Generate a request with the message history and no functions
										let request = generate_completion_request(self.messages.clone(), "none");

										emit(&self.sender, QueryEvent::GenerateResponse(None)).await;

										let response = match self.send_request(request) {
											Ok(response) => response,
											Err(e) => {
												dbg!(e.to_string());
												return Err(e);
											}
										};
										dbg!("Response: {}", &response);
										let response = response.choices[0].message.content.clone().unwrap_or_default();

										emit(&self.sender, QueryEvent::Done(Some(response.into()))).await;

										return Ok(());
									}
								}
							};
						}

						FinishReason::stop => {
							dbg!("Finish reason: Stop");
							// As of yet, there isn't a robust way to instruct the model to respond with function calls only except for switching to
							// GPT-4 We can only suggest it do so in the system message
							// prompts.rs#L127
							// A warning from OpenAI's official documentation:
							// "gpt-3.5-turbo-0301 does not always pay strong attention to system messages. Future models will be trained to pay
							// strong attention to system messages." "If you are using GPT-3.5-turbo, you can already utilize the system role input;
							// however, be aware that it will not pay strong attention to it. On the other hand, if you have access to the GPT-4
							// preview, you can take full advantage of this powerful feature."

							let response = response.choices[0].message.content.clone().unwrap_or_default();
							dbg!("Response: {}", &response);
							emit(&self.sender, QueryEvent::Done(Some(response.into()))).await;

							return Ok(());
						}

						_ => {
							dbg!("Model returned an unexpected response.");
							return Err(anyhow::anyhow!("Model returned an unexpected response."));
						}
					}
				}
				Err(e) => {
					dbg!("Error: {}", e.to_string());
					return Err(e);
				}
			};
		}
	}
}

fn sanitize_query(query: &str) -> Result<String> {
	let message = ChatCompletionMessage {
		name: None,
		function_call: None,
		role: MessageRole::user,
		content: sanitize_query_prompt(query)
	};
	let client = Client::new(env::var("OPENAI_API_KEY")?);
	let request = generate_completion_request(vec![message], "none");
	let response = client.chat_completion(request)?;
	if let FinishReason::stop = response.choices[0].finish_reason {
		let sanitized_query = response.choices[0].message.content.clone().unwrap_or_default();
		if sanitized_query.is_empty() {
			Err(anyhow::anyhow!("No query found"))
		} else {
			Ok(sanitized_query)
		}
	} else {
		Err(anyhow::anyhow!("Query sanitization failed"))
	}
}
