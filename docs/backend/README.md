# Backend Documentation

This document provides a high-level overview of the backend architecture of the Axon-Mail application.

## Overview

The backend is a Rust application built with the Axum framework. Its primary responsibility is to serve a RESTful API that the frontend can consume. It handles business logic, data processing, and interactions with external services, such as Large Language Models (LLMs).

## Key Technologies

- **Rust**: The programming language used for the backend, chosen for its performance, safety, and concurrency features.
- **Axum**: A web application framework that's ergonomic and modular, built on top of Tokio, Tower, and Hyper.
- **Tokio**: An asynchronous runtime for Rust, used to power the web server.
- **Serde**: A framework for serializing and deserializing Rust data structures efficiently.
- **Dotenvy**: Used for loading environment variables from a `.env` file.

## Project Structure

The backend code is organized into the following main parts:

- `src/main.rs`: The main entry point of the application. It's responsible for initializing the server, setting up routes, and loading configurations.
- `src/api/`: This module contains all the API-related logic, including request handlers and data structures.

## API Endpoints

The backend exposes the following API endpoints:

- `GET /`: A simple health check endpoint.
- `POST /api/llm`: An endpoint for interacting with various LLM providers.

For more detailed information about the API, please refer to the [API documentation](./api/README.md).