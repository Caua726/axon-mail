use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LlmPayload {
    provider: String,
}

pub async fn llm(Json(payload): Json<LlmPayload>) -> Response {
    match payload.provider.as_str() {
        "ollama" => ollama().await.into_response(),
        "gemini" => gemini().await.into_response(),
        "gpt" => gpt().await.into_response(),
        _ => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid provider" })),
        )
            .into_response(),
    }
}

async fn ollama() -> impl IntoResponse {
    Json(json!({ "provider": "ollama", "response": "Response from Ollama" }))
}

async fn gemini() -> impl IntoResponse {
    Json(json!({ "provider": "gemini", "response": "Response from Gemini" }))
}

async fn gpt() -> impl IntoResponse {
    Json(json!({ "provider": "gpt", "response": "Response from GPT" }))
}

async fn nebius() -> impl IntoResponse {
    Json(json!({ "provider": "nebius", "response": "Response from Nebius" }))
}

async fn deepseek() -> impl IntoResponse {
    Json(json!({ "provider": "deepseek", "response": "Response from DeepSeek" }))
}

async fn custom(payload: Json<LlmPayload>) -> impl IntoResponse {
    Json(json!({ "provider": "custom", "response": "Response from Custom" }))
}