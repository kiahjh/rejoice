use crate::components::{self as ui, icon, ButtonSize, ButtonVariant};
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, Req, Res};

#[derive(FromRow)]
struct Project {
    id: String,
    name: String,
    github_repo: String,
    fly_app_name: Option<String>,
}



pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    let projects = query_as::<_, Project>(
        r#"
        SELECT p.id, p.name, p.github_repo, p.fly_app_name
        FROM projects p
        JOIN users u ON p.user_id = u.id
        WHERE u.github_id = ?
        ORDER BY p.created_at DESC
        "#,
    )
    .bind(github_id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    res.html(html! {
        div class="max-w-4xl mx-auto px-6 py-10" {
            // Header with stats
            div class="flex items-start justify-between mb-10" {
                div {
                    h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { "Projects" }
                    p class="mt-1 text-sm text-[var(--text-muted)]" { 
                        (format!("{} project{}", projects.len(), if projects.len() == 1 { "" } else { "s" }))
                    }
                }
                (ui::button_link_with_icon(
                    "/projects/new",
                    "New project",
                    icon::plus(16),
                    ButtonVariant::Primary,
                    ButtonSize::Medium
                ))
            }

            @if projects.is_empty() {
                // Empty state
                (ui::empty_state(
                    Some(icon::folder(48)),
                    "No projects yet",
                    "Create your first project to get started with Rejoice Cloud.",
                    Some(ui::button_link_with_icon(
                        "/projects/new",
                        "Create project",
                        icon::plus(16),
                        ButtonVariant::Primary,
                        ButtonSize::Large
                    ))
                ))
            } @else {
                // Project list with better visual hierarchy
                div class="space-y-3 stagger-children" {
                    @for project in &projects {
                        (project_card_enhanced(&project))
                    }
                }
            }
        }
    })
}

fn project_card_enhanced(project: &Project) -> rejoice::Markup {
    let is_deployed = project.fly_app_name.is_some();
    let status = if is_deployed {
        ui::status_indicator(ui::StatusVariant::Success, Some("Live"))
    } else {
        ui::status_indicator(ui::StatusVariant::Neutral, Some("Not deployed"))
    };

    html! {
        a
            href=(format!("/projects/{}", project.id))
            class="group block bg-[var(--bg-elevated)] border border-[var(--border-subtle)] rounded-xl p-5 \
                   no-underline transition-all duration-200 cursor-pointer \
                   hover:bg-[var(--bg-surface)] hover:border-[var(--border-default)] \
                   hover:shadow-lg hover:-translate-y-0.5"
        {
            div class="flex items-center justify-between gap-4" {
                // Left: Project info
                div class="min-w-0 flex-1" {
                    div class="flex items-center gap-3" {
                        // Project icon
                        div class="flex-shrink-0 w-10 h-10 rounded-lg bg-gradient-to-br from-[var(--bg-surface)] to-[var(--bg-hover)] border border-[var(--border-default)] flex items-center justify-center" {
                            span class="text-lg" { "◈" }
                        }
                        
                        div class="min-w-0" {
                            // Project name with arrow on hover
                            div class="flex items-center gap-2" {
                                h3 class="text-base font-medium text-[var(--text-primary)] truncate group-hover:text-[var(--accent-light)] transition-colors" { 
                                    (&project.name) 
                                }
                                span class="text-[var(--text-faint)] opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-200" {
                                    "→"
                                }
                            }
                            // Repo path
                            p class="mt-0.5 text-sm text-[var(--text-muted)] truncate font-mono" { 
                                (&project.github_repo) 
                            }
                        }
                    }
                }
                
                // Right: Status and URL
                div class="flex-shrink-0 flex items-center gap-4" {
                    @if let Some(fly_app) = &project.fly_app_name {
                        // Live URL badge
                        span class="hidden md:inline-flex items-center gap-1.5 px-2.5 py-1 text-xs font-mono text-[var(--text-muted)] bg-[var(--bg-surface)] border border-[var(--border-subtle)] rounded-lg" {
                            (icon::globe(12))
                            (format!("{}.fly.dev", fly_app))
                        }
                    }
                    (status)
                }
            }
        }
    }
}
