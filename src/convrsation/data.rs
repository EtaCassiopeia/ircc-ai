use std::str::FromStr;

use openai_api_rs::v1::chat_completion::FunctionCall;
use serde::Deserialize;

use crate::prelude::*;
use crate::utils::functions::Function;

#[derive(Debug, Deserialize)]
pub struct Query {
	pub query: String
}

impl ToString for Query {
	fn to_string(&self) -> String {
		self.query.clone()
	}
}

#[derive(Debug)]
pub struct RelevantChunk {
	pub path: String,
	pub content: String
}

impl ToString for RelevantChunk {
	fn to_string(&self) -> String {
		format!("##Relevant file chunk##\nPath argument:{}\nRelevant content: {}", self.path, self.content.trim())
	}
}

#[derive(Debug, Clone)]
pub struct ParsedFunctionCall {
	pub name: Function,
	pub args: serde_json::Value
}

impl TryFrom<&FunctionCall> for ParsedFunctionCall {
	type Error = anyhow::Error;

	fn try_from(func: &FunctionCall) -> Result<Self> {
		let func = func.clone();
		let name = Function::from_str(&func.name.unwrap_or("done".into()))?;
		let args = func.arguments.unwrap_or("{}".to_string());
		let args = serde_json::from_str::<serde_json::Value>(&args)?;
		Ok(ParsedFunctionCall { name, args })
	}
}
