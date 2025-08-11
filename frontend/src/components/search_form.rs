use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

const AI_MODELS: [&str; 2] = ["Gemini", "Ollama (Local)"];
const OLLAMA_MODELS: [&str; 4] = ["Deepseek R1 1.5B", "Qwen 2.5 1.5B", "Deepseek R1 8B", "Phi4 14B"];
const URL: &str = "http://127.0.0.1:3000/";

async fn fetch_ai_response(url: String, prompt: String) -> String {
    let client = Client::new();
    match client.post(url).body(prompt).send().await {
        Ok(response) => response.text().await.unwrap_or_else(|_| "Error reading response".to_string()),
        Err(_) => "Network error occurred".to_string(),
    }
}

#[derive(Properties, PartialEq)]
pub struct SearchFormProps {
    pub search_type: String,
}

#[function_component]
pub fn SearchForm(props: &SearchFormProps) -> Html {
    // Basic search states
    let search_query = use_state(|| String::new());
    let ai_model = use_state(|| AI_MODELS[0].to_string());
    let ollama_model = use_state(|| OLLAMA_MODELS[0].to_string());
    let ai_response = use_state(|| String::new());
    let is_loading = use_state(|| false);
    
    // Advanced search states
    let domain_filter = use_state(|| String::new());
    let location_filter = use_state(|| String::new());
    let job_title_filter = use_state(|| String::new());
    let company_filter = use_state(|| String::new());
    let industry_filter = use_state(|| String::new());
    let results_limit = use_state(|| 100);
    let search_depth = use_state(|| "standard".to_string());
    let email_types = use_state(|| vec!["personal".to_string(), "business".to_string()]);
    let exclude_domains = use_state(|| String::new());
    let min_confidence = use_state(|| 80);

    let on_search_query_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            search_query.set(input.value());
        })
    };

    let on_ai_model_change = {
        let ai_model = ai_model.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            ai_model.set(select.value());
        })
    };

    let on_ollama_model_change = {
        let ollama_model = ollama_model.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            ollama_model.set(select.value());
        })
    };

    let on_search = {
        let search_query = search_query.clone();
        let ai_model = ai_model.clone();
        let ollama_model = ollama_model.clone();
        let ai_response = ai_response.clone();
        let is_loading = is_loading.clone();
        let search_type = props.search_type.clone();
        let domain_filter = domain_filter.clone();
        let location_filter = location_filter.clone();
        let job_title_filter = job_title_filter.clone();
        let company_filter = company_filter.clone();
        let industry_filter = industry_filter.clone();
        let results_limit = results_limit.clone();

        Callback::from(move |_: MouseEvent| {
            if search_query.is_empty() {
                ai_response.set("Please enter your search query.".to_string());
                return;
            }

            is_loading.set(true);
            
            if search_type == "Search" {
                // Simple AI search
                if *ai_model == "Ollama (Local)" {
                    let response = format!("AI Search Results for: \"{}\"\n\nUsing {} model to find emails...\n\nThis would return email addresses matching your query.", *search_query, *ollama_model);
                    ai_response.set(response);
                    is_loading.set(false);
                } else if *ai_model == "Gemini" {
                    let prompt = (*search_query).clone();
                    let url: String = URL.to_string() + "gemini";
                    let response_handle = ai_response.clone();
                    let loading_handle = is_loading.clone();
                    
                    spawn_local(async move {
                        let response = fetch_ai_response(url, prompt).await;
                        response_handle.set(response);
                        loading_handle.set(false);
                    });
                }
            } else {
                // Advanced search with all filters
                let mut search_params = vec![format!("Query: {}", *search_query)];
                
                if !domain_filter.is_empty() {
                    search_params.push(format!("Domain: {}", *domain_filter));
                }
                if !location_filter.is_empty() {
                    search_params.push(format!("Location: {}", *location_filter));
                }
                if !job_title_filter.is_empty() {
                    search_params.push(format!("Job Title: {}", *job_title_filter));
                }
                if !company_filter.is_empty() {
                    search_params.push(format!("Company: {}", *company_filter));
                }
                if !industry_filter.is_empty() {
                    search_params.push(format!("Industry: {}", *industry_filter));
                }
                
                search_params.push(format!("Results Limit: {}", *results_limit));
                search_params.push(format!("AI Model: {}", *ai_model));
                
                let response = format!("Advanced Search Initiated\n\n{}\n\nThis enterprise-level search would process multiple data sources and return detailed email profiles with verification status.", search_params.join("\n"));
                ai_response.set(response);
                is_loading.set(false);
            }
        })
    };

    match props.search_type.as_str() {
        "Search" => html! {
            <div class="search-card">
                <div class="form-group">
                    <label class="form-label">{"Search Query"}</label>
                    <input 
                        type="text"
                        class="form-control"
                        placeholder="e.g., startup founders in San Francisco, marketing directors at SaaS companies..."
                        value={(*search_query).clone()}
                        oninput={on_search_query_change}
                    />
                    <small style="color: #6c757d; font-size: 0.875rem;">{"Describe what you're looking for in natural language"}</small>
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{"AI Model"}</label>
                        <select class="form-control form-select" onchange={on_ai_model_change}>
                            {AI_MODELS.iter().map(|model| html! {
                                <option value={*model} selected={*model == *ai_model}>{*model}</option>
                            }).collect::<Html>()}
                        </select>
                    </div>

                    if *ai_model == "Ollama (Local)" {
                        <div class="form-group">
                            <label class="form-label">{"Local Model"}</label>
                            <select class="form-control form-select" onchange={on_ollama_model_change}>
                                {OLLAMA_MODELS.iter().map(|model| html! {
                                    <option value={*model} selected={*model == *ollama_model}>{*model}</option>
                                }).collect::<Html>()}
                            </select>
                        </div>
                    }
                </div>

                <button
                    class={if *is_loading { "btn btn-primary btn-lg loading" } else { "btn btn-primary btn-lg" }}
                    onclick={on_search}
                    disabled={*is_loading}
                    style="width: 100%; margin-top: 1rem;"
                >
                    if *is_loading {
                        {"Searching..."}
                    } else {
                        <><i class="fas fa-search"></i>{"Start Search"}</>
                    }
                </button>

                if !ai_response.is_empty() {
                    <div class="results-area">
                        <h3 style="margin-top: 0; color: #495057;">{"Results"}</h3>
                        <textarea 
                            class="form-control"
                            readonly=true
                            value={(*ai_response).clone()}
                            style="min-height: 200px; font-family: monospace;"
                        ></textarea>
                        
                        <div style="display: flex; gap: 1rem; margin-top: 1rem;">
                            <button class="btn btn-success">
                                <i class="fas fa-download"></i>
                                {"Export CSV"}
                            </button>
                            <button class="btn btn-outline-secondary">
                                <i class="fas fa-save"></i>
                                {"Save List"}
                            </button>
                            <button class="btn btn-outline-secondary">
                                <i class="fas fa-envelope"></i>
                                {"Verify Emails"}
                            </button>
                        </div>
                    </div>
                }
            </div>
        },
        "Advanced Search" => html! {
            <div class="search-card">
                <h3 style="margin-top: 0; color: #495057;">{"Enterprise Search Filters"}</h3>
                
                <div class="form-group">
                    <label class="form-label">{"Search Query"}</label>
                    <input 
                        type="text"
                        class="form-control"
                        placeholder="Primary search terms..."
                        value={(*search_query).clone()}
                        oninput={on_search_query_change}
                    />
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{"Target Domain"}</label>
                        <input 
                            type="text"
                            class="form-control"
                            placeholder="company.com"
                            value={(*domain_filter).clone()}
                            oninput={
                                let domain_filter = domain_filter.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    domain_filter.set(input.value());
                                })
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label class="form-label">{"Location"}</label>
                        <input 
                            type="text"
                            class="form-control"
                            placeholder="San Francisco, CA"
                            value={(*location_filter).clone()}
                            oninput={
                                let location_filter = location_filter.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    location_filter.set(input.value());
                                })
                            }
                        />
                    </div>
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{"Job Title"}</label>
                        <input 
                            type="text"
                            class="form-control"
                            placeholder="CEO, Marketing Director, Developer"
                            value={(*job_title_filter).clone()}
                            oninput={
                                let job_title_filter = job_title_filter.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    job_title_filter.set(input.value());
                                })
                            }
                        />
                    </div>

                    <div class="form-group">
                        <label class="form-label">{"Company"}</label>
                        <input 
                            type="text"
                            class="form-control"
                            placeholder="Company name or pattern"
                            value={(*company_filter).clone()}
                            oninput={
                                let company_filter = company_filter.clone();
                                Callback::from(move |e: InputEvent| {
                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                    company_filter.set(input.value());
                                })
                            }
                        />
                    </div>
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{"Industry"}</label>
                        <select class="form-control form-select" 
                            onchange={
                                let industry_filter = industry_filter.clone();
                                Callback::from(move |e: Event| {
                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                    industry_filter.set(select.value());
                                })
                            }>
                            <option value="">{"All Industries"}</option>
                            <option value="technology">{"Technology"}</option>
                            <option value="finance">{"Finance"}</option>
                            <option value="healthcare">{"Healthcare"}</option>
                            <option value="retail">{"Retail"}</option>
                            <option value="manufacturing">{"Manufacturing"}</option>
                            <option value="consulting">{"Consulting"}</option>
                            <option value="education">{"Education"}</option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label class="form-label">{"Results Limit"}</label>
                        <select class="form-control form-select"
                            onchange={
                                let results_limit = results_limit.clone();
                                Callback::from(move |e: Event| {
                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                    if let Ok(limit) = select.value().parse::<i32>() {
                                        results_limit.set(limit);
                                    }
                                })
                            }>
                            <option value="50" selected={*results_limit == 50}>{"50 results"}</option>
                            <option value="100" selected={*results_limit == 100}>{"100 results"}</option>
                            <option value="250" selected={*results_limit == 250}>{"250 results"}</option>
                            <option value="500" selected={*results_limit == 500}>{"500 results"}</option>
                            <option value="1000" selected={*results_limit == 1000}>{"1,000 results"}</option>
                        </select>
                    </div>
                </div>

                <div class="form-row">
                    <div class="form-group">
                        <label class="form-label">{"Search Depth"}</label>
                        <select class="form-control form-select"
                            onchange={
                                let search_depth = search_depth.clone();
                                Callback::from(move |e: Event| {
                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                    search_depth.set(select.value());
                                })
                            }>
                            <option value="fast" selected={*search_depth == "fast"}>{"Fast (Surface web)"}</option>
                            <option value="standard" selected={*search_depth == "standard"}>{"Standard (Multiple sources)"}</option>
                            <option value="deep" selected={*search_depth == "deep"}>{"Deep (Comprehensive crawl)"}</option>
                        </select>
                    </div>

                    <div class="form-group">
                        <label class="form-label">{"AI Model"}</label>
                        <select class="form-control form-select" onchange={on_ai_model_change}>
                            {AI_MODELS.iter().map(|model| html! {
                                <option value={*model} selected={*model == *ai_model}>{*model}</option>
                            }).collect::<Html>()}
                        </select>
                    </div>
                </div>

                <div class="form-group">
                    <label class="form-label">{"Exclude Domains"}</label>
                    <input 
                        type="text"
                        class="form-control"
                        placeholder="spam.com, noreply.com (comma separated)"
                        value={(*exclude_domains).clone()}
                        oninput={
                            let exclude_domains = exclude_domains.clone();
                            Callback::from(move |e: InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                exclude_domains.set(input.value());
                            })
                        }
                    />
                </div>

                <div class="checkbox-group" style="margin-bottom: 1.5rem;">
                    <div class="checkbox-item">
                        <input type="checkbox" id="include-social" />
                        <label for="include-social">{"Include social media profiles"}</label>
                    </div>
                    <div class="checkbox-item">
                        <input type="checkbox" id="verify-emails" checked=true />
                        <label for="verify-emails">{"Verify email addresses"}</label>
                    </div>
                    <div class="checkbox-item">
                        <input type="checkbox" id="enrich-data" />
                        <label for="enrich-data">{"Enrich with company data"}</label>
                    </div>
                    <div class="checkbox-item">
                        <input type="checkbox" id="phone-numbers" />
                        <label for="phone-numbers">{"Include phone numbers"}</label>
                    </div>
                </div>

                <button
                    class={if *is_loading { "btn btn-primary btn-lg loading" } else { "btn btn-primary btn-lg" }}
                    onclick={on_search}
                    disabled={*is_loading}
                    style="width: 100%; margin-bottom: 1rem;"
                >
                    if *is_loading {
                        {"Processing Advanced Search..."}
                    } else {
                        <><i class="fas fa-rocket"></i>{"Launch Enterprise Search"}</>
                    }
                </button>

                if !ai_response.is_empty() {
                    <div class="results-area">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                            <h3 style="margin: 0; color: #495057;">{"Search Results"}</h3>
                            <div style="display: flex; gap: 0.5rem;">
                                <button class="btn btn-success">
                                    <i class="fas fa-download"></i>
                                    {"Export"}
                                </button>
                                <button class="btn btn-outline-secondary">
                                    <i class="fas fa-filter"></i>
                                    {"Filter"}
                                </button>
                            </div>
                        </div>
                        
                        <textarea 
                            class="form-control"
                            readonly=true
                            value={(*ai_response).clone()}
                            style="min-height: 250px; font-family: monospace;"
                        ></textarea>
                    </div>
                }
            </div>
        },
        _ => html! {
            <div class="search-card">
                <p style="text-align: center; color: #6c757d;">{"Select a search type to get started"}</p>
            </div>
        }
    }
}