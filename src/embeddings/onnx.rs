use std::{path::Path, sync::Arc, thread::available_parallelism};

use ndarray::{Array, Axis, CowArray};
use ort::{execution_providers::CPUExecutionProviderOptions, Environment, ExecutionProvider, GraphOptimizationLevel, SessionBuilder, Value};

use super::{Embeddings, EmbeddingsModel};
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Onnx {
	tokenizer: Arc<tokenizers::Tokenizer>,
	session: Arc<ort::Session>
}

impl Onnx {
	pub fn new<P: AsRef<Path>>(model_dir: P) -> Result<Self> {
		let environment = Arc::new(
			Environment::builder()
				.with_name("Embeddings")
				.with_execution_providers([ExecutionProvider::CPU(CPUExecutionProviderOptions::default())])
				.build()?
		);

		let threads = available_parallelism().unwrap().get() as i16;

		Ok(Self {
			tokenizer: tokenizers::Tokenizer::from_file(model_dir.as_ref().join("tokenizer.json"))
				.unwrap()
				.into(),
			session: SessionBuilder::new(&environment)?
                .with_optimization_level(GraphOptimizationLevel::Level3)?
                .with_intra_threads(threads)?
                // https://huggingface.co/rawsh/multi-qa-MiniLM-distill-onnx-L6-cos-v1
                .with_model_from_file(model_dir.as_ref().join("model_quantized.onnx"))?
                .into()
		})
	}
}

impl EmbeddingsModel for Onnx {
	/// The primary purpose of this function appears to be to convert a text sequence into a vector representation
	/// (embedding) that can be used to find the documents siliar to the query
	fn embed(&self, sequence: &str) -> Result<Embeddings> {
		let tokenizer_output = self.tokenizer.encode(sequence, true).unwrap();

		// The IDs are the main input to a Language Model. They are the token indices, the numerical representations that a LM
		// understands.
		let input_ids = tokenizer_output.get_ids();
		// This indicates to the LM which tokens should be attended to, and which should not. This is especially important when
		// batching sequences, where we need to applying padding.
		let attention_mask = tokenizer_output.get_attention_mask();
		// Generally used for tasks like sequence classification or question answering, these tokens let the LM know which input
		// sequence corresponds to each tokens.
		let token_type_ids = tokenizer_output.get_type_ids();
		let length = input_ids.len();

		let inputs_ids_array = CowArray::from(Array::from_shape_vec((1, length), input_ids.iter().map(|&x| x as i64).collect())?).into_dyn();

		let attention_mask_array = CowArray::from(Array::from_shape_vec((1, length), attention_mask.iter().map(|&x| x as i64).collect())?).into_dyn();

		let token_type_ids_array = CowArray::from(Array::from_shape_vec((1, length), token_type_ids.iter().map(|&x| x as i64).collect())?).into_dyn();

		let outputs = self.session.run(vec![
			Value::from_array(self.session.allocator(), &inputs_ids_array)?,
			Value::from_array(self.session.allocator(), &attention_mask_array)?,
			Value::from_array(self.session.allocator(), &token_type_ids_array)?,
		])?;

		let output_tensor = outputs[0].try_extract().unwrap();
		let sequence_embedding = &*output_tensor.view();
		let pooled = sequence_embedding.mean_axis(Axis(1)).unwrap();
		Ok(pooled.to_owned().as_slice().unwrap().to_vec())
	}
}
