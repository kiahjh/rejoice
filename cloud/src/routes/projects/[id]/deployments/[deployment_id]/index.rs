use crate::components::{self as ui, icon, BadgeVariant};
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, island, Req, Res};

#[derive(FromRow)]
struct Project {
    name: String,
}

#[derive(FromRow)]
struct Deployment {
    id: String,
    git_sha: String,
    git_branch: String,
    git_message: Option<String>,
    pr_number: Option<i64>,
    status: String,
    url: Option<String>,
    build_logs: Option<String>,
    error_message: Option<String>,
    started_at: String,
    finished_at: Option<String>,
}

pub async fn get(
    state: AppState,
    req: Req,
    res: Res,
    id: String,
    deployment_id: String,
) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user
    let project = query_as::<_, Project>(
        r#"
        SELECT p.name
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

    // Fetch deployment
    let deployment = query_as::<_, Deployment>(
        r#"
        SELECT id, git_sha, git_branch, git_message, pr_number, status, url, build_logs, error_message, started_at, finished_at
        FROM deployments
        WHERE id = ? AND project_id = ?
        "#,
    )
    .bind(&deployment_id)
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    let deployment = match deployment {
        Ok(Some(d)) => d,
        Ok(None) => return res.not_found("Deployment not found"),
        Err(_) => return res.internal_error("Database error"),
    };

    let (status_badge, _status_icon) = match deployment.status.as_str() {
        "pending" => (
            ui::badge("Pending", BadgeVariant::Default),
            icon::clock(16),
        ),
        "building" => (
            ui::badge("Building", BadgeVariant::Warning),
            icon::terminal(16),
        ),
        "deploying" => (
            ui::badge("Deploying", BadgeVariant::Warning),
            icon::rocket(16),
        ),
        "success" => (
            ui::badge("Live", BadgeVariant::Success),
            icon::check_circle(16),
        ),
        "failed" => (
            ui::badge("Failed", BadgeVariant::Error),
            icon::x_circle(16),
        ),
        _ => (
            ui::badge(&deployment.status, BadgeVariant::Default),
            icon::clock(16),
        ),
    };

    let commit_message = deployment
        .git_message
        .as_deref()
        .unwrap_or("No commit message");

    // Check if deployment is in progress
    let is_in_progress = matches!(deployment.status.as_str(), "pending" | "building" | "deploying");
    let is_finished = !is_in_progress;
    let is_success = deployment.status == "success";
    let is_failed = deployment.status == "failed";

    // Prepare island props
    let project_id = id.clone();
    let dep_id = deployment.id.clone();
    let initial_logs = deployment.build_logs.clone();
    let initial_status = deployment.status.clone();

    let short_sha = &deployment.git_sha[..7.min(deployment.git_sha.len())];

    res.html(html! {
        div class="max-w-4xl mx-auto px-6 py-10" {
            // Back link
            (ui::back_link(&format!("/projects/{}", id), &project.name))

            // Header
            div class="mt-6" {
                // Status header with icon and badge
                div class="flex items-center gap-3 mb-2" {
                    // Status icon with colored background
                    @if is_success {
                        div class="w-10 h-10 rounded-xl bg-emerald-500/20 flex items-center justify-center text-emerald-400" {
                            (icon::check_circle(20))
                        }
                    } @else if is_failed {
                        div class="w-10 h-10 rounded-xl bg-red-500/20 flex items-center justify-center text-red-400" {
                            (icon::x_circle(20))
                        }
                    } @else if is_in_progress {
                        div class="w-10 h-10 rounded-xl bg-amber-500/20 flex items-center justify-center text-amber-400 animate-pulse" {
                            (icon::rocket(20))
                        }
                    } @else {
                        div class="w-10 h-10 rounded-xl bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-muted)]" {
                            (icon::clock(20))
                        }
                    }
                    
                    div {
                        h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { 
                            "Deployment" 
                        }
                        // Commit info
                        div class="flex items-center gap-2 mt-0.5 text-sm text-[var(--text-muted)]" {
                            (icon::git_commit(14))
                            code class="font-mono text-[var(--text-secondary)]" { (short_sha) }
                            span { "on" }
                            span class="font-medium text-[var(--text-secondary)]" { (&deployment.git_branch) }
                            @if let Some(pr) = deployment.pr_number {
                                span class="text-[var(--text-faint)]" { (format!("(PR #{})", pr)) }
                            }
                        }
                    }
                    
                    // Status badge
                    div class="ml-auto" {
                        (status_badge)
                    }
                }
            }

            // Content
            div class="mt-8 space-y-6" {
                // Success state with URL
                @if is_success {
                    @if let Some(url) = &deployment.url {
                        (ui::card_prominent(html! {
                            div class="flex items-center justify-between" {
                                div class="flex items-center gap-3" {
                                    div class="w-10 h-10 rounded-lg bg-emerald-500/20 flex items-center justify-center text-emerald-400" {
                                        (icon::globe(20))
                                    }
                                    div {
                                        p class="text-sm font-medium text-[var(--text-primary)]" { "Deployment live" }
                                        p class="text-xs text-[var(--text-muted)] mt-0.5" { "Your app is now accessible at:" }
                                    }
                                }
                                a 
                                    href=(url)
                                    target="_blank"
                                    class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-[var(--text-primary)] bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg hover:border-[var(--accent)] hover:text-[var(--accent-light)] transition-colors no-underline"
                                {
                                    (url)
                                    (icon::external_link(14))
                                }
                            }
                        }))
                    }
                }

                // Error message (if failed)
                @if let Some(error) = &deployment.error_message {
                    (ui::card(html! {
                        div class="flex items-start gap-3" {
                            div class="flex-shrink-0 w-8 h-8 rounded-lg bg-red-500/20 flex items-center justify-center text-red-400" {
                                (icon::x_circle(16))
                            }
                            div class="min-w-0 flex-1" {
                                h2 class="text-sm font-medium text-red-400" { "Deployment failed" }
                                pre class="mt-3 text-sm text-red-300/80 whitespace-pre-wrap font-mono bg-red-950/20 rounded-lg p-4 overflow-x-auto border border-red-900/30" {
                                    (error)
                                }
                            }
                        }
                    }))
                }

                // Two-column layout for commit and details
                div class="grid md:grid-cols-2 gap-6" {
                    // Commit info card
                    (ui::card(html! {
                        (ui::card_header("Commit", None))
                        p class="text-sm text-[var(--text-secondary)] whitespace-pre-wrap leading-relaxed" { 
                            (commit_message) 
                        }
                    }))

                    // Deployment details card
                    (ui::card(html! {
                        (ui::card_header("Details", None))

                        dl class="space-y-3" {
                            (ui::detail_row("Status", html! { 
                                span class="capitalize" { (&deployment.status) }
                            }))
                            (ui::detail_row("Started", html! { 
                                span { (&deployment.started_at) }
                            }))
                            @if let Some(finished) = &deployment.finished_at {
                                (ui::detail_row("Finished", html! { 
                                    span { (finished) }
                                }))
                            }
                            (ui::detail_row("ID", html! { 
                                code class="text-xs font-mono text-[var(--text-muted)] bg-[var(--bg-surface)] px-1.5 py-0.5 rounded" { 
                                    (&deployment.id[..8.min(deployment.id.len())]) 
                                }
                            }))
                        }
                    }))
                }

                // Build logs (interactive island with auto-refresh)
                (island!(LogViewer, {
                    projectId: project_id,
                    deploymentId: dep_id,
                    initialLogs: initial_logs,
                    initialStatus: initial_status,
                    initialFinished: is_finished
                }))
            }
        }
    })
}
