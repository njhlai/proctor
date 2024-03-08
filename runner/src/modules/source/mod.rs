mod leetcode;

use std::error::Error;

use serde::Serialize;
use strum::{Display, EnumIter, EnumString};

use super::lang::Lang;

/// Sources of coding challenge questions.
#[derive(Clone, Debug, Default, Display, EnumIter, EnumString, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[strum(serialize = "leetcode")]
    #[default]
    LeetCode,
}

impl Source {
    /// Returns the result of querying data associated to problem `id` in language `lang`.
    pub fn query(&self, id: &str, lang: &Lang) -> Result<(String, Option<String>, String), Box<dyn Error>> {
        match self {
            Source::LeetCode => leetcode::query(id, lang),
        }
    }
}
