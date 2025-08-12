use yew::prelude::*;
use gloo_timers::callback::{Interval, Timeout};

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
    Paused,
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
    let iteration = use_state(|| 0u32);

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
                        
                        // Atualizar métricas
                        let emails_count = (iter * 2) as u32;
                        metrics_clone.set(SearchMetrics {
                            emails_found: emails_count,
                            verified_emails: ((iter as f32 * 1.5) as u32),
                            domains_searched: iter,
                            sources_used: std::cmp::min((iter / 3 + 1) as u32, 8),
                            search_progress: progress,
                            current_domain,
                            elapsed_time: format!("00:{:02}", iter),
                        });
                        
                        // Adicionar emails progressivamente
                        let end_index = std::cmp::min(emails_count as usize, sample_data_clone.len());
                        let current_results = sample_data_clone[0..end_index].to_vec();
                        results_clone.set(current_results);
                        
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

    html! {
        <div class="search-container">
            // Header section following app pattern
            <div class="search-header">
                <div style="display: flex; align-items: center; gap: 1rem;">
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
                    
                    <div>
                        <h1 class="search-title">{"Campanha de Busca de Emails"}</h1>
                        <p class="search-subtitle">
                            {"ID da Campanha: "}
                            <code style="background: var(--surface-alt); padding: 0.25rem 0.5rem; border-radius: 0.25rem; font-size: 0.75rem;">
                                {&props.search_id[..8]}{"..."}
                            </code>
                        </p>
                    </div>
                </div>
                
                <div style="display: flex; align-items: center; gap: 1rem;">
                    <div class={format!("status-indicator status-{}", 
                        match &*status {
                            SearchStatus::Initializing => "warning",
                            SearchStatus::Searching => "info", 
                            SearchStatus::Completed => "success",
                            SearchStatus::Paused => "danger",
                        }
                    )}>
                        <div class="status-dot"></div>
                        {match &*status {
                            SearchStatus::Initializing => "Inicializando",
                            SearchStatus::Searching => "Buscando",
                            SearchStatus::Completed => "Concluído", 
                            SearchStatus::Paused => "Pausado",
                        }}
                    </div>
                    
                    <div style="color: var(--text-muted); font-size: 0.875rem; font-family: monospace;">
                        <i class="fas fa-clock" style="margin-right: 0.5rem;"></i>
                        {&metrics.elapsed_time}
                    </div>
                </div>
            </div>

            <div class="search-content">
                // Metrics Cards Grid
                <div class="metrics-grid">
                    <div class="metric-card">
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
                            <span class="metric-label">{"Domínios"}</span>
                            <i class="fas fa-globe metric-icon"></i>
                        </div>
                        <div class="metric-value">{metrics.domains_searched}</div>
                        <div class="metric-subtitle">
                            {"Atual: "}
                            <code style="font-size: 0.75rem; background: var(--surface-alt); padding: 0.125rem 0.25rem; border-radius: 0.125rem;">
                                {&metrics.current_domain}
                            </code>
                        </div>
                    </div>

                    <div class="metric-card">
                        <div class="metric-header">
                            <span class="metric-label">{"Fontes"}</span>
                            <i class="fas fa-database metric-icon"></i>
                        </div>
                        <div class="metric-value">{metrics.sources_used}</div>
                        <div class="metric-subtitle">{"de 12 fontes de dados"}</div>
                    </div>

                    <div class="metric-card">
                        <div class="metric-header">
                            <span class="metric-label">{"Progresso"}</span>
                            <i class="fas fa-chart-line metric-icon"></i>
                        </div>
                        <div class="metric-value">{format!("{}%", metrics.search_progress)}</div>
                        <div class="progress-bar">
                            <div class="progress-fill" style={format!("width: {}%", metrics.search_progress)}></div>
                        </div>
                    </div>
                </div>

                // Results Table
                <div class="results-area">
                    <div class="results-header">
                        <div>
                            <h2 class="results-title">{"Resultados da Busca"}</h2>
                            <p style="color: var(--text-muted); margin: 0; font-size: 0.875rem;">
                                {"Emails descobertos em tempo real"}
                            </p>
                        </div>
                        
                        if !results.is_empty() {
                            <div class="results-actions">
                                <button class="btn btn-primary">
                                    <i class="fas fa-download"></i>
                                    {"Exportar CSV"}
                                </button>
                                <button class="btn btn-outline-secondary">
                                    <i class="fas fa-filter"></i>
                                    {"Filtrar"}
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
                                <div>{"Nome / Cargo"}</div>
                                <div>{"Empresa"}</div>
                                <div>{"Fonte"}</div>
                                <div>{"Confiança"}</div>
                                <div>{"Status"}</div>
                            </div>

                            // Table Body
                            <div class="table-body">
                                {results.iter().enumerate().map(|(i, result)| {
                                    html! {
                                        <div key={i} class="table-row">
                                            <div class="email-cell">
                                                <i class="fas fa-envelope" style="color: var(--primary); margin-right: 0.5rem;"></i>
                                                <span style="font-family: monospace; font-weight: 500;">{&result.email}</span>
                                            </div>
                                            <div class="name-cell">
                                                <div class="name">{&result.name}</div>
                                                <div class="title">{&result.title}</div>
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