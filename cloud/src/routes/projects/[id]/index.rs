use crate::components::{self as ui, icon, BadgeVariant, ButtonSize, ButtonVariant, StatusVariant};
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, Req, Res};

#[derive(FromRow)]
struct Project {
    name: String,
    github_repo: String,
    fly_app_name: Option<String>,
    created_at: String,
}

#[derive(FromRow)]
struct Deployment {
    id: String,
    git_sha: String,
    git_branch: String,
    git_message: Option<String>,
    pr_number: Option<i64>,
    status: String,
    #[allow(dead_code)]
    url: Option<String>,
    started_at: String,
    #[allow(dead_code)]
    finished_at: Option<String>,
}

#[derive(FromRow)]
struct ActiveDomain {
    hostname: String,
    status: String,
}

#[derive(FromRow)]
struct PreviewEnvironment {
    pr_number: i64,
    pr_branch: String,
    #[allow(dead_code)]
    fly_app_name: String,
    url: Option<String>,
    #[allow(dead_code)]
    status: String,
}

pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    let project = query_as::<_, Project>(
        r#"
        SELECT p.name, p.github_repo, p.fly_app_name, p.created_at
        FROM projects p
        JOIN users u ON p.user_id = u.id
        WHERE p.id = ? AND u.github_id = ?
        "#,
    )
    .bind(&id)
    .bind(github_id)
    .fetch_optional(&state.db)
    .await;

    let project = match project {
        Ok(Some(p)) => p,
        Ok(None) => return res.not_found("Project not found"),
        Err(_) => return res.internal_error("Database error"),
    };

    let is_deployed = project.fly_app_name.is_some();

    // Fetch active custom domains
    let custom_domains = query_as::<_, ActiveDomain>(
        r#"
        SELECT hostname, status
        FROM custom_domains
        WHERE project_id = ?
        ORDER BY created_at ASC
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // Fetch recent deployments
    let deployments = query_as::<_, Deployment>(
        r#"
        SELECT id, git_sha, git_branch, git_message, pr_number, status, url, started_at, finished_at
        FROM deployments
        WHERE project_id = ?
        ORDER BY started_at DESC
        LIMIT 10
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // Fetch active preview environments
    let preview_environments = query_as::<_, PreviewEnvironment>(
        r#"
        SELECT pr_number, pr_branch, fly_app_name, url, status
        FROM preview_environments
        WHERE project_id = ? AND status = 'active'
        ORDER BY created_at DESC
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    res.html(html! {
        div class="max-w-4xl mx-auto px-6 py-10" {
            // Back link
            (ui::back_link("/projects", "Projects"))
            
            // Header section
            div class="mt-6 flex items-start justify-between gap-4" {
                div class="flex items-start gap-4" {
                    // Project icon
                    div class="flex-shrink-0 w-14 h-14 rounded-xl bg-gradient-to-br from-[var(--bg-surface)] to-[var(--bg-hover)] border border-[var(--border-default)] flex items-center justify-center shadow-sm" {
                        span class="text-2xl" { "◈" }
                    }
                    
                    div {
                        // Name with status
                        div class="flex items-center gap-3" {
                            h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { (&project.name) }
                            @if is_deployed {
                                (ui::status_indicator(StatusVariant::Success, Some("Live")))
                            }
                        }
                        // GitHub repo link
                        a 
                            href=(format!("https://github.com/{}", project.github_repo))
                            target="_blank"
                            class="inline-flex items-center gap-1.5 mt-1 text-sm text-[var(--text-muted)] hover:text-[var(--text-secondary)] no-underline transition-colors"
                        {
                            (icon::github(14))
                            (&project.github_repo)
                        }
                    }
                }
                
                // Action buttons
                div class="flex items-center gap-2" {
                    (ui::button_link_with_icon(
                        &format!("/projects/{}/settings", id),
                        "Settings",
                        icon::settings(16),
                        ButtonVariant::Secondary,
                        ButtonSize::Medium
                    ))
                    (ui::button_link_with_icon(
                        &format!("/projects/{}/deploy", id),
                        "Deploy now",
                        icon::rocket(16),
                        ButtonVariant::Primary,
                        ButtonSize::Medium
                    ))
                }
            }
            
            // Main content grid
            div class="mt-10 grid lg:grid-cols-3 gap-6" {
                // Left column: Deployments (spans 2 columns)
                div class="lg:col-span-2 space-y-6" {
                    // Recent deployments card
                    (ui::card(html! {
                        (ui::card_header("Recent deployments", None))
                        
                        @if deployments.is_empty() {
                            div class="py-8 text-center" {
                                div class="text-[var(--text-faint)] mb-3" {
                                    (icon::rocket(32))
                                }
                                p class="text-sm text-[var(--text-muted)]" { 
                                    "No deployments yet" 
                                }
                                p class="text-xs text-[var(--text-faint)] mt-1" { 
                                    "Click \"Deploy now\" to create your first deployment." 
                                }
                            }
                        } @else {
                            div class="space-y-1 -mx-2" {
                                @for deployment in &deployments {
                                    (deployment_row(&id, deployment))
                                }
                            }
                        }
                    }))
                }
                
                // Right column: Info cards
                div class="space-y-6" {
                    // Deployment status card
                    @if let Some(fly_app) = &project.fly_app_name {
                        (ui::card_prominent(html! {
                            div class="flex items-center gap-3 mb-4" {
                                div class="w-8 h-8 rounded-lg bg-emerald-500/20 flex items-center justify-center" {
                                    (icon::globe(16))
                                }
                                h2 class="text-sm font-medium text-[var(--text-primary)]" { "URLs" }
                            }

                            div class="space-y-2" {
                                // Custom domains (shown first if any are active)
                                @for domain in &custom_domains {
                                    @if domain.status == "ready" {
                                        a 
                                            href=(format!("https://{}", domain.hostname))
                                            target="_blank"
                                            class="group flex items-center justify-between px-3 py-2.5 -mx-1 rounded-lg bg-[var(--bg-base)] border border-emerald-500/20 hover:border-emerald-500/40 transition-colors no-underline"
                                        {
                                            div class="flex items-center gap-2" {
                                                span class="block w-1.5 h-1.5 rounded-full bg-emerald-500" {}
                                                span class="text-sm font-mono text-[var(--text-primary)] group-hover:text-[var(--accent-light)] transition-colors" {
                                                    (&domain.hostname)
                                                }
                                            }
                                            span class="text-[var(--text-faint)] group-hover:text-[var(--text-muted)] transition-colors" {
                                                (icon::external_link(14))
                                            }
                                        }
                                    } @else {
                                        div class="flex items-center justify-between px-3 py-2.5 -mx-1 rounded-lg bg-[var(--bg-base)] border border-[var(--border-subtle)]" {
                                            div class="flex items-center gap-2" {
                                                span class="block w-1.5 h-1.5 rounded-full bg-amber-500 animate-pulse" {}
                                                span class="text-sm font-mono text-[var(--text-muted)]" {
                                                    (&domain.hostname)
                                                }
                                            }
                                            span class="text-xs text-[var(--text-faint)]" { "Pending" }
                                        }
                                    }
                                }

                                // Fly.dev URL (always shown)
                                a 
                                    href=(format!("https://{}.fly.dev", fly_app))
                                    target="_blank"
                                    class="group flex items-center justify-between px-3 py-2.5 -mx-1 rounded-lg bg-[var(--bg-base)] border border-[var(--border-subtle)] hover:border-[var(--border-default)] transition-colors no-underline"
                                {
                                    span class="text-sm font-mono text-[var(--text-secondary)] group-hover:text-[var(--accent-light)] transition-colors" {
                                        (format!("{}.fly.dev", fly_app))
                                    }
                                    span class="text-[var(--text-faint)] group-hover:text-[var(--text-muted)] transition-colors" {
                                        (icon::external_link(14))
                                    }
                                }
                            }
                        }))
                    } @else {
                        (ui::card(html! {
                            div class="flex items-center gap-3 mb-4" {
                                div class="w-8 h-8 rounded-lg bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-faint)]" {
                                    (icon::globe(16))
                                }
                                h2 class="text-sm font-medium text-[var(--text-secondary)]" { "Status" }
                            }
                            
                            p class="text-sm text-[var(--text-muted)]" { 
                                "Not deployed yet. Create your first deployment to go live."
                            }
                        }))
                    }
                    
                    // Preview environments card
                    @if !preview_environments.is_empty() {
                        (ui::card(html! {
                            div class="flex items-center gap-3 mb-4" {
                                div class="w-8 h-8 rounded-lg bg-purple-500/20 flex items-center justify-center" {
                                    (icon::git_branch(16))
                                }
                                h2 class="text-sm font-medium text-[var(--text-primary)]" { "Preview Environments" }
                            }

                            div class="space-y-2" {
                                @for preview in &preview_environments {
                                    @let preview_url = preview.url.as_deref().unwrap_or_else(|| {
                                        // Should not happen, but fallback
                                        ""
                                    });
                                    a
                                        href=(preview_url)
                                        target="_blank"
                                        class="group flex items-center justify-between px-3 py-2.5 -mx-1 rounded-lg bg-[var(--bg-base)] border border-purple-500/20 hover:border-purple-500/40 transition-colors no-underline"
                                    {
                                        div class="min-w-0 flex-1" {
                                            div class="flex items-center gap-2" {
                                                span class="block w-1.5 h-1.5 rounded-full bg-purple-500" {}
                                                span class="text-sm font-mono text-[var(--text-primary)] group-hover:text-purple-400 transition-colors truncate" {
                                                    (format!("PR #{}", preview.pr_number))
                                                }
                                            }
                                            div class="ml-3.5 mt-0.5 text-xs text-[var(--text-faint)] truncate" {
                                                (icon::git_branch(10))
                                                " "
                                                (&preview.pr_branch)
                                            }
                                        }
                                        span class="flex-shrink-0 text-[var(--text-faint)] group-hover:text-[var(--text-muted)] transition-colors" {
                                            (icon::external_link(14))
                                        }
                                    }
                                }
                            }
                        }))
                    }

                    // Project details card
                    (ui::card(html! {
                        (ui::card_header("Details", None))
                        
                        dl class="space-y-3" {
                            (ui::detail_row("Created", html! { 
                                span class="text-[var(--text-secondary)]" { (&project.created_at) }
                            }))
                            (ui::detail_row("Project ID", html! { 
                                code class="text-xs font-mono text-[var(--text-muted)] bg-[var(--bg-surface)] px-1.5 py-0.5 rounded" { 
                                    (&id[..8.min(id.len())]) 
                                }
                            }))
                        }
                    }))
                }
            }
        }
    })
}

fn deployment_row(project_id: &str, deployment: &Deployment) -> rejoice::Markup {
    let status_variant = match deployment.status.as_str() {
        "pending" => StatusVariant::Neutral,
        "building" => StatusVariant::Building,
        "deploying" => StatusVariant::Building,
        "success" => StatusVariant::Success,
        "failed" => StatusVariant::Error,
        _ => StatusVariant::Neutral,
    };

    let status_badge = match deployment.status.as_str() {
        "pending" => ui::badge("Pending", BadgeVariant::Default),
        "building" => ui::badge("Building", BadgeVariant::Warning),
        "deploying" => ui::badge("Deploying", BadgeVariant::Warning),
        "success" => ui::badge("Live", BadgeVariant::Success),
        "failed" => ui::badge("Failed", BadgeVariant::Error),
        _ => ui::badge(&deployment.status, BadgeVariant::Default),
    };

    let commit_message = deployment
        .git_message
        .as_deref()
        .unwrap_or("No commit message")
        .lines()
        .next()
        .unwrap_or("No commit message");

    // Truncate long messages
    let commit_message = if commit_message.len() > 50 {
        format!("{}...", &commit_message[..47])
    } else {
        commit_message.to_string()
    };

    let short_sha = &deployment.git_sha[..7.min(deployment.git_sha.len())];
    let is_in_progress = matches!(deployment.status.as_str(), "pending" | "building" | "deploying");

    html! {
        a
            href=(format!("/projects/{}/deployments/{}", project_id, deployment.id))
            class="group flex items-center gap-3 px-2 py-3 rounded-lg hover:bg-[var(--bg-surface)] no-underline transition-colors"
        {
            // Status indicator dot
            div class="flex-shrink-0" {
                @if is_in_progress {
                    span class="block w-2 h-2 rounded-full bg-amber-500 animate-pulse shadow-sm shadow-amber-500/50" {}
                } @else {
                    (ui::status_dot(status_variant))
                }
            }
            
            // Main content
            div class="min-w-0 flex-1" {
                // Commit message
                div class="flex items-center gap-2" {
                    span class="text-sm text-[var(--text-secondary)] group-hover:text-[var(--text-primary)] truncate transition-colors" { 
                        (commit_message) 
                    }
                    @if let Some(pr) = deployment.pr_number {
                        span class="flex-shrink-0 text-xs text-[var(--text-faint)] font-mono" { 
                            (format!("#{}", pr)) 
                        }
                    }
                }
                // Meta info
                div class="flex items-center gap-2 mt-0.5 text-xs text-[var(--text-faint)]" {
                    code class="font-mono" { (short_sha) }
                    span { "on" }
                    span class="font-medium" { (&deployment.git_branch) }
                    span class="opacity-50" { "·" }
                    span { (&deployment.started_at) }
                }
            }
            
            // Status badge
            div class="flex-shrink-0" {
                (status_badge)
            }
            
            // Arrow on hover
            span class="flex-shrink-0 text-[var(--text-faint)] opacity-0 -translate-x-1 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-150" {
                (icon::arrow_right(14))
            }
        }
    }
}
