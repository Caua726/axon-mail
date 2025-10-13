use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::json;

/// Represents the payload for the LLM API endpoint.
///
/// This struct is used to deserialize the JSON payload from the client,
/// which should contain the name of the desired LLM provider.
#[derive(Deserialize)]
pub struct LlmPayload {
    /// The name of the LLM provider (e.g., "ollama", "gemini", "gpt").
    provider: String,
}

/// The main handler for the `/api/llm` endpoint.
///
/// This function receives a JSON payload with an LLM provider specified
/// and routes the request to the corresponding provider function.
/// If the provider is not supported, it returns a `400 Bad Request` error.
///
/// # Arguments
///
/// * `payload` - A `Json<LlmPayload>` containing the deserialized request body.
///
/// # Returns
///
/// * `Response` - An Axum response, which is either a JSON response from
///   one of the LLM provider functions or a JSON error object.
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

/// Simulates a response from the Ollama LLM provider.
///
/// # Returns
///
/// * `impl IntoResponse` - A JSON response indicating that the provider is Ollama.
async fn ollama() -> impl IntoResponse {
    Json(json!({ "provider": "ollama", "response": "Response from Ollama" }))
}

/// Simulates a response from the Gemini LLM provider.
///
/// # Returns
///
/// * `impl IntoResponse` - A JSON response indicating that the provider is Gemini.
async fn gemini() -> impl IntoResponse {
    Json(json!({ "provider": "gemini", "response": "Response from Gemini" }))
}

/// Simulates a response from the GPT LLM provider.
///
/// # Returns
///
/// * `impl IntoResponse` - A JSON response indicating that the provider is GPT.
async fn gpt() -> impl IntoResponse {
    Json(json!({ "provider": "gpt", "response": "Response from GPT" }))
}

/// Simulates a response from the Nebius LLM provider.
///
/// This function is currently not used by the `llm` handler.
///
/// # Returns
///
/// * `impl IntoResponse` - A JSON response indicating that the provider is Nebius.
async fn nebius() -> impl IntoResponse {
    Json(json!({ "provider": "nebius", "response": "Response from Nebius" }))
}

/// Simulates a response from the DeepSeek LLM provider.
///
/// This function is currently not used by the `llm` handler.
///
/// # Returns
///
/// * `impl IntoResponse` - A JSON response indicating that the provider is DeepSeek.
async fn deepseek() -> impl IntoResponse {
    Json(json!({ "provider": "deepseek", "response": "Response from DeepSeek" }))
}

/// Simulates a response from a custom LLM provider.
///
/// This function is currently not used by the `llm` handler.
///
/// # Arguments
///
/// * `payload` - A `Json<LlmPayload>` containing the deserialized request body.
///
/// # Returns
///
/// * `impl IntoResponse` - A JSON response indicating that the provider is Custom.
async fn custom(payload: Json<LlmPayload>) -> impl IntoResponse {
    Json(json!({ "provider": "custom", "response": "Response from Custom" }))
}