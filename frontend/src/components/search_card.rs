use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchCardProps {
    pub search_type: String,
}

#[function_component]
pub fn SearchCard(props: &SearchCardProps) -> Html {
    match props.search_type.as_str() {
        "Domain" => html! {
            <div class="bg-gradient-to-br from-slate-800/50 via-blue-900/30 to-slate-800/50 backdrop-blur-xl border border-blue-500/20 rounded-2xl p-8 hover:border-blue-400/40 transition-all duration-300 hover:shadow-2xl hover:shadow-blue-500/10 group">
                <div class="flex items-center mb-6">
                    <div class="w-12 h-12 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-xl flex items-center justify-center mr-4 group-hover:scale-110 transition-transform duration-300">
                        <i class="fas fa-globe text-white text-xl"></i>
                    </div>
                    <div>
                        <h3 class="text-2xl font-bold text-white mb-1">{"Domain Search"}</h3>
                        <p class="text-blue-300/80">{"Extract emails from specific domains"}</p>
                    </div>
                </div>

                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">
                            {"Target Domain"}
                            <span class="text-red-400 ml-1">{"*"}</span>
                        </label>
                        <div class="relative">
                            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <i class="fas fa-link text-blue-400"></i>
                            </div>
                            <input
                                type="url"
                                class="block w-full pl-10 pr-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500/50 transition-all duration-200 hover:bg-white/10"
                                placeholder="https://example.com"
                            />
                        </div>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <div>
                            <label class="flex items-center space-x-2 text-gray-300 hover:text-white transition-colors cursor-pointer">
                                <input type="checkbox" class="w-4 h-4 text-blue-500 bg-white/5 border-gray-600 rounded focus:ring-blue-500/50" />
                                <span class="text-sm">{"Deep crawl"}</span>
                            </label>
                        </div>
                        <div>
                            <label class="flex items-center space-x-2 text-gray-300 hover:text-white transition-colors cursor-pointer">
                                <input type="checkbox" class="w-4 h-4 text-blue-500 bg-white/5 border-gray-600 rounded focus:ring-blue-500/50" />
                                <span class="text-sm">{"Include subdomains"}</span>
                            </label>
                        </div>
                    </div>

                    <button class="w-full mt-6 bg-gradient-to-r from-blue-600 to-cyan-600 hover:from-blue-500 hover:to-cyan-500 text-white font-semibold py-3 px-6 rounded-xl transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] hover:shadow-lg hover:shadow-blue-500/25 flex items-center justify-center group">
                        <i class="fas fa-search mr-2 group-hover:scale-110 transition-transform"></i>
                        {"Start Domain Search"}
                    </button>
                </div>
            </div>
        },
        "Google" => html! {
            <div class="bg-gradient-to-br from-slate-800/50 via-green-900/30 to-slate-800/50 backdrop-blur-xl border border-green-500/20 rounded-2xl p-8 hover:border-green-400/40 transition-all duration-300 hover:shadow-2xl hover:shadow-green-500/10 group">
                <div class="flex items-center mb-6">
                    <div class="w-12 h-12 bg-gradient-to-r from-green-500 to-emerald-500 rounded-xl flex items-center justify-center mr-4 group-hover:scale-110 transition-transform duration-300">
                        <i class="fab fa-google text-white text-xl"></i>
                    </div>
                    <div>
                        <h3 class="text-2xl font-bold text-white mb-1">{"Google Search"}</h3>
                        <p class="text-green-300/80">{"Find emails through Google search results"}</p>
                    </div>
                </div>

                <div class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-300 mb-2">
                            {"Search Keywords"}
                            <span class="text-red-400 ml-1">{"*"}</span>
                        </label>
                        <div class="relative">
                            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                <i class="fas fa-tags text-green-400"></i>
                            </div>
                            <input
                                type="text"
                                class="block w-full pl-10 pr-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-green-500/50 focus:border-green-500/50 transition-all duration-200 hover:bg-white/10"
                                placeholder="startup, founder, CEO (comma separated)"
                            />
                        </div>
                        <p class="text-xs text-gray-400 mt-2">{"Separate multiple keywords with commas for better results"}</p>
                    </div>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-300 mb-2">{"Results per keyword"}</label>
                            <select class="block w-full py-2.5 px-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-green-500/50">
                                <option value="10">{"10 results"}</option>
                                <option value="25" selected={true}>{"25 results"}</option>
                                <option value="50">{"50 results"}</option>
                                <option value="100">{"100 results"}</option>
                            </select>
                        </div>
                        <div>
                            <label class="block text-sm font-medium text-gray-300 mb-2">{"Search region"}</label>
                            <select class="block w-full py-2.5 px-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-green-500/50">
                                <option value="global" selected={true}>{"Global"}</option>
                                <option value="us">{"United States"}</option>
                                <option value="uk">{"United Kingdom"}</option>
                                <option value="ca">{"Canada"}</option>
                            </select>
                        </div>
                    </div>

                    <button class="w-full mt-6 bg-gradient-to-r from-green-600 to-emerald-600 hover:from-green-500 hover:to-emerald-500 text-white font-semibold py-3 px-6 rounded-xl transition-all duration-200 hover:scale-[1.02] active:scale-[0.98] hover:shadow-lg hover:shadow-green-500/25 flex items-center justify-center group">
                        <i class="fab fa-google mr-2 group-hover:scale-110 transition-transform"></i>
                        {"Start Google Search"}
                    </button>
                </div>
            </div>
        },
        _ => html! {
            <div class="bg-gradient-to-br from-slate-800/50 via-purple-900/30 to-slate-800/50 backdrop-blur-xl border border-purple-500/20 rounded-2xl p-8">
                <div class="text-center text-gray-400">
                    <i class="fas fa-search text-4xl mb-4"></i>
                    <p>{"Select a search type to get started"}</p>
                </div>
            </div>
        }
    }
}