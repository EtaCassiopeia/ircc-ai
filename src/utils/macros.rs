// Custom implementation for SSE Events based on https://crates.io/crates/enum_str
/// Example usage
// sse_events! {
//    QueryEvent,
//    (ProcessQuery, "PROCESS_QUERY"),
//    (SearchCodebase, "SEARCH_CODEBASE"),
//    (SearchFile, "SEARCH_FILE"),
//    (SearchPath, "SEARCH_PATH"),
//    (GenerateResponse, "GENERATE_RESPONSE"),
//    (Done, "DONE"),
//    (Error, "ERROR"),
// }
// tx.send(EmbedEvent::QueryEvent(Some(json!({
//         "files": files.len(),
//          }))).into())
///
#[macro_export]
macro_rules!  sse_events {
    ($name:ident, $(($key:ident, $value:expr),)*) => {
       #[derive(Debug, PartialEq)]
       pub enum $name
        {
            $($key(Option<serde_json::Value>)),*
        }

        impl From<$name> for Data {
            fn from(event: $name) -> Data {
                match event {
                    $(
                        $name::$key(data) => Data::new(data.unwrap_or_default().to_string()).event($value)
                    ),*
                }
            }
        }

        impl $name {

        }

    }
}

/// Example usage
// functions_enum!{
// Function,
// (SearchDocuments, "search_documents"),
// (SearchFile, "search_file"),
// (SearchPath, "search_path"),
// (Done, "done"),
// }
// Function::from_str("search_documents").unwrap();
// Function::SearchDocuments.to_string();
///
#[macro_export]
macro_rules! functions_enum {
    ($name:ident, $(($key:ident, $value:expr),)*) => {
       #[derive(Debug, PartialEq, Clone)]
       pub enum $name
        {
            $($key),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $(
                        &$name::$key => $value.to_string()
                    ),*
                }
            }
        }

        impl FromStr for $name {
            type Err = anyhow::Error;

            fn from_str(val: &str) -> Result<Self> {
                match val
                 {
                    $(
                        $value => Ok($name::$key)
                    ),*,
                    _ => Err(anyhow::anyhow!("Invalid function"))
                }
            }
        }
    }
}
