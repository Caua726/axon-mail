//! # Axon-Mail Backend
//!
//! This is the main entry point for the Axon-Mail backend server.
//! The server is built using the Axum framework and provides a RESTful API
//! for the frontend to interact with.
//!
//! ## Features
//!
//! - **API Endpoints**: Exposes API endpoints for various functionalities,
//!   such as interacting with Large Language Models (LLMs).
//! - **Environment Configuration**: Loads environment variables from a `.env` file
//!   to configure the server's IP address and port.
//! - **Logging**: Prints a colorful startup message to the console with server details.
//!
//! ## Modules
//!
//! - `api`: Contains the API handlers and related logic.

use axum::{
    routing::{get, post},
    Router,
};
use chrono::Local;
use colored::*;
use std::env;
mod api;
use api::llm::llm;

/// The main entry point for the backend server.
///
/// This function initializes the server by:
/// 1. Loading environment variables from a `.env` file.
/// 2. Constructing the server URL from the `BACKEND_IP` and `BACKEND_PORT` variables.
/// 3. Printing a startup message to the console.
/// 4. Setting up the application routes.
/// 5. Binding the server to the specified URL and serving requests.
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

/// Defines the application's routes.
///
/// This function creates a new Axum `Router` and defines the following routes:
/// - `GET /`: A simple health check endpoint that returns "Hello, World".
/// - `POST /api/llm`: The main API endpoint for interacting with LLMs, handled by the `llm` function.
///
/// # Returns
///
/// * `Router` - The configured Axum router.
fn routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .route("/api/llm", post(llm))
}