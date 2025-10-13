# Axon-Mail

Axon-Mail is a full-stack web application designed for finding and managing email contacts efficiently. It features a powerful backend service and a modern, responsive frontend built entirely in Rust.

## Table of Contents

- [Features](#features)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Setup and Installation](#setup-and-installation)
  - [Prerequisites](#prerequisites)
  - [Backend Setup](#backend-setup)
  - [Frontend Setup](#frontend-setup)
- [Running the Application](#running-the-application)
- [Documentation](#documentation)

## Features

- **Advanced Email Search**: Perform searches by business category, specific company, or domain.
- **Modern UI**: A responsive and intuitive user interface built with the Yew framework.
- **RESTful API**: A robust backend API built with Axum.
- **Extensible LLM Integration**: The backend is designed to support multiple Large Language Model (LLM) providers.

## Architecture

The project is a monorepo containing two main crates:

- **`backend`**: A Rust-based web server using the Axum framework. It provides a RESTful API for the frontend.
- **`frontend`**: A single-page application (SPA) built with the Yew framework, which compiles to WebAssembly (Wasm) to run in the browser.

## Project Structure

```
.
├── backend/         # Backend Axum application
│   ├── src/
│   └── Cargo.toml
├── docs/            # All project documentation
│   ├── backend/
│   ├── frontend/
│   └── main.md
├── frontend/        # Frontend Yew application
│   ├── src/
│   └── Cargo.toml
├── src/             # Root project source (minimal)
│   └── main.rs
├── .env             # Environment variables
├── Cargo.toml       # Root Cargo.toml (workspace definition)
└── README.md
```

## Setup and Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install) (for building the frontend)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (also for the frontend)

### Backend Setup

1.  Navigate to the `backend` directory:
    ```sh
    cd backend
    ```

2.  Install the dependencies:
    ```sh
    cargo build
    ```

### Frontend Setup

1.  Navigate to the `frontend` directory:
    ```sh
    cd frontend
    ```

2.  Install the wasm32 target:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

3.  Build the project:
    ```sh
    trunk build
    ```

## Running the Application

### 1. Set up Environment Variables

Create a `.env` file in the root of the project with the following content:

```
BACKEND_IP=127.0.0.1
BACKEND_PORT=3000
```

### 2. Run the Backend

In a terminal, navigate to the `backend` directory and run:

```sh
cargo run
```

The backend server should now be running at `http://127.0.0.1:3000`.

### 3. Run the Frontend

In a separate terminal, navigate to the `frontend` directory and run:

```sh
trunk serve
```

The frontend application will be available at `http://127.0.0.1:8080`.

## Documentation

This repository is thoroughly documented. For more details, please refer to the documentation directory:

- **[Root Documentation](./docs/main.md)**
- **Backend Documentation**
  - [General Overview](./docs/backend/README.md)
  - [API Module](./docs/backend/api/README.md)
  - [Hyper-Detailed `llm.rs`](./docs/backend/api/llm.md)
  - [Hyper-Detailed `config.rs`](./docs/backend/api/config.md)
- **Frontend Documentation**
  - [General Overview](./docs/frontend/README.md)
  - [Components Module](./docs/frontend/components/README.md)
  - [Hyper-Detailed `sidebar.rs`](./docs/frontend/components/sidebar.md)
  - [Hyper-Detailed `header.rs`](./docs/frontend/components/header.md)
  - [Hyper-Detailed `search_interface.rs`](./docs/frontend/components/search_interface.md)
  - [Hyper-Detailed `email_search.rs`](./docs/frontend/components/email_search.md)