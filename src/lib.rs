use serde::Deserialize;
use chrono::serde::ts_seconds;
use sqlx::{Postgres, Pool, types::chrono::{DateTime, Utc}, FromRow};
use url::Url;

pub mod app_config;
pub mod common;
pub mod handlers;
pub mod dao;

pub const GROUP_CHAT_PRICE: &str = "model_price";
pub const CHAT_INPUT_SUFFIX: &str = "_input";
pub const CHAT_OUTPUT_SUFFIX: &str = "_output";
pub const DEFAULT_CHATGPT_URL: &str = "https://api.openai.com/v1/chat/completions";

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

#[derive(Debug, FromRow)]
pub struct SysUserToken {
    pub id: i32,
    pub name: String,
    pub local_token: String,
    pub gpt_token: String,
}

#[derive(Debug, serde::Serialize, FromRow)]
pub struct SysModelRequest {
    pub remote_id: String,
    pub model: String,
    pub total_token: i32,
    pub price: f32,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>
}