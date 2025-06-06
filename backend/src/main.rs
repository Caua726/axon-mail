use axum::{
    extract::Query, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use reqwest::Client;

const AI_PROMPT: &str = "hi";
//const AI_PROMPT: &str = "You are an AI that helps users generate optimized Google search queries. Your task is to output exactly 20 refined search queries based on the user's input. Output only a strict JSON array: [\"query1\", \"query2\", ..., \"query20\"]. No extra text, no explanations, no formatting errors, no deviations, no thoughts, no opinions, no mistakes—just the array, nothing else, ever.";

#[derive(Deserialize, Serialize)]
struct AIModel {
    model: String,
}

#[derive(Serialize)]
struct AIResponse {
    response: Vec<String>,
}

async fn ollama_handler() -> &'static str {
    "Hello, Ollama"
}

async fn gemini_handler(Query(params): Query<AIModel>) -> Json<AIResponse> {
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=AIzaSyAnZ-0LO552K3dTnVGOT-RiQpk6tLNr9-s");
    let prompt = AI_PROMPT.to_string() + &params.model;
    let body = json!({"contents": [{"parts":[{"text": prompt}]}]});
    let client = Client::new();

    let response = match client.post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await {
            Ok(resp) => match resp.text().await {
                Ok(text) => text,
                Err(_) => "Error reading response".to_string(),
            },
            Err(_) => "Error making request".to_string(),
    };

    let parsed_response: Value = match serde_json::from_str(&response) {
        Ok(value) => value,
        Err(_) => {
            let ai_response = AIResponse { response: vec!["Error parsing response".to_string()] };
            return Json(ai_response);
        }
    };
    let clean_response = parsed_response["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("").to_string();
    let ai_response = AIResponse { response: vec![clean_response.clone()] };
    Json(ai_response)
}   

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .route("/api/ollama", get(ollama_handler))
        .route("/api/gemini", post(gemini_handler));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    
    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();

}