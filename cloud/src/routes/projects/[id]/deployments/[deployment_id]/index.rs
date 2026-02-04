use crate::components::{self as ui, BadgeVariant};
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
        .unwrap_or("No commit message");

    // Check if deployment is in progress
    let is_in_progress = matches!(deployment.status.as_str(), "pending" | "building" | "deploying");
    let is_finished = !is_in_progress;

    // Prepare island props - need to extract values for the macro
    let project_id = id.clone();
    let dep_id = deployment.id.clone();
    let initial_logs = deployment.build_logs.clone();
    let initial_status = deployment.status.clone();

    res.html(html! {
        div class="max-w-3xl mx-auto px-6 py-10" {
            // Back link
            a href=(format!("/projects/{}", id)) class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← " (&project.name)
            }

            // Header
            div class="mt-6" {
                div class="flex items-center gap-3 mb-2" {
                    h1 class="text-xl font-medium text-stone-100" { "Deployment" }
                    (status_badge)
                }
                p class="text-sm text-stone-400" {
                    "Commit "
                    code class="font-mono text-stone-300" { (&deployment.git_sha[..7.min(deployment.git_sha.len())]) }
                    " on "
                    span class="text-stone-300" { (&deployment.git_branch) }
                    @if let Some(pr) = deployment.pr_number {
                        " (PR #" (pr) ")"
                    }
                }
            }

            // Content
            div class="mt-10 space-y-6" {
                // Commit info
                (ui::card(html! {
                    h2 class="text-sm font-medium text-stone-300 mb-4" { "Commit" }
                    p class="text-sm text-stone-200 whitespace-pre-wrap" { (commit_message) }
                }))

                // Deployment details
                (ui::card(html! {
                    h2 class="text-sm font-medium text-stone-300 mb-4" { "Details" }

                    dl class="space-y-3 text-sm" {
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Status" }
                            dd class="text-stone-300" { (&deployment.status) }
                        }
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Started" }
                            dd class="text-stone-300" { (&deployment.started_at) }
                        }
                        @if let Some(finished) = &deployment.finished_at {
                            div class="flex justify-between" {
                                dt class="text-stone-500" { "Finished" }
                                dd class="text-stone-300" { (finished) }
                            }
                        }
                        @if let Some(url) = &deployment.url {
                            div class="flex justify-between" {
                                dt class="text-stone-500" { "URL" }
                                dd {
                                    a
                                        href=(url)
                                        target="_blank"
                                        class="text-amber-400 hover:text-amber-300 no-underline"
                                    {
                                        (url)
                                    }
                                }
                            }
                        }
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Deployment ID" }
                            dd class="text-stone-300 font-mono text-xs" { (&deployment.id) }
                        }
                    }
                }))

                // Error message (if failed)
                @if let Some(error) = &deployment.error_message {
                    (ui::card(html! {
                        h2 class="text-sm font-medium text-red-400 mb-4" { "Error" }
                        pre class="text-sm text-red-300 whitespace-pre-wrap font-mono bg-red-950/30 rounded-lg p-4 overflow-x-auto" {
                            (error)
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
