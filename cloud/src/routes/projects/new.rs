use crate::components::{self as ui, ButtonSize, ButtonVariant};
use crate::AppState;
use rejoice::db::{query, query_as, FromRow};
use rejoice::{html, Markup, Req, Res};
use serde::Deserialize;
use uuid::Uuid;

#[derive(FromRow)]
struct UserWithInstallation {
    id: String,
    github_app_installation_id: Option<i64>,
}

#[derive(Deserialize)]
struct NewProjectForm {
    name: String,
    github_repo: String,
}

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Get user with installation info
    let user: Option<UserWithInstallation> = query_as(
        "SELECT id, github_app_installation_id FROM users WHERE github_id = ?",
    )
    .bind(github_id)
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten();

    let user = match user {
        Some(u) => u,
        None => return res.redirect("/"),
    };

    // Check if GitHub App is installed
    let installation_id = match user.github_app_installation_id {
        Some(id) => id,
        None => {
            // Show page prompting user to install GitHub App
            return res.html(render_install_prompt());
        }
    };

    // Fetch repos from GitHub App installation
    let repos = match state.github_app.list_installation_repos(installation_id).await {
        Ok(repos) => repos,
        Err(e) => {
            eprintln!("Failed to fetch repos: {}", e);
            return res.html(render_error(&format!(
                "Failed to fetch repositories from GitHub: {}. You may need to reinstall the GitHub App.",
                e
            )));
        }
    };

    res.html(render_repo_selector(&repos))
}

fn render_install_prompt() -> Markup {
    html! {
        div class="max-w-md mx-auto px-6 py-10" {
            a href="/projects" class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← Back"
            }
            
            h1 class="mt-6 text-xl font-medium text-stone-100" { "Connect GitHub" }
            
            p class="mt-4 text-stone-400" {
                "To create a project, you need to install the Rejoice Cloud GitHub App on your account. "
                "This allows us to access your repositories and deploy your code."
            }
            
            div class="mt-8" {
                a 
                    href="https://github.com/apps/rejoice-cloud-dev/installations/new"
                    class="inline-flex items-center justify-center h-10 px-5 text-sm font-medium rounded-lg bg-amber-600 text-white hover:bg-amber-500 transition-colors no-underline"
                {
                    "Install GitHub App"
                }
            }
            
            p class="mt-4 text-xs text-stone-500" {
                "You'll be redirected to GitHub to authorize the app."
            }
        }
    }
}

fn render_error(message: &str) -> Markup {
    html! {
        div class="max-w-md mx-auto px-6 py-10" {
            a href="/projects" class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← Back"
            }
            
            h1 class="mt-6 text-xl font-medium text-stone-100" { "Error" }
            
            p class="mt-4 text-red-400" {
                (message)
            }
            
            div class="mt-8 flex gap-3" {
                (ui::button_link("/projects", "Back to Projects", ButtonVariant::Secondary, ButtonSize::Medium))
                a 
                    href="https://github.com/apps/rejoice-cloud-dev/installations/new"
                    class="inline-flex items-center justify-center h-9 px-4 text-sm font-medium rounded-lg bg-amber-600 text-white hover:bg-amber-500 transition-colors no-underline"
                {
                    "Reinstall GitHub App"
                }
            }
        }
    }
}

fn render_repo_selector(repos: &[crate::github::Repository]) -> Markup {
    html! {
        div class="max-w-lg mx-auto px-6 py-10" {
            a href="/projects" class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← Back"
            }
            
            h1 class="mt-6 text-xl font-medium text-stone-100" { "New project" }
            p class="mt-2 text-stone-400" { 
                "Select a repository to deploy" 
                span class="text-stone-500" { (format!(" ({} available)", repos.len())) }
            }
            
            @if repos.is_empty() {
                div class="mt-8 p-6 rounded-lg border border-stone-800 text-center" {
                    p class="text-stone-400" { "No repositories found." }
                    p class="mt-2 text-sm text-stone-500" {
                        "Make sure you've granted access to your repositories when installing the GitHub App."
                    }
                    a 
                        href="https://github.com/apps/rejoice-cloud-dev/installations/new"
                        class="mt-4 inline-block text-sm text-amber-500 hover:text-amber-400"
                    {
                        "Configure repository access"
                    }
                }
            } @else {
                // Search input
                div class="mt-6" {
                    input 
                        type="text"
                        id="repo-search"
                        placeholder="Search repositories..."
                        autocomplete="off"
                        class="w-full px-4 py-2.5 bg-stone-900 border border-stone-700 rounded-lg text-stone-100 placeholder-stone-500 focus:outline-none focus:ring-2 focus:ring-amber-600 focus:border-transparent"
                    {}
                }
                
                // Repo list
                div id="repo-list" class="mt-4 space-y-2 max-h-96 overflow-y-auto" {
                    @for repo in repos {
                        form method="post" class="block repo-item" data-repo-name=(repo.full_name.to_lowercase()) {
                            input type="hidden" name="github_repo" value=(repo.full_name);
                            input type="hidden" name="name" value=(repo.name);
                            button 
                                type="submit"
                                class="w-full text-left p-4 rounded-lg border border-stone-800 hover:border-stone-700 hover:bg-stone-900/50 transition-colors"
                            {
                                div class="flex items-center justify-between" {
                                    div {
                                        span class="font-medium text-stone-100" { (repo.full_name) }
                                        @if repo.private {
                                            span class="ml-2 text-xs px-1.5 py-0.5 rounded bg-stone-800 text-stone-400" {
                                                "private"
                                            }
                                        }
                                    }
                                    span class="text-stone-500 text-sm" {
                                        (repo.default_branch)
                                    }
                                }
                            }
                        }
                    }
                }
                
                // No results message (hidden by default)
                p id="no-results" class="mt-4 text-center text-stone-500 hidden" {
                    "No repositories match your search."
                }
                
                // Search script
                script {
                    (maud::PreEscaped(r#"
                        document.getElementById('repo-search').addEventListener('input', function(e) {
                            const query = e.target.value.toLowerCase();
                            const items = document.querySelectorAll('.repo-item');
                            let visibleCount = 0;
                            
                            items.forEach(item => {
                                const name = item.dataset.repoName;
                                if (name.includes(query)) {
                                    item.style.display = 'block';
                                    visibleCount++;
                                } else {
                                    item.style.display = 'none';
                                }
                            });
                            
                            document.getElementById('no-results').classList.toggle('hidden', visibleCount > 0);
                        });
                        
                        // Focus search on page load
                        document.getElementById('repo-search').focus();
                    "#))
                }
            }
        }
    }
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

    // Get user with installation info
    let user: Option<UserWithInstallation> = query_as(
        "SELECT id, github_app_installation_id FROM users WHERE github_id = ?",
    )
    .bind(github_id)
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten();

    let user = match user {
        Some(u) => u,
        None => return res.internal_error("User not found"),
    };

    let installation_id = match user.github_app_installation_id {
        Some(id) => id,
        None => return res.redirect("/projects/new"),
    };

    let project_id = Uuid::new_v4().to_string();
    let result = query(
        "INSERT INTO projects (id, user_id, name, github_repo, github_installation_id) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&project_id)
    .bind(&user.id)
    .bind(&form.name)
    .bind(&form.github_repo)
    .bind(installation_id)
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
