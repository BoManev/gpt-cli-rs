use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokes: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub text: Option<String>,
    pub index: u32,
    pub logprobs: Option<u32>,
    pub finish_reason: Option<String>,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionBody {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: u64,
    pub model: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}
