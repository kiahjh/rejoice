use crate::components::{self as ui, BadgeVariant, ButtonSize, ButtonVariant, StatusVariant};
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
    url: Option<String>,
    started_at: String,
    finished_at: Option<String>,
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

    res.html(html! {
        div class="max-w-3xl mx-auto px-6 py-10" {
            // Back link
            a href="/projects" class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← Projects"
            }
            
            // Header
            div class="mt-6 flex items-start justify-between" {
                div {
                    div class="flex items-center gap-3" {
                        h1 class="text-xl font-medium text-stone-100" { (&project.name) }
                        @if is_deployed {
                            (ui::status_dot(StatusVariant::Success))
                        }
                    }
                    a 
                        href=(format!("https://github.com/{}", project.github_repo))
                        target="_blank"
                        class="text-sm text-stone-500 hover:text-stone-300 no-underline"
                    {
                        (&project.github_repo)
                    }
                }
                // Action buttons
                div class="flex items-center gap-2" {
                    (ui::button_link(
                        &format!("/projects/{}/settings", id),
                        "Settings",
                        ButtonVariant::Secondary,
                        ButtonSize::Medium
                    ))
                    (ui::button_link(
                        &format!("/projects/{}/deploy", id),
                        "Deploy now",
                        ButtonVariant::Primary,
                        ButtonSize::Medium
                    ))
                }
            }
            
            // Content
            div class="mt-10 space-y-6" {
                // Status
                (ui::card(html! {
                    h2 class="text-sm font-medium text-stone-300 mb-4" { "Deployment" }
                    
                    @if let Some(fly_app) = &project.fly_app_name {
                        div class="flex items-center justify-between" {
                            div {
                                p class="text-sm text-stone-400" { "Live at" }
                                a 
                                    href=(format!("https://{}.fly.dev", fly_app))
                                    target="_blank"
                                    class="text-stone-200 hover:text-amber-400 no-underline"
                                {
                                    (format!("{}.fly.dev", fly_app))
                                }
                            }
                            (ui::badge("Deployed", BadgeVariant::Success))
                        }
                    } @else {
                        p class="text-sm text-stone-500" { "Not deployed yet" }
                    }
                }))
                
                // Info
                (ui::card(html! {
                    h2 class="text-sm font-medium text-stone-300 mb-4" { "Details" }
                    
                    dl class="space-y-3 text-sm" {
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Created" }
                            dd class="text-stone-300" { (&project.created_at) }
                        }
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Project ID" }
                            dd class="text-stone-300 font-mono text-xs" { (&id) }
                        }
                    }
                }))

                // Deployments
                (ui::card(html! {
                    h2 class="text-sm font-medium text-stone-300 mb-4" { "Deployments" }
                    
                    @if deployments.is_empty() {
                        p class="text-sm text-stone-500" { "No deployments yet. Click \"Deploy now\" to get started." }
                    } @else {
                        div class="space-y-3" {
                            @for deployment in &deployments {
                                (deployment_row(&id, deployment))
                            }
                        }
                    }
                }))
            }
        }
    })
}

fn deployment_row(project_id: &str, deployment: &Deployment) -> rejoice::Markup {
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
    let commit_message = if commit_message.len() > 60 {
        format!("{}...", &commit_message[..57])
    } else {
        commit_message.to_string()
    };

    let short_sha = &deployment.git_sha[..7.min(deployment.git_sha.len())];

    html! {
        a
            href=(format!("/projects/{}/deployments/{}", project_id, deployment.id))
            class="block -mx-2 px-2 py-3 rounded-lg hover:bg-stone-800/50 no-underline transition-colors"
        {
            div class="flex items-center justify-between gap-4" {
                div class="min-w-0 flex-1" {
                    div class="flex items-center gap-2" {
                        span class="text-sm text-stone-200 truncate" { (commit_message) }
                        @if let Some(pr) = deployment.pr_number {
                            span class="text-xs text-stone-500" { (format!("#{}", pr)) }
                        }
                    }
                    div class="flex items-center gap-2 mt-1 text-xs text-stone-500" {
                        span class="font-mono" { (short_sha) }
                        span { " on " }
                        span { (&deployment.git_branch) }
                        span { " • " }
                        span { (&deployment.started_at) }
                    }
                }
                (status_badge)
            }
        }
    }
}
