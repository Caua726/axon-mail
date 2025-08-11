use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;

const AI_MODELS: [&str; 2] = ["Gemini", "Ollama (Locally)"];
const OLLAMA_MODELS: [&str; 4] = ["Deepseek R1 1.5B", "Qwen 2.5 1.5B", "Deepseek R1 8B", "Phi4 14B"];
const URL: &str = "http://127.0.0.1:3000/";

async fn fetch_ai_response(url: String, prompt: String) -> String {
    let client = Client::new();
    match client.post(url).body(prompt).send().await {
        Ok(response) => response.text().await.unwrap_or_else(|_| "Error reading response".to_string()),
        Err(_) => "Network error occurred".to_string(),
    }
}

#[function_component]
pub fn AiSearch() -> Html {
    let ai_model = use_state(|| AI_MODELS[0].to_string());
    let ollama_model = use_state(|| OLLAMA_MODELS[0].to_string());
    let ai_response = use_state(|| String::new());
    let ai_user_input = use_state(|| String::new());
    let is_loading = use_state(|| false);

    let ai_search = {
        let ai_model = ai_model.clone();
        let ollama_model = ollama_model.clone();
        let ai_response = ai_response.clone();
        let ai_user_input = ai_user_input.clone();
        let is_loading = is_loading.clone();
        
        Callback::from(move |_: MouseEvent| {
            if ai_user_input.is_empty() {
                ai_response.set("Please enter your search query first.".to_string());
                return;
            }

            is_loading.set(true);
            
            if *ai_model == "Ollama (Locally)" {
                let response = format!("Selected Ollama model: {} - This would process your query: \"{}\"", 
                    *ollama_model, *ai_user_input);
                ai_response.set(response);
                is_loading.set(false);
            } else if *ai_model == "Gemini" {
                let prompt = (*ai_user_input).clone();
                let url: String = URL.to_string() + "gemini";
                let response_handle = ai_response.clone();
                let loading_handle = is_loading.clone();
                
                spawn_local(async move {
                    let response = fetch_ai_response(url, prompt).await;
                    response_handle.set(response);
                    loading_handle.set(false);
                });
            } else {
                ai_response.set("Invalid AI model selected.".to_string());
                is_loading.set(false);
            }
        })
    };

    let ai_model_on_change = {
        let ai_model = ai_model.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
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
            let input = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
            ollama_model.set(input.value());
        })
    };

    html! {
        <div class="bg-gradient-to-br from-slate-800/50 via-purple-900/30 to-slate-800/50 backdrop-blur-xl border border-purple-500/20 rounded-2xl p-8 hover:border-purple-400/40 transition-all duration-300 hover:shadow-2xl hover:shadow-purple-500/10 group">
            <div class="flex items-center mb-6">
                <div class="w-12 h-12 bg-gradient-to-r from-purple-500 to-pink-500 rounded-xl flex items-center justify-center mr-4 group-hover:scale-110 transition-transform duration-300">
                    <i class="fas fa-robot text-white text-xl"></i>
                </div>
                <div>
                    <h3 class="text-2xl font-bold text-white mb-1 flex items-center">
                        {"AI-Powered Search"}
                        <span class="ml-2 px-2 py-1 bg-gradient-to-r from-purple-600/30 to-pink-600/30 text-xs font-medium text-purple-300 border border-purple-500/30 rounded-full">{"BETA"}</span>
                    </h3>
                    <p class="text-purple-300/80">{"Let AI help you find the perfect emails"}</p>
                </div>
            </div>

            <div class="space-y-6">
                // AI Model Selection
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">
                            {"AI Model"}
                            <span class="text-red-400 ml-1">{"*"}</span>
                        </label>
                        <div class="relative">
                            <select 
                                onchange={ai_model_on_change}
                                value={(*ai_model).clone()}
                                class="block w-full py-3 px-4 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all duration-200 hover:bg-white/10"
                            >
                                {AI_MODELS.iter().map(|model| html! {
                                    <option value={*model}>{*model}</option>
                                }).collect::<Html>()}
                            </select>
                            <div class="absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                                <i class="fas fa-chevron-down text-gray-400"></i>
                            </div>
                        </div>
                    </div>

                    if *ai_model == "Ollama (Locally)" {
                        <div>
                            <label class="block text-sm font-medium text-gray-300 mb-2">
                                {"Local Model"}
                                <span class="text-red-400 ml-1">{"*"}</span>
                            </label>
                            <div class="relative">
                                <select 
                                    onchange={ollama_model_on_change}
                                    value={(*ollama_model).clone()}
                                    class="block w-full py-3 px-4 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all duration-200 hover:bg-white/10"
                                >
                                    {OLLAMA_MODELS.iter().map(|model| html! {
                                        <option value={*model}>{*model}</option>
                                    }).collect::<Html>()}
                                </select>
                                <div class="absolute inset-y-0 right-0 flex items-center pr-2 pointer-events-none">
                                    <i class="fas fa-chevron-down text-gray-400"></i>
                                </div>
                            </div>
                        </div>
                    }
                </div>

                // Input and Response
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">
                            {"Describe your search needs"}
                            <span class="text-red-400 ml-1">{"*"}</span>
                        </label>
                        <div class="relative">
                            <textarea
                                value={(*ai_user_input).clone()}
                                oninput={ai_user_input_onchange}
                                placeholder="e.g., Find emails of startup founders in the fintech industry, or marketing managers at SaaS companies..."
                                rows="8"
                                class="block w-full p-4 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-500/50 focus:border-purple-500/50 transition-all duration-200 hover:bg-white/10 resize-none"
                            ></textarea>
                            <div class="absolute bottom-3 right-3 text-xs text-gray-400">
                                {format!("{}/500", ai_user_input.len())}
                            </div>
                        </div>
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">
                            {"AI Response"}
                        </label>
                        <div class="relative">
                            <textarea
                                value={(*ai_response).clone()}
                                placeholder="AI response will appear here..."
                                readonly=true
                                rows="8"
                                class="block w-full p-4 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-400 resize-none"
                            ></textarea>
                            if *is_loading {
                                <div class="absolute inset-0 bg-white/5 rounded-xl flex items-center justify-center">
                                    <div class="flex items-center space-x-2 text-purple-400">
                                        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-purple-400"></div>
                                        <span class="text-sm font-medium">{"Processing..."}</span>
                                    </div>
                                </div>
                            }
                        </div>
                    </div>
                </div>

                // Action Buttons
                <div class="flex flex-col sm:flex-row gap-3">
                    <button 
                        onclick={ai_search}
                        disabled={*is_loading}
                        class="flex-1 bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-500 hover:to-pink-500 disabled:from-gray-600 disabled:to-gray-600 text-white font-semibold py-3 px-6 rounded-xl transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] hover:shadow-lg hover:shadow-purple-500/25 flex items-center justify-center group disabled:cursor-not-allowed disabled:hover:scale-100"
                    >
                        if *is_loading {
                            <>
                                <div class="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
                                {"Processing..."}
                            </>
                        } else {
                            <>
                                <i class="fas fa-magic mr-2 group-hover:scale-110 transition-transform"></i>
                                {"Generate AI Search"}
                            </>
                        }
                    </button>
                    
                    <button class="px-6 py-3 bg-white/5 hover:bg-white/10 border border-white/10 hover:border-purple-500/30 text-gray-300 hover:text-white rounded-xl transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] flex items-center justify-center">
                        <i class="fas fa-eraser mr-2"></i>
                        {"Clear"}
                    </button>
                    
                    <button class="px-6 py-3 bg-white/5 hover:bg-white/10 border border-white/10 hover:border-green-500/30 text-gray-300 hover:text-green-400 rounded-xl transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] flex items-center justify-center">
                        <i class="fas fa-download mr-2"></i>
                        {"Export"}
                    </button>
                </div>

                // Tips
                <div class="bg-gradient-to-r from-purple-500/10 to-pink-500/10 border border-purple-500/20 rounded-xl p-4">
                    <div class="flex items-start space-x-3">
                        <i class="fas fa-lightbulb text-yellow-400 mt-1"></i>
                        <div>
                            <h4 class="font-semibold text-white mb-2">{"Pro Tips:"}</h4>
                            <ul class="text-sm text-gray-300 space-y-1">
                                <li>{"• Be specific about the industry, role, or company type"}</li>
                                <li>{"• Include location preferences if relevant"}</li>
                                <li>{"• Mention any specific domains or company sizes"}</li>
                                <li>{"• Use natural language - the AI understands context"}</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}