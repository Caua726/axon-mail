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

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/buscando/:id")]
    Buscando { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::Buscando { id } => html! { <BuscandoPage search_id={id} /> },
        Route::NotFound => html! { <h1>{ "404 - Página não encontrada" }</h1> },
    }
}

#[function_component]
fn HomePage() -> Html {
    let sidebar_open = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_| sidebar_open.set(!*sidebar_open))
    };

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

#[derive(Properties, PartialEq)]
struct BuscandoPageProps {
    pub search_id: String,
}

#[function_component]
fn BuscandoPage(props: &BuscandoPageProps) -> Html {
    let sidebar_open = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_| sidebar_open.set(!*sidebar_open))
    };

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

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}