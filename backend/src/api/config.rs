async fn get_config() -> impl IntoResponse {
    Json(json!({ "config": "config" }))
}