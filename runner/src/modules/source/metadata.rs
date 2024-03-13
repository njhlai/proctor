use serde::{Deserialize, Deserializer};
use serde_json::Result as jsonResult;

use crate::modules::lang::Lang;

/// A structure defining the metadata associated to a question.
#[derive(Debug, Deserialize)]
pub struct MetaData {
    pub name: String,
    pub params: Vec<Variable>,
    #[serde(rename = "return", deserialize_with = "flatten_return")]
    pub return_type: String,
}

/// A structure defining the name and type for a variable.
#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    #[serde(rename = "type")]
    pub typ: String,
}

impl MetaData {
    /// Returns the [`MetaData`] from the JSON string `metadata_json` if possible.
    pub fn from(metadata_json: &str, _: &Lang) -> jsonResult<Self> {
        serde_json::from_str(metadata_json)
    }
}

/// Returns the underlying object from unwrapping a nested deserialised structure.
fn flatten_return<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(rename = "type")]
        typ: String,
    }

    Ok(Wrapper::deserialize(deserializer)?.typ)
}
