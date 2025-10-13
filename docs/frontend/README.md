# Frontend Documentation

This document provides a high-level overview of the frontend architecture of the Axon-Mail application.

## Overview

The frontend is a modern single-page application (SPA) built entirely in Rust using the Yew framework. It compiles to WebAssembly (Wasm), allowing it to run natively in the browser with near-native performance. The frontend is responsible for rendering the user interface, managing user interactions, and communicating with the backend API.

## Key Technologies

- **Rust**: The programming language used for the frontend, enabling a high-performance, type-safe, and concurrent application.
- **Yew**: A modern Rust framework for creating multi-threaded frontend web apps with a component-based architecture, inspired by React.
- **WebAssembly (Wasm)**: The compilation target for the frontend code, which allows it to run in the browser.
- **yew-router**: A routing library for Yew applications, used to manage navigation and different pages.
- **uuid**: A crate for generating unique identifiers, used here to create unique IDs for each search session.

## Project Structure

The frontend code is organized into the following main parts:

- `src/main.rs`: The main entry point of the application. It sets up the routing and renders the main application component.
- `src/components/`: This module contains all the reusable UI components that make up the application's interface.

## Component-Based Architecture

The application is built around a set of components, each with its own state and logic. This makes the codebase modular, reusable, and easy to maintain. For more information about the components, please refer to the [components documentation](./components/README.md).