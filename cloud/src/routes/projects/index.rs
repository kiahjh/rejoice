use crate::components::{self as ui, ButtonSize, ButtonVariant};
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
        div class="max-w-3xl mx-auto px-6 py-10" {
            // Header
            div class="flex items-center justify-between mb-8" {
                h1 class="text-xl font-medium text-stone-100" { "Projects" }
                (ui::button_link_with_icon(
                    "/projects/new",
                    "New project",
                    ui::icon::plus(16),
                    ButtonVariant::Primary,
                    ButtonSize::Medium
                ))
            }

            @if projects.is_empty() {
                // Empty state
                div class="text-center py-16" {
                    p class="text-stone-500" { "No projects yet" }
                }
            } @else {
                // Project list
                div class="space-y-3" {
                    @for project in &projects {
                        (ui::project_card(
                            &format!("/projects/{}", project.id),
                            &project.name,
                            &project.github_repo,
                            project.fly_app_name.is_some()
                        ))
                    }
                }
            }
        }
    })
}
