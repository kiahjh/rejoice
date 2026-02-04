use crate::components::{self as ui, ButtonSize, ButtonVariant};
use crate::AppState;
use rejoice::db::{query, query_scalar};
use rejoice::{html, Req, Res};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
struct NewProjectForm {
    name: String,
    github_repo: String,
}

pub async fn get(_state: AppState, req: Req, res: Res) -> Res {
    if req.cookies.get("session").is_none() {
        return res.redirect("/");
    }

    res.html(html! {
        div class="max-w-md mx-auto px-6 py-10" {
            // Back link
            a href="/projects" class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← Back"
            }
            
            h1 class="mt-6 text-xl font-medium text-stone-100" { "New project" }
            
            // Form
            form method="post" class="mt-8 space-y-5" {
                div {
                    label for="name" class="block text-sm font-medium text-stone-300 mb-2" {
                        "Project name"
                    }
                    (ui::input("name", "my-app"))
                }

                div {
                    label for="github_repo" class="block text-sm font-medium text-stone-300 mb-2" {
                        "GitHub repository"
                    }
                    (ui::input("github_repo", "username/repo"))
                    p class="mt-2 text-xs text-stone-500" {
                        "The repository containing your Rejoice app."
                    }
                }

                div class="pt-4 flex gap-3" {
                    (ui::button_link("/projects", "Cancel", ButtonVariant::Secondary, ButtonSize::Medium))
                    button 
                        type="submit" 
                        class="inline-flex items-center justify-center h-9 px-4 text-sm font-medium rounded-lg bg-amber-600 text-white hover:bg-amber-500 transition-colors"
                    {
                        "Create project"
                    }
                }
            }
        }
    })
}

pub async fn post(state: AppState, req: Req, res: Res) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    let form: NewProjectForm = match req.body.as_form() {
        Ok(f) => f,
        Err(_) => return res.bad_request("Invalid form data"),
    };

    let user_id: Option<String> = query_scalar("SELECT id FROM users WHERE github_id = ?")
        .bind(github_id)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten();

    let user_id = match user_id {
        Some(id) => id,
        None => return res.internal_error("User not found"),
    };

    let project_id = Uuid::new_v4().to_string();
    let result = query("INSERT INTO projects (id, user_id, name, github_repo) VALUES (?, ?, ?, ?)")
        .bind(&project_id)
        .bind(&user_id)
        .bind(&form.name)
        .bind(&form.github_repo)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => res.redirect(&format!("/projects/{}", project_id)),
        Err(e) => {
            eprintln!("Failed to create project: {}", e);
            res.internal_error("Failed to create project")
        }
    }
}
