use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EmailSearchProps {
    pub search_id: String,
    pub on_back: Callback<()>,
}

#[derive(Clone, PartialEq)]
pub enum SearchStatus {
    Initializing,
    Searching,
    Completed,
}

#[derive(Clone, PartialEq)]
pub struct EmailResult {
    pub email: String,
    pub name: String,
    pub title: String,
    pub company: String,
    pub source: String,
    pub confidence: u8,
    pub verified: bool,
}

#[derive(Clone, PartialEq)]
pub struct SearchMetrics {
    pub emails_found: u32,
    pub verified_emails: u32,
    pub domains_searched: u32,
    pub sources_used: u32,
    pub search_progress: u32,
    pub current_domain: String,
    pub elapsed_time: String,
}

#[function_component]
pub fn EmailSearch(props: &EmailSearchProps) -> Html {
    let status = use_state(|| SearchStatus::Initializing);
    let metrics = use_state(|| SearchMetrics {
        emails_found: 0,
        verified_emails: 0,
        domains_searched: 0,
        sources_used: 0,
        search_progress: 0,
        current_domain: "Preparando busca...".to_string(),
        elapsed_time: "00:00".to_string(),
    });
    let results = use_state(|| Vec::<EmailResult>::new());
    let filtered_results = use_state(|| Vec::<EmailResult>::new());
    let iteration = use_state(|| 0u32);
    let search_filter = use_state(|| String::new());
    let status_filter = use_state(|| "all".to_string());

    // Dados de exemplo pre-definidos
    let sample_data = vec![
        EmailResult { email: "sarah.johnson@google.com".to_string(), name: "Sarah Johnson".to_string(), title: "Software Engineer".to_string(), company: "Google".to_string(), source: "LinkedIn".to_string(), confidence: 89, verified: true },
        EmailResult { email: "michael.chen@microsoft.com".to_string(), name: "Michael Chen".to_string(), title: "Product Manager".to_string(), company: "Microsoft".to_string(), source: "Company Website".to_string(), confidence: 92, verified: false },
        EmailResult { email: "emily.davis@apple.com".to_string(), name: "Emily Davis".to_string(), title: "Marketing Director".to_string(), company: "Apple".to_string(), source: "GitHub".to_string(), confidence: 85, verified: true },
        EmailResult { email: "david.rodriguez@meta.com".to_string(), name: "David Rodriguez".to_string(), title: "Sales Manager".to_string(), company: "Meta".to_string(), source: "Hunter.io".to_string(), confidence: 95, verified: false },
        EmailResult { email: "jessica.wang@amazon.com".to_string(), name: "Jessica Wang".to_string(), title: "Data Scientist".to_string(), company: "Amazon".to_string(), source: "Apollo".to_string(), confidence: 88, verified: true },
        EmailResult { email: "robert.kim@netflix.com".to_string(), name: "Robert Kim".to_string(), title: "UX Designer".to_string(), company: "Netflix".to_string(), source: "ZoomInfo".to_string(), confidence: 91, verified: false },
        EmailResult { email: "amanda.thompson@spotify.com".to_string(), name: "Amanda Thompson".to_string(), title: "VP of Engineering".to_string(), company: "Spotify".to_string(), source: "Clearbit".to_string(), confidence: 87, verified: true },
        EmailResult { email: "daniel.garcia@uber.com".to_string(), name: "Daniel Garcia".to_string(), title: "Head of Marketing".to_string(), company: "Uber".to_string(), source: "EmailFinder".to_string(), confidence: 93, verified: false },
        EmailResult { email: "lauren.martinez@airbnb.com".to_string(), name: "Lauren Martinez".to_string(), title: "CTO".to_string(), company: "Airbnb".to_string(), source: "LinkedIn".to_string(), confidence: 96, verified: true },
        EmailResult { email: "christopher.lee@stripe.com".to_string(), name: "Christopher Lee".to_string(), title: "CMO".to_string(), company: "Stripe".to_string(), source: "Company Website".to_string(), confidence: 89, verified: false },
        EmailResult { email: "samantha.brown@shopify.com".to_string(), name: "Samantha Brown".to_string(), title: "CEO".to_string(), company: "Shopify".to_string(), source: "GitHub".to_string(), confidence: 94, verified: true },
        EmailResult { email: "kevin.wilson@adobe.com".to_string(), name: "Kevin Wilson".to_string(), title: "CFO".to_string(), company: "Adobe".to_string(), source: "Hunter.io".to_string(), confidence: 90, verified: false },
        EmailResult { email: "ashley.taylor@salesforce.com".to_string(), name: "Ashley Taylor".to_string(), title: "Lead Developer".to_string(), company: "Salesforce".to_string(), source: "Apollo".to_string(), confidence: 86, verified: true },
        EmailResult { email: "james.anderson@oracle.com".to_string(), name: "James Anderson".to_string(), title: "Senior Designer".to_string(), company: "Oracle".to_string(), source: "ZoomInfo".to_string(), confidence: 92, verified: false },
        EmailResult { email: "maria.gonzalez@ibm.com".to_string(), name: "Maria Gonzalez".to_string(), title: "Growth Manager".to_string(), company: "IBM".to_string(), source: "Clearbit".to_string(), confidence: 88, verified: true },
        EmailResult { email: "ryan.miller@tesla.com".to_string(), name: "Ryan Miller".to_string(), title: "Software Engineer".to_string(), company: "Tesla".to_string(), source: "EmailFinder".to_string(), confidence: 91, verified: false },
        EmailResult { email: "nicole.white@twitter.com".to_string(), name: "Nicole White".to_string(), title: "Product Manager".to_string(), company: "Twitter".to_string(), source: "LinkedIn".to_string(), confidence: 89, verified: true },
        EmailResult { email: "andrew.clark@linkedin.com".to_string(), name: "Andrew Clark".to_string(), title: "Marketing Director".to_string(), company: "LinkedIn".to_string(), source: "Company Website".to_string(), confidence: 95, verified: false },
        EmailResult { email: "rachel.lewis@paypal.com".to_string(), name: "Rachel Lewis".to_string(), title: "Sales Manager".to_string(), company: "PayPal".to_string(), source: "GitHub".to_string(), confidence: 87, verified: true },
        EmailResult { email: "matthew.hall@dropbox.com".to_string(), name: "Matthew Hall".to_string(), title: "Data Scientist".to_string(), company: "Dropbox".to_string(), source: "Hunter.io".to_string(), confidence: 93, verified: false },
        EmailResult { email: "jennifer.young@zoom.us".to_string(), name: "Jennifer Young".to_string(), title: "UX Designer".to_string(), company: "Zoom".to_string(), source: "Apollo".to_string(), confidence: 90, verified: true },
        EmailResult { email: "steven.walker@slack.com".to_string(), name: "Steven Walker".to_string(), title: "VP of Engineering".to_string(), company: "Slack".to_string(), source: "ZoomInfo".to_string(), confidence: 88, verified: false },
        EmailResult { email: "lisa.allen@discord.com".to_string(), name: "Lisa Allen".to_string(), title: "Head of Marketing".to_string(), company: "Discord".to_string(), source: "Clearbit".to_string(), confidence: 92, verified: true },
        EmailResult { email: "brian.king@reddit.com".to_string(), name: "Brian King".to_string(), title: "CTO".to_string(), company: "Reddit".to_string(), source: "EmailFinder".to_string(), confidence: 94, verified: false },
        EmailResult { email: "amy.wright@pinterest.com".to_string(), name: "Amy Wright".to_string(), title: "CMO".to_string(), company: "Pinterest".to_string(), source: "LinkedIn".to_string(), confidence: 91, verified: true },
        EmailResult { email: "contato@knxbrasil.com.br".to_string(), name: "".to_string(), title: "Contato Geral".to_string(), company: "KNX Brasil".to_string(), source: "Company Website".to_string(), confidence: 95, verified: true },
        EmailResult { email: "knxbrasil@knxbrasil.com.br".to_string(), name: "".to_string(), title: "Suporte Técnico".to_string(), company: "KNX Brasil".to_string(), source: "Company Website".to_string(), confidence: 94, verified: false },
    ];

    // Timer para simulação usando use_effect
    {
        let status = status.clone();
        let metrics = metrics.clone();
        let results = results.clone();
        let iteration = iteration.clone();

        use_effect_with((), move |_| {
            use gloo_timers::callback::Timeout;
            
            let domains = vec![
                "linkedin.com", "company.com", "github.com", "twitter.com",
                "crunchbase.com", "angel.co", "facebook.com", "instagram.com",
                "medium.com", "dev.to", "stackoverflow.com", "dribbble.com"
            ];
            let sample_data_clone = sample_data.clone();
            
            // Função recursiva para continuar o timer
            fn run_simulation(
                iter: u32, 
                status: UseStateHandle<SearchStatus>,
                metrics: UseStateHandle<SearchMetrics>,
                results: UseStateHandle<Vec<EmailResult>>,
                iteration: UseStateHandle<u32>,
                domains: Vec<&'static str>,
                sample_data: Vec<EmailResult>
            ) {
                if iter <= 25 {
                    let status_clone = status.clone();
                    let metrics_clone = metrics.clone();
                    let results_clone = results.clone();
                    let iteration_clone = iteration.clone();
                    let domains_clone = domains.clone();
                    let sample_data_clone = sample_data.clone();
                    
                    let handle = Timeout::new(1000, move || {
                        iteration_clone.set(iter);
                        
                        if iter == 1 {
                            status_clone.set(SearchStatus::Searching);
                        }
                        
                        let progress = std::cmp::min((iter * 4) as u32, 100);
                        let current_domain = domains_clone[(iter as usize - 1) % domains_clone.len()].to_string();
                        
                        // Adicionar emails progressivamente
                        let end_index = std::cmp::min((iter * 2) as usize, sample_data_clone.len());
                        let current_results = sample_data_clone[0..end_index].to_vec();
                        results_clone.set(current_results.clone());
                        
                        // Atualizar métricas com base nos resultados reais
                        let emails_count = current_results.len() as u32;
                        metrics_clone.set(SearchMetrics {
                            emails_found: emails_count,
                            verified_emails: current_results.iter().filter(|r| r.verified).count() as u32,
                            domains_searched: iter,
                            sources_used: std::cmp::min((iter / 3 + 1) as u32, 8),
                            search_progress: progress,
                            current_domain,
                            elapsed_time: format!("00:{:02}", iter),
                        });
                        
                        if iter == 25 {
                            status_clone.set(SearchStatus::Completed);
                        } else {
                            // Recursivamente agendar próxima execução
                            run_simulation(
                                iter + 1,
                                status_clone,
                                metrics_clone,
                                results_clone,
                                iteration_clone,
                                domains_clone,
                                sample_data_clone
                            );
                        }
                    });
                    
                    // Prevenimos o handle de ser dropado muito cedo
                    handle.forget();
                }
            }
            
            // Começar após 1 segundo
            run_simulation(1, status, metrics, results, iteration, domains, sample_data_clone);
            
            || {}
        });
    }
    
    // Lógica de filtro
    {
        let filtered_results_clone = filtered_results.clone();
        
        use_effect_with((results.clone(), search_filter.clone(), status_filter.clone()), move |(results, search_filter, status_filter)| {
            let mut filtered = results.iter().cloned().collect::<Vec<_>>();
            
            // Filtro por texto
            if !search_filter.is_empty() {
                let search_lower = search_filter.to_lowercase();
                filtered.retain(|email| {
                    email.email.to_lowercase().contains(&search_lower) ||
                    email.name.to_lowercase().contains(&search_lower) ||
                    email.company.to_lowercase().contains(&search_lower) ||
                    email.title.to_lowercase().contains(&search_lower)
                });
            }
            
            // Filtro por status
            if status_filter.as_str() != "all" {
                match status_filter.as_str() {
                    "verified" => filtered.retain(|email| email.verified),
                    "pending" => filtered.retain(|email| !email.verified),
                    _ => {}
                }
            }
            
            filtered_results_clone.set(filtered);
            || {}
        });
    }

    let on_search_change = {
        let search_filter = search_filter.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            search_filter.set(input.value());
        })
    };
    
    let on_status_filter_change = {
        let status_filter = status_filter.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            status_filter.set(select.value());
        })
    };

    html! {
        <div class="search-container">
            // Header compacto com barra de progresso integrada
            <div class="search-header">
                <div class="header-main">
                    <div class="header-left">
                        <button 
                            class="btn btn-outline-secondary"
                            onclick={
                                let on_back = props.on_back.clone();
                                Callback::from(move |_| on_back.emit(()))
                            }
                        >
                            <i class="fas fa-arrow-left"></i>
                            {"Voltar"}
                        </button>
                        
                        <div class="title-section">
                            <h1 class="search-title">{"Busca de Emails"}</h1>
                            <span class="campaign-id">
                                {"ID: "}{&props.search_id[..8]}{"..."}
                            </span>
                        </div>
                    </div>
                    
                    <div class="header-right">
                        <div class={format!("status-badge status-{}", 
                            match &*status {
                                SearchStatus::Initializing => "warning",
                                SearchStatus::Searching => "info", 
                                SearchStatus::Completed => "success",
                            }
                        )}>
                            <div class="status-dot"></div>
                            {match &*status {
                                SearchStatus::Initializing => "Inicializando",
                                SearchStatus::Searching => "Buscando",
                                SearchStatus::Completed => "Concluído", 
                            }}
                        </div>
                        
                        <div class="elapsed-time">
                            <i class="fas fa-clock"></i>
                            {&metrics.elapsed_time}
                        </div>
                        
                        <div class="progress-text">
                            {format!("{}%", metrics.search_progress)}
                        </div>
                    </div>
                </div>
                
                // Barra de progresso integrada
                <div class="progress-bar-integrated">
                    <div class="progress-fill" style={format!("width: {}%", metrics.search_progress)}></div>
                </div>
            </div>

            <div class="search-content">
                // Metrics Cards - apenas os essenciais
                <div class="metrics-row">
                    <div class="metric-card primary">
                        <div class="metric-header">
                            <span class="metric-label">{"Emails Encontrados"}</span>
                            <i class="fas fa-envelope metric-icon"></i>
                        </div>
                        <div class="metric-value">{metrics.emails_found}</div>
                        <div class="metric-subtitle">
                            <i class="fas fa-check-circle" style="color: var(--success); margin-right: 0.25rem;"></i>
                            {format!("{} verificados", metrics.verified_emails)}
                        </div>
                    </div>

                    <div class="metric-card">
                        <div class="metric-header">
                            <span class="metric-label">{"Domínios Pesquisados"}</span>
                            <i class="fas fa-globe metric-icon"></i>
                        </div>
                        <div class="metric-value">{metrics.domains_searched}</div>
                        <div class="metric-subtitle">
                            {"Atual: "}
                            <span class="current-domain">{&metrics.current_domain}</span>
                        </div>
                    </div>

                    <div class="metric-card accent">
                        <div class="metric-header">
                            <span class="metric-label">{"Status da Busca"}</span>
                            <i class="fas fa-chart-pulse metric-icon"></i>
                        </div>
                        <div class="metric-value">{format!("{}min", (metrics.elapsed_time.split(':').nth(1).unwrap_or("0").parse::<i32>().unwrap_or(0) * 100 / 60) / 100)}</div>
                        <div class="metric-subtitle">
                            {"Tempo decorrido"}
                        </div>
                    </div>
                </div>

                // Results Table
                <div class="results-area">
                    <div class="results-toolbar">
                        <div class="toolbar-left">
                            <h2 class="results-title">{"Resultados"}</h2>
                            <span class="results-count">
                                {format!("{} emails encontrados", if filtered_results.is_empty() { results.len() } else { filtered_results.len() })}
                            </span>
                        </div>
                        
                        if !results.is_empty() {
                            <div class="toolbar-right">
                                <div class="search-box">
                                    <i class="fas fa-search"></i>
                                    <input 
                                        type="text" 
                                        placeholder="Buscar emails..." 
                                        class="search-input"
                                        oninput={on_search_change}
                                        value={(*search_filter).clone()}
                                    />
                                </div>
                                
                                <select class="filter-select" onchange={on_status_filter_change} value={(*status_filter).clone()}>
                                    <option value="all">{"Todos"}</option>
                                    <option value="verified">{"Verificados"}</option>
                                    <option value="pending">{"Pendentes"}</option>
                                </select>
                                
                                <button class="btn btn-outline-secondary btn-sm">
                                    <i class="fas fa-download"></i>
                                    {"CSV"}
                                </button>
                            </div>
                        }
                    </div>

                    if results.is_empty() {
                        <div class="empty-state">
                            <i class="fas fa-search empty-icon"></i>
                            <h3 class="empty-title">{"Descobrindo emails..."}</h3>
                            <p class="empty-subtitle">{"Os resultados aparecerão aqui conforme forem encontrados"}</p>
                        </div>
                    } else {
                        <div class="results-table">
                            // Table Header
                            <div class="table-header">
                                <div>{"Email"}</div>
                                <div>{"Contato / Informações"}</div>
                                <div>{"Empresa"}</div>
                                <div>{"Fonte"}</div>
                                <div>{"Confiança"}</div>
                                <div>{"Status"}</div>
                            </div>

                            // Table Body
                            <div class="table-body">
                                {(if filtered_results.is_empty() && search_filter.is_empty() && status_filter.as_str() == "all" {
                                    results.iter().collect::<Vec<_>>()
                                } else {
                                    filtered_results.iter().collect::<Vec<_>>()
                                }).iter().enumerate().map(|(i, result)| {
                                    html! {
                                        <div key={i} class="table-row">
                                            <div class="email-cell">
                                                <i class="fas fa-envelope" style="color: var(--primary); margin-right: 0.5rem;"></i>
                                                <span style="font-family: monospace; font-weight: 500;">{&result.email}</span>
                                            </div>
                                            <div class="name-cell">
                                                {&result.title}
                                                if !result.name.is_empty() {
                                                    {" - "}{&result.name}
                                                }
                                            </div>
                                            <div class="company-cell">{&result.company}</div>
                                            <div class="source-cell">
                                                <span class="source-badge">{&result.source}</span>
                                            </div>
                                            <div class="confidence-cell">
                                                <span class={format!("confidence-score confidence-{}", 
                                                    if result.confidence >= 90 { "high" }
                                                    else if result.confidence >= 75 { "medium" }
                                                    else { "low" }
                                                )}>
                                                    {format!("{}%", result.confidence)}
                                                </span>
                                            </div>
                                            <div class="status-cell">
                                                if result.verified {
                                                    <span class="status-verified">
                                                        <i class="fas fa-check-circle"></i>
                                                        {"Verificado"}
                                                    </span>
                                                } else {
                                                    <span class="status-pending">
                                                        <i class="fas fa-clock"></i>
                                                        {"Pendente"}
                                                    </span>
                                                }
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()}
                            </div>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}