use axum::{
    routing::{get, post},
    Router,
};
use chrono::Local;
use colored::*;
use std::env;
mod api;
use api::llm::llm;

#[tokio::main]
async fn main() {
    dotenvy::from_path("../.env").ok();

    let server_ip = std::env::var("BACKEND_IP").expect("BACKEND_IP não definida");
    let server_port = std::env::var("BACKEND_PORT").expect("BACKEND_PORT não definida");
    let server_url = format!("{}:{}", server_ip, server_port);
    println!(
        "{}\n{} {} {}\n{}\n{} {}\n{} {}\n{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue().bold(),
        "🚀".bright_yellow(),
        "Axon-Mail Backend Running".bright_yellow().bold(),
        "🚀".bright_yellow(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue().bold(),
        "🌐 Server running at:".bright_cyan(),
        server_url.bright_green().bold(),
        "📅 Started at:  ".bright_cyan(),
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string().bright_magenta(),
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_blue().bold()
    );

    let app = routes();
    axum::serve(tokio::net::TcpListener::bind(server_url).await.unwrap(), app).await.unwrap();
}

fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .route("/api/llm", post(llm))
}