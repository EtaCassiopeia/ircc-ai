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
