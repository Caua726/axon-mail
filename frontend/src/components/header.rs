//! # Header Component
//!
//! This module defines the `Header` component, which serves as the top navigation
//! bar for the application. It includes a button to toggle the sidebar, the
//! application title, and action buttons on the right.

use yew::prelude::*;

/// Properties for the `Header` component.
#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    /// A callback that will be invoked when the menu toggle button is clicked.
    /// This is used to signal the parent component to open or close the sidebar.
    pub on_sidebar_toggle: Callback<()>,
}

/// The `Header` component renders the top navigation bar of the application.
///
/// It receives a callback property, `on_sidebar_toggle`, which it uses to
/// notify the parent component when the user clicks the menu button. This allows
/// for a clean separation of concerns, where the `Header` is only responsible
/// for displaying the UI and emitting events, while the parent manages the state.
#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    // This callback is created to handle the `onclick` event of the menu toggle button.
    // It clones the `on_sidebar_toggle` callback from the props and emits an empty
    // message `()` when the button is clicked.
    let toggle_sidebar = {
        let callback = props.on_sidebar_toggle.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <header class="top-header">
            <div class="header-left">
                // The menu toggle button. When clicked, it invokes the `toggle_sidebar` callback.
                <button class="menu-toggle" onclick={toggle_sidebar}>
                    <i class="fas fa-bars"></i>
                </button>
                <div>
                    <h1 class="header-title">{"axonemail"}</h1>
                    <p class="header-subtitle">{"Find contacts efficiently"}</p>
                </div>
            </div>

            <div class="header-right">
                // Action buttons for alerts and user account.
                <button class="header-btn">
                    <i class="fas fa-bell"></i>
                    <span>{"Alerts"}</span>
                </button>
                
                <button class="header-btn is-primary">
                    <i class="fas fa-user-circle"></i>
                    <span>{"Account"}</span>
                </button>
            </div>
        </header>
    }
}