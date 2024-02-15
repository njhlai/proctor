use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;

use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

pub struct Query<V, T> {
    url: String,
    pub query: &'static str,
    pub variable: V,
    response_type: PhantomData<T>,
}

impl<V, T> Query<V, T> {
    pub fn from(url: String, query: &'static str, variable: V) -> Self {
        Query { url, query, variable, response_type: PhantomData }
    }
}

pub trait Constructible {
    fn json(&self) -> HashMap<&str, String>;
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
