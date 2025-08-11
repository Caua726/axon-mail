use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub on_sidebar_toggle: Callback<()>,
}

#[function_component]
pub fn Header(props: &HeaderProps) -> Html {
    let toggle_sidebar = {
        let callback = props.on_sidebar_toggle.clone();
        Callback::from(move |_| callback.emit(()))
    };

    html! {
        <header class="top-header">
            <div class="header-left">
                <button class="menu-toggle" onclick={toggle_sidebar}>
                    <i class="fas fa-bars"></i>
                </button>
                <div>
                    <h1 class="header-title">{"axonemail"}</h1>
                    <p class="header-subtitle">{"Find contacts efficiently"}</p>
                </div>
            </div>

            <div class="header-right">
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