use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn calculate_hash(input: &str) -> u64 {
	let mut hasher = DefaultHasher::new();
	input.hash(&mut hasher);
	hasher.finish()
}
