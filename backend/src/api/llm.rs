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
    match payload.provider.to_lowercase().as_str() {
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

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, Method, StatusCode},
    };
    use tower::ServiceExt;
    use crate::routes;

    #[tokio::test]
    async fn test_llm_case_insensitive() {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/llm")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"provider": "Ollama"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}