use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

pub struct Empty;

impl std::fmt::Display for Empty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub struct Query<V, T> {
    url: String,
    pub query: &'static str,
    pub variable: V,
    response_type: PhantomData<T>,
}

impl<V: Display, T> Query<V, T> {
    pub fn from(url: String, query: &'static str, variable: V) -> Self {
        Query { url, query, variable, response_type: PhantomData }
    }
}

trait Constructible {
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

pub trait Response<T: Sized> {
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

#[derive(Debug, Deserialize)]
pub struct QueryResponse<T: for<'a> Deserialize<'a>> {
    #[serde(deserialize_with = "denest")]
    pub data: T,
}

fn denest<'de, D: Deserializer<'de>, T: Deserialize<'de>>(deserializer: D) -> Result<T, D::Error> {
    #[derive(Deserialize)]
    struct Wrapper<T> {
        obj: T,
    }

    Ok(Wrapper::deserialize(deserializer)?.obj)
}
