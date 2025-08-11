use yew::prelude::*;

mod components;
use components::{
    sidebar::Sidebar,
    header::Header,
    search_interface::SearchInterface,
};

#[function_component]
fn App() -> Html {
    let sidebar_open = use_state(|| false);

    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_| sidebar_open.set(!*sidebar_open))
    };

    html! {
        <div class="main-container">
            <Sidebar is_open={*sidebar_open} on_toggle={toggle_sidebar.clone()} />
            
            <div class={classes!("content-area", if *sidebar_open { "sidebar-open" } else { "" })}>
                <Header on_sidebar_toggle={toggle_sidebar} />
                
                <main class="main-content">
                    <SearchInterface />
                </main>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}