use std::ops::RangeInclusive;

// Env var defaults
pub const QDRANT_URL_DEFAULT: &str = "http://qdrant:6334";
pub const WEBSERVER_PORT_DEFAULT: &str = "3000";

pub const ORACLE_QUERY_URL_DEFAULT: &str = "http://oracle:3000/query";

// Embeddings
pub const EMBEDDINGS_DIMENSION: usize = 384;

pub const QDRANT_COLLECTION_NAME: &str = "IRCC";

// Actix-web
pub const HOME_ROUTE_REDIRECT_URL: &str = "https://ircc.ai";
pub const SSE_CHANNEL_BUFFER_SIZE: usize = 1;

// Semantic search
pub const MAX_FILES_COUNT: usize = 1000;
pub const FILE_CHUNKER_CAPACITY_RANGE: RangeInclusive<usize> = 300..=400;
pub const RELEVANT_FILES_LIMIT: usize = 3;
pub const RELEVANT_CHUNKS_LIMIT: usize = 2;

// OpenAI
pub const CHAT_COMPLETION_TEMPERATURE: f64 = 0.7;

// See https://platform.openai.com/docs/models/gpt-4 for more info (tested with gpt-3.5-turbo and gpt-4)
pub const CHAT_COMPLETION_MODEL: &str = "gpt-3.5-turbo";
