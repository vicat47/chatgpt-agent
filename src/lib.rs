use serde::Deserialize;
use sqlx::{Postgres, Pool};
use url::Url;

pub mod app_config;
pub mod common;
pub mod handlers;
pub mod dao;

pub const GROUP_CHAT_PRICE: &str = "model_price";
pub const CHAT_INPUT_SUFFIX: &str = "_input";
pub const CHAT_OUTPUT_SUFFIX: &str = "_output";

pub struct MyData {
    pub pool: Pool<Postgres>,
    pub client: reqwest::Client,
    pub target_url: Url,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiChat {
    pub id: String,
    pub model: String,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: i32,
    pub prompt_tokens: i32,
    pub total_tokens: i32,
}