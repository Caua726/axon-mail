//! # Components Module
//!
//! This module serves as the central hub for all the UI components used in the application.
//! By declaring each component as a public submodule, it allows other parts of the application,
//! such as the main application logic in `main.rs`, to easily import and use them.
//!
//! ## Component-Based Architecture
//!
//! The application follows a component-based architecture, which is a core principle of
//! modern frontend development and frameworks like Yew. This approach offers several benefits:
//!
//! - **Reusability**: Components can be reused across different parts of the application,
//!   which reduces code duplication and development time.
//! - **Encapsulation**: Each component encapsulates its own logic and styling, making it
//!   easier to understand, maintain, and test in isolation.
//! - **Composability**: Simple components can be combined to create more complex UIs,
//!   leading to a more modular and scalable codebase.
//!
//! ## Available Components
//!
//! The following components are available in this module:
//!
//! - `sidebar`: A collapsible sidebar for navigation.
//! - `header`: The main header of the application, containing the title and action buttons.
//! - `search_interface`: The main interface for initiating an email search.
//! - `email_search`: The view that displays the results of an active email search.

/// The `sidebar` module, which defines the main navigation sidebar.
pub mod sidebar;

/// The `header` module, which defines the top header bar of the application.
pub mod header;

/// The `search_interface` module, which provides the UI for starting a new search.
pub mod search_interface;

/// The `email_search` module, which displays the real-time results of a search.
pub mod email_search;