use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub is_open: bool,
    pub on_toggle: Callback<()>,
}

#[function_component]
pub fn Sidebar(props: &SidebarProps) -> Html {
    html! {
        <div class={classes!("sidebar", if props.is_open { "is-open" } else { "" })}>
            // Header with modern design
            <div class="sidebar-header">
                <div style="display: flex; align-items: center;">
                    <div style="width: 42px; height: 42px; background: linear-gradient(135deg, #007bff 0%, #0056b3 100%); border-radius: 12px; display: flex; align-items: center; justify-content: center; margin-right: 12px; box-shadow: 0 4px 12px rgba(0, 123, 255, 0.25);">
                        <i class="fas fa-at" style="color: white; font-size: 20px;"></i>
                    </div>
                    <div>
                        <div style="font-weight: 700; font-size: 18px; color: white;">{"axonemail"}</div>
                        <div style="opacity: 0.9; font-size: 13px; color: rgba(255,255,255,0.8);">{"Professional Suite"}</div>
                    </div>
                </div>
            </div>

            // Main navigation content
            <div class="sidebar-content">
                // Search Section
                <div class="nav-section">
                    <div class="nav-section-title">{"Search & Discovery"}</div>
                    <button class="nav-item is-active">
                        <div class="nav-item-icon active">
                            <i class="fas fa-search"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Email Search"}</div>
                            <div class="nav-item-subtitle">{"Find contact emails"}</div>
                        </div>
                    </button>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-list-ul"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Saved Lists"}</div>
                            <div class="nav-item-subtitle">{"Your collections"}</div>
                        </div>
                        <div class="nav-item-badge">{"24"}</div>
                    </button>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-history"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Search History"}</div>
                            <div class="nav-item-subtitle">{"Recent searches"}</div>
                        </div>
                    </button>
                </div>

                // Tools Section
                <div class="nav-section">
                    <div class="nav-section-title">{"Email Tools"}</div>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-shield-check"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Email Verifier"}</div>
                            <div class="nav-item-subtitle">{"Validate email addresses"}</div>
                        </div>
                    </button>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-users"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Contact Manager"}</div>
                            <div class="nav-item-subtitle">{"Organize contacts"}</div>
                        </div>
                    </button>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-filter"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Data Cleaner"}</div>
                            <div class="nav-item-subtitle">{"Remove duplicates"}</div>
                        </div>
                    </button>
                </div>

                // Analytics Section
                <div class="nav-section">
                    <div class="nav-section-title">{"Analytics & Reports"}</div>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-chart-line"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Dashboard"}</div>
                            <div class="nav-item-subtitle">{"Usage overview"}</div>
                        </div>
                    </button>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-file-export"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Export Manager"}</div>
                            <div class="nav-item-subtitle">{"Download reports"}</div>
                        </div>
                    </button>
                </div>

                // API Section
                <div class="nav-section">
                    <div class="nav-section-title">{"Integration"}</div>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-code"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"API Access"}</div>
                            <div class="nav-item-subtitle">{"Developer tools"}</div>
                        </div>
                    </button>
                    <button class="nav-item">
                        <div class="nav-item-icon">
                            <i class="fas fa-puzzle-piece"></i>
                        </div>
                        <div class="nav-item-content">
                            <div class="nav-item-title">{"Integrations"}</div>
                            <div class="nav-item-subtitle">{"Connect apps"}</div>
                        </div>
                    </button>
                </div>
            </div>

            // Modern footer with plan info and quick actions
            <div class="sidebar-footer">
                <div class="plan-info">
                    <div class="plan-info-main">
                        <div style="display: flex; align-items: center; justify-content: space-between;">
                            <div>
                                <div class="plan-name">{"Pro Plan"}</div>
                                <div class="plan-usage">{"12,450 / 15,000 searches"}</div>
                            </div>
                            <div class="plan-progress-circle">
                                <span>{"83%"}</span>
                            </div>
                        </div>
                        <div class="plan-progress-bar">
                            <div class="plan-progress-fill" style="width: 83%;"></div>
                        </div>
                    </div>
                    <div class="footer-actions">
                        <button class="footer-btn">
                            <i class="fas fa-crown"></i>
                            {"Upgrade"}
                        </button>
                        <button class="footer-btn secondary">
                            <i class="fas fa-cog"></i>
                            {"Settings"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}