use yew::prelude::*;
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;

const SEARCH_TYPES: [&str; 3] = ["Domain", "Google", "Google + AI"];
const SIDEBAR_CLASSES: &[&str] = &["column", "is-one-fifth", "bg-sky-800", "rounded-lg", "p-4", "shadow-xl", "fixed", "left-0", "top-0", "h-full", "transform", "transition-transform", "duration-300", "z-20"];
const AI_MODELS: [&str; 2] = ["Gemini", "Ollama (Locally)"];
const OLLAMA_MODELS: [&str; 4] = ["Deepseek R1 1.5B", "Qwen 2.5 1.5B", "Deepseek R1 8B", "Phi4 14B"];
const URL: &str = "http://127.0.0.1:3000/";

async fn fetch_ai_response(url: String, prompt: String) -> String {
    let client = Client::new();
    let response = client.post(url)
        .body(prompt)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    response
}

#[function_component]
fn App() -> Html {
    let search_type_state = use_state(|| SEARCH_TYPES[2].to_string());
    let menu_open = use_state(|| true);
    let ai_model = use_state(|| AI_MODELS[0].to_string());
    let ollama_model = use_state(|| OLLAMA_MODELS[0].to_string());
    let ai_response = use_state(|| String::new());
    let ai_user_input = use_state(|| String::new());

    let aisearch = {
        let ai_model = ai_model.clone();
        let ollama_model = ollama_model.clone();
        let ai_response = ai_response.clone();
        let ai_user_input = ai_user_input.clone();
        Callback::from(move |_: MouseEvent| {
            if (*ai_model).is_empty() {
                ai_response.set("Please select an AI model.".to_string());
            } else if *ai_model == "Ollama (Locally)" {
                ai_response.set(match ollama_model.as_str() {
                    "Phi4 14B" => "Phi4 14B".to_string(),
                    "Deepseek R1 8B" => "Deepseek R1 8B model.".to_string(),
                    "Qwen 2.5 1.5B" => "Qwen 2.5 1.5B model.".to_string(),
                    "Deepseek R1 1.5B" => "Deepseek R1 1.5B model.".to_string(),
                    _ => "Invalid Ollama model selected.".to_string(),
                });
            } else if *ai_model == "Gemini" {
                let prompt = (*ai_user_input).clone();
                let url: String = URL.to_string() + "gemini";
                let response_handle = ai_response.clone();
                spawn_local(async move {
                    let response = fetch_ai_response(url, prompt).await;
                    response_handle.set(response);
                });
            } else {
                ai_response.set("Invalid AI model selected.".to_string());
            }
        })
    };
    let ai_model_on_change = {
        let ai_model = ai_model.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            ai_model.set(input.value());
        })
    };

    let ai_user_input_onchange = {
        let ai_user_input = ai_user_input.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            ai_user_input.set(input.value());
        })
    };

    let ollama_model_on_change = {
        let ollama_model = ollama_model.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            ollama_model.set(input.value());
        })
    };
    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    let search_type = {
        let search_type = search_type_state.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            search_type.set(select.value());
        })
    };

    html! {
        <div class="columns m-5 h-screen">
            <div class={classes!(SIDEBAR_CLASSES, if *menu_open { "translate-x-0" } else { "-translate-x-full" })}>
                <aside class="menu">
                    <p class="title has-text-ce has-text-white mb-5">{"Menu"}</p>
                    <ul class="menu-list">
                        <li class="mb-3">
                            <a class="has-text-white hover:bg-sky-700 p-3 rounded transition-colors">
                                <span class="fa-color"><i class="fa-solid fa-magnifying-glass mr-2"></i>{"Search"}</span>
                            </a>
                        </li>
                        <li class="mb-3">
                            <a class="has-text-white hover:bg-sky-700 p-3 rounded transition-colors">
                                <span class="fa-color"><i class="fa-solid fa-envelope mr-2"></i>{"Email History"}</span>
                            </a>
                        </li>
                        <li class="mb-3">
                            <a class="has-text-white hover:bg-sky-700 p-3 rounded transition-colors">
                                <span class="fa-color"><i class="fa-solid fa-clock-rotate-left mr-2"></i>{"Search History"}</span>
                            </a>
                        </li>
                    </ul>
                    <ul class="menu-list mt-6">
                        <li class="mb-3">
                            <a class="has-text-white hover:bg-sky-700 p-3 rounded transition-colors">
                                <span class="fa-color"><i class="fa-solid fa-gear mr-2"></i>{"Settings"}</span>
                            </a>
                        </li>
                        <li>
                            <a class="has-text-white hover:bg-sky-700 p-3 rounded transition-colors">
                                <span class="fa-color"><i class="fa-solid fa-circle-info mr-2"></i>{"About"}</span>
                            </a>
                        </li>
                    </ul>
                </aside>
            </div>
            <div class={classes!("column", "mt-5", "transition-all", "duration-300",
                if *menu_open { "ml-[20%]" } else { "ml-0" })}>
                <div class="is-flex is-align-items-center mb-4">
                    <button class="button is-size-4 mr-3" onclick={toggle_menu}
                    >
                        <i class="fa-solid fa-bars"></i>
                    </button>
                    </div>
                    <h1 class="title has-text-centered flex-grow-1">{"Email Extractor"}</h1>

                    <span class="fa-color"><i class="fa-solid fa-envelope fa-7x" style="display: flex; align-items: center; justify-content: center;"></i></span>

                <div class="is-flex is-justify-content-center is-align-items-center mt-3">
                    <div class="select is-medium">
                        <select
                        style="display: block;"
                        value={(*search_type_state).clone()}
                        onchange={search_type}
                        >
                            {for SEARCH_TYPES.iter().map(|&t| html! { <option value={t}>{t}</option> })}
                        </select>
                    </div>
                </div>
                <div key={(*search_type_state).clone()} class="box mt-5 overflow-hidden slide-animation">
                    {match (*search_type_state).as_str() {
                        "Domain" => html! {
                            <div>
                                <p class="title is-4">{"Domain Search"}</p>
                                <p>{"Enter a domain name to extract emails:"}</p><input class="input" type="text" placeholder="https://" />
                                <button class="button is-primary mt-3">{"Search"}</button>
                            </div>
                        },
                        "Google" => html! {
                            <div>
                                <p class="title is-4">{"Google Search"}</p>
                                <p>{"Enter keywords to search through Google separate by comma (ex: fish, cat, balls):"}</p>
                                <input class="input mt-3" type="text" placeholder="Search keywords" />
                                <button class="button is-primary mt-3">{"Google Search"}</button>
                            </div>
                         },
                        "Google + AI" => html! {
                            <div>
                                <p class="title is-4"><i class="fas fa-robot is-4"></i>{" AI-Powered Search"}</p>
                                <p class=" mt-2 is-size-5 subtitle has-text-grey">{"What Model You Want To Use?"}</p>
                                <div class="is-flex">
                                    <div class="select">
                                        <select onchange={ai_model_on_change}>
                                            {AI_MODELS.iter().map(|model| {
                                                let is_selected = *model == "Gemini"; // Check if this is the default model
                                                html! {
                                                    <option selected={is_selected}>{*model}</option>
                                                }
                                            }).collect::<Html>()}
                                        </select>
                                    </div>
                                    <div class="ml-5">
                                        {
                                            match ai_model.as_str() {
                                                "Ollama (Locally)" => html! {
                                                    <div class="select">
                                                        <select onchange={ollama_model_on_change}>
                                                            {OLLAMA_MODELS.iter().map(|model| {
                                                                let is_selected = *model == "Deepseek R1 1.5B"; // Check if this is the default model
                                                                html! {
                                                                    <option selected={is_selected}>{*model}</option>
                                                                }
                                                            }).collect::<Html>()}
                                                        </select>
                                                    </div>
                                                },
                                                _ => html! {},
                                            }
                                        }
                                    </div>
                                </div>
                                <p class="mt-3">{"What do you want search?"}</p>
                                <div class="columns">
                                    <div class="column">
                                        <textarea value={(*ai_user_input).clone()} oninput={ai_user_input_onchange} class="textarea mt-3" id="ai_user_input" placeholder="Describe your search needs"></textarea>
                                    </div>
                                    <div class="column">
                                        <textarea value={(*ai_response).clone()} class="textarea mt-3" id="ai_response" placeholder="Response"></textarea>
                                    </div>
                                </div>
                                <button onclick={aisearch} class="button is-primary mt-5">{"AI Search"}</button>
                            </div>
                        },
                        _ => html! { <h1 class="title">{"Domain"}</h1> },
                    }}
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}