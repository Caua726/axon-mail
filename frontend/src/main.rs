//! # Axon-Mail Frontend
//!
//! This is the main entry point for the Axon-Mail frontend application.
//! The application is built using the Yew framework and handles all the user
//! interface logic, including routing, state management, and component rendering.
//!
//! ## Features
//!
//! - **Component-Based Architecture**: The UI is built using a set of reusable
//!   components, such as `Sidebar`, `Header`, `SearchInterface`, and `EmailSearch`.
//! - **Routing**: It uses `yew_router` to manage navigation between different pages
//!   of the application, such as the home page and the search results page.
//! - **State Management**: The application uses Yew's `use_state` hook for managing
//!   local component state, such as the visibility of the sidebar.
//!
//! ## Crate Dependencies
//!
//! - `yew`: The main framework for building the web application.
//! - `yew_router`: Provides routing capabilities.
//! - `uuid`: Used to generate unique IDs for new search sessions.

use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;

mod components;
use components::{
    sidebar::Sidebar,
    header::Header,
    search_interface::SearchInterface,
    email_search::EmailSearch,
};

/// Defines the routes for the application.
#[derive(Clone, Routable, PartialEq)]
enum Route {
    /// The home page, which displays the main search interface.
    #[at("/")]
    Home,
    /// The search results page. It takes a unique `id` as a parameter to identify the search session.
    #[at("/buscando/:id")]
    Buscando { id: String },
    /// The "Not Found" page, which is displayed for any unrecognized routes.
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// The main routing switch for the application.
///
/// This function takes a `Route` enum and returns the corresponding page component
/// to be rendered.
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Buscando { id } => html! { <BuscandoPage search_id={id} /> },
        Route::NotFound => html! { <h1>{ "404 - Página não encontrada" }</h1> },
    }
}

/// The component for the home page (`/`).
///
/// This component renders the main layout, including the `Sidebar`, `Header`,
/// and the `SearchInterface` component. It also manages the state of the sidebar.
#[function_component]
fn HomePage() -> Html {
    // State to control whether the sidebar is open or closed.
    let sidebar_open = use_state(|| false);
    // The navigator hook from yew_router, used for programmatic navigation.
    let navigator = use_navigator().unwrap();

    // Callback to toggle the sidebar's state.
    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_| sidebar_open.set(!*sidebar_open))
    };

    // Callback to start a new search. It generates a new UUID for the search ID
    // and navigates to the search results page.
    let start_search = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let search_id = Uuid::new_v4().to_string();
            navigator.push(&Route::Buscando { id: search_id });
        })
    };

    html! {
        <div class="main-container">
            <Sidebar is_open={*sidebar_open} on_toggle={toggle_sidebar.clone()} />
            
            <div class={classes!("content-area", if *sidebar_open { "sidebar-open" } else { "" })}>
                <Header on_sidebar_toggle={toggle_sidebar} />
                
                <main class="main-content">
                    <SearchInterface on_start_search={start_search} />
                </main>
            </div>
        </div>
    }
}

/// Properties for the `BuscandoPage` component.
#[derive(Properties, PartialEq)]
struct BuscandoPageProps {
    /// The unique ID of the search, passed from the route.
    pub search_id: String,
}

/// The component for the search results page (`/buscando/:id`).
///
/// This component renders the main layout and the `EmailSearch` component,
/// passing the `search_id` to it. It also provides a callback to navigate back
/// to the home page.
#[function_component]
fn BuscandoPage(props: &BuscandoPageProps) -> Html {
    let sidebar_open = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_| sidebar_open.set(!*sidebar_open))
    };

    // Callback to navigate back to the home page.
    let go_back = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };

    html! {
        <div class="main-container">
            <Sidebar is_open={*sidebar_open} on_toggle={toggle_sidebar.clone()} />
            
            <div class={classes!("content-area", if *sidebar_open { "sidebar-open" } else { "" })}>
                <Header on_sidebar_toggle={toggle_sidebar} />
                
                <main class="main-content">
                    <EmailSearch search_id={props.search_id.clone()} on_back={go_back} />
                </main>
            </div>
        </div>
    }
}

/// The root component of the application.
///
/// This component sets up the `BrowserRouter` and the `Switch` component,
/// which are necessary for `yew_router` to work.
#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// The main entry point of the frontend application.
///
/// This function creates a new Yew renderer and mounts the `App` component
/// to the document's body, starting the application.
fn main() {
    yew::Renderer::<App>::new().render();
}