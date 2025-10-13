# API Module Documentation

This document provides a general overview of the API module in the backend.

## Overview

The `api` module is the core of the backend's functionality. It defines the handlers for all the API endpoints and is responsible for processing incoming requests, applying business logic, and sending back responses.

## Sub-modules

The `api` module is composed of the following sub-modules:

- `llm.rs`: This module contains the logic for the `/api/llm` endpoint. It handles requests to different Large Language Model (LLM) providers and returns their responses. For more details, see the [detailed `llm.rs` documentation](./llm.md).
- `config.rs`: This module is intended to manage application configuration. Currently, it contains a placeholder function. For more details, see the [detailed `config.rs` documentation](./config.md).

## Routing

The routes for this module are defined in `src/main.rs`. The `llm` handler is mapped to the `POST /api/llm` endpoint.