use crate::components::{self as ui, icon, ButtonSize, ButtonVariant};
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
        div class="max-w-lg mx-auto px-6 py-10" {
            (ui::back_link("/projects", "Projects"))
            
            div class="mt-10 text-center" {
                // Icon
                div class="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-[var(--bg-surface)] border border-[var(--border-default)] mb-6" {
                    (icon::github(32))
                }
                
                h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { 
                    "Connect GitHub" 
                }
                
                p class="mt-3 text-[var(--text-secondary)] max-w-sm mx-auto" {
                    "Install the Rejoice Cloud GitHub App to grant access to your repositories and enable automatic deployments."
                }
                
                div class="mt-8" {
                    a 
                        href="https://github.com/apps/rejoice-cloud-dev/installations/new"
                        class="inline-flex items-center justify-center gap-2 h-11 px-6 text-sm font-medium rounded-xl \
                               bg-gradient-to-b from-amber-500 to-amber-600 text-white \
                               shadow-md shadow-amber-900/25 \
                               hover:from-amber-400 hover:to-amber-500 \
                               hover:shadow-lg hover:shadow-amber-900/30 \
                               transition-all no-underline"
                    {
                        (icon::github(18))
                        "Install GitHub App"
                    }
                }
                
                p class="mt-4 text-xs text-[var(--text-faint)]" {
                    "You'll be redirected to GitHub to authorize the app."
                }
            }
        }
    }
}

fn render_error(message: &str) -> Markup {
    html! {
        div class="max-w-lg mx-auto px-6 py-10" {
            (ui::back_link("/projects", "Projects"))
            
            div class="mt-10" {
                // Error card
                (ui::card(html! {
                    div class="flex items-start gap-3" {
                        div class="flex-shrink-0 w-10 h-10 rounded-lg bg-red-500/10 flex items-center justify-center text-red-400" {
                            (icon::x_circle(20))
                        }
                        div {
                            h1 class="text-lg font-semibold text-[var(--text-primary)]" { "Unable to load repositories" }
                            p class="mt-2 text-sm text-red-400/80" {
                                (message)
                            }
                        }
                    }
                }))
                
                div class="mt-6 flex gap-3" {
                    (ui::button_link("/projects", "Back to Projects", ButtonVariant::Secondary, ButtonSize::Medium))
                    a 
                        href="https://github.com/apps/rejoice-cloud-dev/installations/new"
                        class="inline-flex items-center justify-center gap-2 h-9 px-4 text-sm font-medium rounded-lg \
                               bg-gradient-to-b from-amber-500 to-amber-600 text-white \
                               hover:from-amber-400 hover:to-amber-500 \
                               transition-all no-underline"
                    {
                        "Reinstall GitHub App"
                    }
                }
            }
        }
    }
}

fn render_repo_selector(repos: &[crate::github::Repository]) -> Markup {
    html! {
        div class="max-w-xl mx-auto px-6 py-10" {
            (ui::back_link("/projects", "Projects"))
            
            div class="mt-6" {
                h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { 
                    "New project" 
                }
                p class="mt-2 text-[var(--text-muted)]" { 
                    "Select a repository to deploy" 
                    span class="text-[var(--text-faint)]" { (format!(" ({} available)", repos.len())) }
                }
            }
            
            @if repos.is_empty() {
                // Empty state
                div class="mt-8" {
                    (ui::card(html! {
                        div class="py-8 text-center" {
                            div class="text-[var(--text-faint)] mb-3" {
                                (icon::folder(32))
                            }
                            p class="text-sm text-[var(--text-muted)]" { "No repositories found" }
                            p class="text-xs text-[var(--text-faint)] mt-1 max-w-xs mx-auto" {
                                "Make sure you've granted access to your repositories when installing the GitHub App."
                            }
                            a 
                                href="https://github.com/apps/rejoice-cloud-dev/installations/new"
                                class="mt-4 inline-flex items-center gap-1.5 text-sm text-[var(--accent)] hover:text-[var(--accent-light)] no-underline transition-colors"
                            {
                                "Configure repository access"
                                span { "→" }
                            }
                        }
                    }))
                }
            } @else {
                // Search input
                div class="mt-6" {
                    div class="relative" {
                        div class="absolute left-3.5 top-1/2 -translate-y-1/2 text-[var(--text-faint)] pointer-events-none" {
                            svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" {
                                circle cx="11" cy="11" r="8" {}
                                line x1="21" y1="21" x2="16.65" y2="16.65" {}
                            }
                        }
                        input 
                            type="text"
                            id="repo-search"
                            placeholder="Search repositories..."
                            autocomplete="off"
                            class="w-full h-11 pl-10 pr-4 text-sm \
                                   bg-[var(--bg-base)] border border-[var(--border-default)] rounded-xl \
                                   text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                                   outline-none transition-all duration-150 \
                                   hover:border-[var(--border-strong)] \
                                   focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)]"
                        {}
                    }
                }
                
                // Repo list
                div id="repo-list" class="mt-4 space-y-2 max-h-[480px] overflow-y-auto pr-1" {
                    @for repo in repos {
                        form method="post" class="block repo-item" data-repo-name=(repo.full_name.to_lowercase()) {
                            input type="hidden" name="github_repo" value=(repo.full_name);
                            input type="hidden" name="name" value=(repo.name);
                            button 
                                type="submit"
                                class="group w-full text-left p-4 rounded-xl \
                                       bg-[var(--bg-elevated)] border border-[var(--border-subtle)] \
                                       hover:bg-[var(--bg-surface)] hover:border-[var(--border-default)] \
                                       transition-all duration-150 cursor-pointer"
                            {
                                div class="flex items-center justify-between gap-3" {
                                    div class="flex items-center gap-3 min-w-0" {
                                        // Repo icon
                                        div class="flex-shrink-0 w-9 h-9 rounded-lg bg-[var(--bg-surface)] border border-[var(--border-subtle)] flex items-center justify-center text-[var(--text-faint)] group-hover:border-[var(--border-default)] transition-colors" {
                                            (icon::folder(16))
                                        }
                                        
                                        div class="min-w-0" {
                                            div class="flex items-center gap-2" {
                                                span class="font-medium text-[var(--text-primary)] truncate group-hover:text-[var(--accent-light)] transition-colors" { 
                                                    (repo.full_name) 
                                                }
                                                @if repo.private {
                                                    span class="flex-shrink-0 text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-surface)] text-[var(--text-faint)] border border-[var(--border-subtle)]" {
                                                        "private"
                                                    }
                                                }
                                            }
                                            div class="flex items-center gap-2 mt-0.5 text-xs text-[var(--text-faint)]" {
                                                (icon::git_branch(12))
                                                span { (repo.default_branch) }
                                            }
                                        }
                                    }
                                    
                                    // Arrow on hover
                                    span class="flex-shrink-0 text-[var(--text-faint)] opacity-0 -translate-x-1 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-150" {
                                        (icon::arrow_right(16))
                                    }
                                }
                            }
                        }
                    }
                }
                
                // No results message (hidden by default)
                div id="no-results" class="hidden mt-8" {
                    (ui::card(html! {
                        div class="py-6 text-center" {
                            p class="text-sm text-[var(--text-muted)]" { "No repositories match your search." }
                        }
                    }))
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
