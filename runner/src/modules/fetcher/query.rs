use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

/// An empty struct with a [`Display`] trait.
pub struct Empty;

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

/// A struct which represents an API query request.
pub struct Query<V, T> {
    url: String,
    pub query: &'static str,
    pub variable: V,
    response_type: PhantomData<T>,
}

impl<V: Display, T> Query<V, T> {
    /// Returns the [`Query`] formed from the given parameters.
    pub fn from(url: String, query: &'static str, variable: V) -> Self {
        Query { url, query, variable, response_type: PhantomData }
    }
}

/// A trait to allow construction of request body.
trait Constructible {
    /// Returns a JSON request body.
    fn json(&self) -> HashMap<&str, String>;
}

impl<V: Display, T> Constructible for Query<V, T> {
    fn json(&self) -> HashMap<&str, String> {
        let mut json = HashMap::new();

        if !self.query.is_empty() {
            json.insert("query", String::from(self.query));

            let variable = self.variable.to_string();
            if !variable.is_empty() {
                json.insert("variables", variable);
            }
        }

        json
    }
}

/// A trait to extract a response from a request to a [`Client`].
pub trait Response<T: Sized> {
    /// Returns the result of a request to `client`.
    fn response(&self, client: &Client) -> Result<T, Box<dyn Error>>;
}

impl<V, T: DeserializeOwned> Response<T> for Query<V, T>
where
    Self: Constructible,
{
    fn response(&self, client: &Client) -> Result<T, Box<dyn Error>> {
        Ok(client
            .post(self.url.to_string())
            .json(&self.json())
            .send()?
            .json::<T>()?)
    }
}

/// A struct for a response from an execution of [`Query`].
#[derive(Debug, Deserialize)]
pub struct QueryResponse<T: for<'a> Deserialize<'a>> {
    #[serde(deserialize_with = "denest")]
    pub data: T,
}

/// Returns the underlying object from unwrapping a nested deserialised structure.
fn denest<'de, D: Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<T, D::Error> {
    #[derive(Deserialize)]
    struct Wrapper<T> {
        obj: T,
    }

    Ok(Wrapper::deserialize(deserializer)?.obj)
}
