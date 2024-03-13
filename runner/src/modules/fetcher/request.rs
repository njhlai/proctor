use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

use colored::Colorize;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

/// HTTP request methods.
#[allow(clippy::upper_case_acronyms, dead_code)]
pub enum Method {
    GET,
    POST,
}

/// A struct which represents an API query request.
pub struct Request<V, T> {
    url: String,
    method: Method,
    pub query: &'static str,
    pub variable: V,
    response_type: PhantomData<T>,
}

impl<V: Display, T> Request<V, T> {
    /// Returns the [`Request`] formed from the given parameters.
    pub fn from(url: String, method: Method, query: &'static str, variable: V) -> Self {
        Request { url, method, query, variable, response_type: PhantomData }
    }
}

/// A trait to allow construction of request body.
trait Constructible {
    /// Returns a JSON request body.
    fn json(&self) -> HashMap<&str, String>;
}

impl<V: Display, T> Constructible for Request<V, T> {
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

impl<V: Display, T: DeserializeOwned> Response<T> for Request<V, T>
where
    Self: Constructible,
{
    fn response(&self, client: &Client) -> Result<T, Box<dyn Error>> {
        match match self.method {
            Method::GET => client.get(self.url.to_string()),
            Method::POST => client.post(self.url.to_string()),
        }
        .header("User-Agent", "rust")
        .json(&self.json())
        .send()
        {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response.json::<T>()?),
                s => Err(format!("Request failed with code {}:\n{response:#?}", s.as_str().yellow().bold()).into()),
            },
            Err(e) => Err(Box::new(e)),
        }
    }
}

/// A wrapper struct for a response from an execution of [`Request`] of type GraphQL.
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T: for<'a> Deserialize<'a>> {
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
