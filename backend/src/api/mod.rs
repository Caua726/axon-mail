//! # API Module
//!
//! This module defines the API endpoints for the backend service.
//! It is responsible for handling incoming HTTP requests and routing
//! them to the appropriate handlers.
//!
//! Currently, it includes the following sub-modules:
//! - `llm`: Handles requests related to Large Language Models (LLMs).
//! - `config`: Manages application configuration.

/// The `llm` module, which contains handlers for LLM-related API endpoints.
pub mod llm;

/// The `config` module, which is responsible for application configuration.
pub mod config;