use yew::prelude::*;

#[function_component]
pub fn SearchInterface() -> Html {
    let search_query = use_state(|| String::new());
    let show_advanced = use_state(|| false);
    
    // Search category state
    let search_category = use_state(|| String::new());
    let ai_model = use_state(|| "gemini".to_string());
    let ollama_model = use_state(|| "deepseek_r1_1_5b".to_string());
    
    // Bulk business category options
    let business_category = use_state(|| String::new());
    let business_category_other = use_state(|| String::new());
    let company_size = use_state(|| String::new());
    let job_roles = use_state(|| String::new());
    let location_filter = use_state(|| String::new());
    
    // Specific company options
    let company_name = use_state(|| String::new());
    let company_domain = use_state(|| String::new());
    let department_filter = use_state(|| String::new());
    let employee_count = use_state(|| String::new());
    
    // Domain emails options
    let website_domain = use_state(|| String::new());
    let subdomain_include = use_state(|| false);
    
    let results_limit = use_state(|| 100);
    let search_mode = use_state(|| "site_count".to_string());

    let toggle_advanced = {
        let show_advanced = show_advanced.clone();
        Callback::from(move |_| show_advanced.set(!*show_advanced))
    };

    let close_advanced = {
        let show_advanced = show_advanced.clone();
        Callback::from(move |_| show_advanced.set(false))
    };

    let on_search_query_change = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            search_query.set(input.value());
        })
    };

    html! {
        <>
            // Main search interface - Google-like
            <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 60vh; padding: 2rem;">
                // Logo/Title area
                <div style="text-align: center; margin-bottom: 2rem;">
                    <h1 style="font-size: 4rem; font-weight: 300; color: var(--text); margin: 0; letter-spacing: -2px;">
                        {"axon"}<span style="color: var(--primary);">{"email"}</span>
                    </h1>
                </div>

                // Search bar
                <div style="width: 100%; max-width: 600px; position: relative;">
                    <div style="display: flex; align-items: center; background: var(--surface); border: 1px solid var(--border); border-radius: 50px; padding: 12px 20px; box-shadow: 0 4px 20px var(--shadow); transition: all 0.2s ease; hover: box-shadow: 0 6px 25px var(--shadow);">
                        <i class="fas fa-search" style="color: var(--text-muted); margin-right: 12px; font-size: 1.1rem;"></i>
                        
                        <input 
                            type="text"
                            placeholder="Find email addresses..."
                            value={(*search_query).clone()}
                            oninput={on_search_query_change}
                            style="flex: 1; border: none; background: transparent; font-size: 1.1rem; color: var(--text); outline: none;"
                        />
                        
                        <i class="fas fa-microphone" style="color: var(--text-muted); margin-left: 12px; font-size: 1.1rem; cursor: pointer; padding: 8px; border-radius: 50%; hover: background-color: var(--surface-alt);"></i>
                    </div>
                </div>

                // Buttons
                <div style="display: flex; gap: 1rem; margin-top: 2rem; flex-wrap: wrap; justify-content: center;">
                    <button 
                        onclick={Callback::from(move |_| {})}
                        disabled={true}
                        style="background: var(--surface); color: var(--text-muted); border: 1px solid var(--border); padding: 10px 20px; border-radius: 4px; font-size: 14px; cursor: not-allowed; opacity: 0.6;"
                    >
                        {"Email Search"}
                    </button>
                    
                    <button 
                        onclick={toggle_advanced}
                        style="background: var(--surface); color: var(--text); border: 1px solid var(--border); padding: 10px 20px; border-radius: 4px; font-size: 14px; cursor: pointer; transition: all 0.2s ease; hover: border-color: var(--primary); hover: box-shadow: 0 2px 8px var(--shadow);"
                    >
                        {"Advanced Options"}
                    </button>
                </div>
            </div>

            // Advanced options modal
            if *show_advanced {
                <>
                    <div 
                        style="position: fixed; inset: 0; background: rgba(0,0,0,0.5); z-index: 1000; display: flex; align-items: center; justify-content: center; padding: 2rem;"
                        onclick={close_advanced.clone()}
                    ></div>
                    
                    <div style="position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background: var(--surface); border: 1px solid var(--border); border-radius: 12px; padding: 2rem; width: 90%; max-width: 700px; max-height: 80vh; overflow-y: auto; z-index: 1001; box-shadow: 0 20px 60px var(--shadow);">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; padding-bottom: 1rem; border-bottom: 1px solid var(--border);">
                            <h2 style="margin: 0; font-size: 1.5rem; font-weight: 600; color: var(--text);">{"Advanced Search Options"}</h2>
                            <button 
                                onclick={close_advanced.clone()}
                                style="background: none; border: none; font-size: 1.5rem; color: var(--text-muted); cursor: pointer; padding: 4px; hover: color: var(--text);"
                            >
                                {"×"}
                            </button>
                        </div>

                        // AI Model Selection
                        <div style="margin-bottom: 2rem; padding: 1rem; background: var(--surface-alt); border-radius: 0.5rem;">
                            <label style="display: block; margin-bottom: 0.5rem; font-weight: 600; color: var(--text); font-size: 0.875rem;">{"AI Model Selection"}</label>
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                <select 
                                    value={(*ai_model).clone()}
                                    onchange={
                                        let ai_model = ai_model.clone();
                                        Callback::from(move |e: Event| {
                                            let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                            ai_model.set(select.value());
                                        })
                                    }
                                    style="padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 1rem;"
                                >
                                    <option value="gemini">{"Gemini (Cloud)"}</option>
                                    <option value="ollama">{"Ollama (Local)"}</option>
                                    <option value="openai">{"OpenAI GPT"}</option>
                                    <option value="claude">{"Claude"}</option>
                                </select>
                                
                                if *ai_model == "ollama" {
                                    <select 
                                        value={(*ollama_model).clone()}
                                        onchange={
                                            let ollama_model = ollama_model.clone();
                                            Callback::from(move |e: Event| {
                                                let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                ollama_model.set(select.value());
                                            })
                                        }
                                        style="padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 1rem;"
                                    >
                                        <option value="deepseek_r1_1_5b">{"Deepseek R1 1.5B"}</option>
                                        <option value="qwen_2_5_1_5b">{"Qwen 2.5 1.5B"}</option>
                                        <option value="deepseek_r1_8b">{"Deepseek R1 8B"}</option>
                                        <option value="phi4_14b">{"Phi4 14B"}</option>
                                    </select>
                                } else {
                                    <div></div>
                                }
                            </div>
                        </div>

                        <div style="margin-bottom: 2rem;">
                            <label style="display: block; margin-bottom: 1rem; font-weight: 600; color: var(--text); font-size: 1rem;">{"Select search type:"}</label>
                            <div style="display: grid; gap: 1rem;">
                                <label style="display: flex; align-items: center; padding: 1rem; border: 1px solid var(--border); border-radius: 0.5rem; cursor: pointer; background: var(--surface); transition: all 0.2s ease; hover: border-color: var(--primary);">
                                    <input 
                                        type="radio" 
                                        name="search_category" 
                                        value="bulk_business"
                                        checked={*search_category == "bulk_business"}
                                        onchange={
                                            let search_category = search_category.clone();
                                            Callback::from(move |_| search_category.set("bulk_business".to_string()))
                                        }
                                        style="margin-right: 0.75rem; accent-color: var(--primary);"
                                    />
                                    <div>
                                        <div style="font-weight: 500; color: var(--text);">{"Bulk emails by business category"}</div>
                                        <div style="font-size: 0.875rem; color: var(--text-muted); margin-top: 0.25rem;">{"Find emails from multiple companies in a specific industry"}</div>
                                    </div>
                                </label>

                                <label style="display: flex; align-items: center; padding: 1rem; border: 1px solid var(--border); border-radius: 0.5rem; cursor: pointer; background: var(--surface); transition: all 0.2s ease; hover: border-color: var(--primary);">
                                    <input 
                                        type="radio" 
                                        name="search_category" 
                                        value="specific_company"
                                        checked={*search_category == "specific_company"}
                                        onchange={
                                            let search_category = search_category.clone();
                                            Callback::from(move |_| search_category.set("specific_company".to_string()))
                                        }
                                        style="margin-right: 0.75rem; accent-color: var(--primary);"
                                    />
                                    <div>
                                        <div style="font-weight: 500; color: var(--text);">{"Specific company emails"}</div>
                                        <div style="font-size: 0.875rem; color: var(--text-muted); margin-top: 0.25rem;">{"Target a specific company and find all their email addresses"}</div>
                                    </div>
                                </label>

                                <label style="display: flex; align-items: center; padding: 1rem; border: 1px solid var(--border); border-radius: 0.5rem; cursor: pointer; background: var(--surface); transition: all 0.2s ease; hover: border-color: var(--primary);">
                                    <input 
                                        type="radio" 
                                        name="search_category" 
                                        value="domain_emails"
                                        checked={*search_category == "domain_emails"}
                                        onchange={
                                            let search_category = search_category.clone();
                                            Callback::from(move |_| search_category.set("domain_emails".to_string()))
                                        }
                                        style="margin-right: 0.75rem; accent-color: var(--primary);"
                                    />
                                    <div>
                                        <div style="font-weight: 500; color: var(--text);">{"All emails from single website"}</div>
                                        <div style="font-size: 0.875rem; color: var(--text-muted); margin-top: 0.25rem;">{"Extract all email addresses from a specific domain/website"}</div>
                                    </div>
                                </label>

                            </div>
                        </div>

                        // Dynamic options based on selected category
                        if *search_category == "bulk_business" {
                            <div style="padding: 1rem; background: var(--surface-alt); border-radius: 0.5rem; margin-bottom: 1.5rem;">
                                <h3 style="margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: var(--text);">{"Bulk Business Search Options"}</h3>
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;">
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Business Category"}</label>
                                        <select 
                                            value={(*business_category).clone()}
                                            onchange={
                                                let business_category = business_category.clone();
                                                Callback::from(move |e: Event| {
                                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                    business_category.set(select.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        >
                                            <option value="">{"Select category"}</option>
                                            <option value="technology">{"Technology"}</option>
                                            <option value="finance">{"Finance & Banking"}</option>
                                            <option value="healthcare">{"Healthcare"}</option>
                                            <option value="retail">{"Retail & E-commerce"}</option>
                                            <option value="manufacturing">{"Manufacturing"}</option>
                                            <option value="consulting">{"Consulting"}</option>
                                            <option value="education">{"Education"}</option>
                                            <option value="real_estate">{"Real Estate"}</option>
                                            <option value="legal">{"Legal Services"}</option>
                                            <option value="marketing">{"Marketing & Advertising"}</option>
                                            <option value="construction">{"Construction"}</option>
                                            <option value="hospitality">{"Hospitality & Travel"}</option>
                                            <option value="other">{"Other"}</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Company Size (Optional)"}</label>
                                        <select 
                                            value={(*company_size).clone()}
                                            onchange={
                                                let company_size = company_size.clone();
                                                Callback::from(move |e: Event| {
                                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                    company_size.set(select.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        >
                                            <option value="">{"Any size"}</option>
                                            <option value="startup">{"Startup (1-10)"}</option>
                                            <option value="small">{"Small (11-50)"}</option>
                                            <option value="medium">{"Medium (51-200)"}</option>
                                            <option value="large">{"Large (201-1000)"}</option>
                                            <option value="enterprise">{"Enterprise (1000+)"}</option>
                                        </select>
                                    </div>
                                </div>
                                
                                if *business_category == "other" {
                                    <div style="margin-bottom: 1rem;">
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Specify Business Category"}</label>
                                        <input 
                                            type="text"
                                            placeholder="Enter custom business category"
                                            value={(*business_category_other).clone()}
                                            oninput={
                                                let business_category_other = business_category_other.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    business_category_other.set(input.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        />
                                    </div>
                                }
                                
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Job Roles/Titles (Optional)"}</label>
                                        <input 
                                            type="text"
                                            placeholder="CEO, CTO, Manager"
                                            value={(*job_roles).clone()}
                                            oninput={
                                                let job_roles = job_roles.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    job_roles.set(input.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        />
                                    </div>
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Location (Optional)"}</label>
                                        <input 
                                            type="text"
                                            placeholder="San Francisco, CA"
                                            value={(*location_filter).clone()}
                                            oninput={
                                                let location_filter = location_filter.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    location_filter.set(input.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        />
                                    </div>
                                </div>
                            </div>
                        } else if *search_category == "specific_company" {
                            <div style="padding: 1rem; background: var(--surface-alt); border-radius: 0.5rem; margin-bottom: 1.5rem;">
                                <h3 style="margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: var(--text);">{"Specific Company Search Options"}</h3>
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;">
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Company Name"}</label>
                                        <input 
                                            type="text"
                                            placeholder="Apple, Google, Microsoft"
                                            value={(*company_name).clone()}
                                            oninput={
                                                let company_name = company_name.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    company_name.set(input.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        />
                                    </div>
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Company Domain (Optional)"}</label>
                                        <input 
                                            type="text"
                                            placeholder="apple.com"
                                            value={(*company_domain).clone()}
                                            oninput={
                                                let company_domain = company_domain.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    company_domain.set(input.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        />
                                    </div>
                                </div>
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Department Filter (Optional)"}</label>
                                        <select 
                                            value={(*department_filter).clone()}
                                            onchange={
                                                let department_filter = department_filter.clone();
                                                Callback::from(move |e: Event| {
                                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                    department_filter.set(select.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        >
                                            <option value="">{"All departments"}</option>
                                            <option value="engineering">{"Engineering"}</option>
                                            <option value="sales">{"Sales"}</option>
                                            <option value="marketing">{"Marketing"}</option>
                                            <option value="hr">{"Human Resources"}</option>
                                            <option value="finance">{"Finance"}</option>
                                            <option value="operations">{"Operations"}</option>
                                            <option value="executive">{"Executive"}</option>
                                            <option value="support">{"Support"}</option>
                                            <option value="legal">{"Legal"}</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Employee Count (Optional)"}</label>
                                        <select 
                                            value={(*employee_count).clone()}
                                            onchange={
                                                let employee_count = employee_count.clone();
                                                Callback::from(move |e: Event| {
                                                    let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                    employee_count.set(select.value());
                                                })
                                            }
                                            style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                        >
                                            <option value="">{"Any count"}</option>
                                            <option value="1-10">{"1-10 employees"}</option>
                                            <option value="11-50">{"11-50 employees"}</option>
                                            <option value="51-200">{"51-200 employees"}</option>
                                            <option value="201-1000">{"201-1000 employees"}</option>
                                            <option value="1000+">{"1000+ employees"}</option>
                                        </select>
                                    </div>
                                </div>
                            </div>
                        } else if *search_category == "domain_emails" {
                            <div style="padding: 1rem; background: var(--surface-alt); border-radius: 0.5rem; margin-bottom: 1.5rem;">
                                <h3 style="margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: var(--text);">{"Domain Email Extraction Options"}</h3>
                                <div style="margin-bottom: 1rem;">
                                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Website Domain"}</label>
                                    <input 
                                        type="text"
                                        placeholder="company.com"
                                        value={(*website_domain).clone()}
                                        oninput={
                                            let website_domain = website_domain.clone();
                                            Callback::from(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                website_domain.set(input.value());
                                            })
                                        }
                                        style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                    />
                                </div>
                                <div>
                                    <label style="display: flex; align-items: center; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">
                                        <input 
                                            type="checkbox"
                                            checked={*subdomain_include}
                                            onchange={
                                                let subdomain_include = subdomain_include.clone();
                                                Callback::from(move |e: Event| {
                                                    let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                    subdomain_include.set(input.checked());
                                                })
                                            }
                                            style="margin-right: 0.5rem; accent-color: var(--primary);"
                                        />
                                        {"Include subdomains (blog.company.com, support.company.com, etc.)"}
                                    </label>
                                </div>
                            </div>
                        }

                        // General Settings
                        <div style="padding: 1rem; background: var(--surface); border: 1px solid var(--border); border-radius: 0.5rem; margin-bottom: 1.5rem;">
                            <h3 style="margin: 0 0 1rem 0; font-size: 1rem; font-weight: 600; color: var(--text);">{"General Settings"}</h3>
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
                                <div>
                                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Results Limit"}</label>
                                    <select 
                                        value={(*results_limit).to_string()}
                                        onchange={
                                            let results_limit = results_limit.clone();
                                            Callback::from(move |e: Event| {
                                                let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                if let Ok(limit) = select.value().parse::<i32>() {
                                                    results_limit.set(limit);
                                                }
                                            })
                                        }
                                        style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                    >
                                        <option value="25">{"25 results"}</option>
                                        <option value="50">{"50 results"}</option>
                                        <option value="100">{"100 results"}</option>
                                        <option value="250">{"250 results"}</option>
                                        <option value="500">{"500 results"}</option>
                                        <option value="1000">{"1,000 results"}</option>
                                        <option value="2500">{"2,500 results"}</option>
                                        <option value="5000">{"5,000 results"}</option>
                                    </select>
                                </div>
                                <div>
                                    <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; color: var(--text); font-size: 0.875rem;">{"Search Mode"}</label>
                                    <select 
                                        value={(*search_mode).clone()}
                                        onchange={
                                            let search_mode = search_mode.clone();
                                            Callback::from(move |e: Event| {
                                                let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                search_mode.set(select.value());
                                            })
                                        }
                                        style="width: 100%; padding: 0.75rem 1rem; border: 1px solid var(--border); border-radius: 0.5rem; background: var(--surface); color: var(--text); font-size: 0.875rem;"
                                    >
                                        <option value="site_count">{"Site Count Mode - How many sites to verify from found/given site"}</option>
                                        <option value="time_limit">{"Time Limit Mode - How long to verify each site and next ones"}</option>
                                        <option value="email_target">{"Email Target Mode - How many emails to find before stopping"}</option>
                                    </select>
                                </div>
                            </div>
                        </div>

                        <div style="display: flex; justify-content: flex-end; gap: 1rem; padding-top: 1rem; border-top: 1px solid var(--border);">
                            <button 
                                onclick={close_advanced.clone()}
                                style="background: var(--surface-alt); color: var(--text); border: 1px solid var(--border); padding: 10px 20px; border-radius: 6px; cursor: pointer; hover: background: var(--surface);"
                            >
                                {"Cancel"}
                            </button>
                            <button 
                                onclick={close_advanced.clone()}
                                style="background: var(--primary); color: white; border: 1px solid var(--primary); padding: 10px 20px; border-radius: 6px; cursor: pointer; hover: background: var(--primary-hover);"
                            >
                                {"Search"}
                            </button>
                        </div>
                    </div>
                </>
            }
        </>
    }
}