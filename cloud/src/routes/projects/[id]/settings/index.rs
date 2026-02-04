use crate::components::{self as ui, BadgeVariant, ButtonSize, ButtonVariant};
use crate::crypto;
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, Req, Res};

#[derive(FromRow)]
struct Project {
    id: String,
    name: String,
}

#[derive(FromRow)]
struct EnvVar {
    id: String,
    key: String,
    encrypted_value: Vec<u8>,
    is_preview_only: bool,
}

pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user
    let project = query_as::<_, Project>(
        r#"
        SELECT p.id, p.name
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

    // Fetch env vars
    let env_vars = query_as::<_, EnvVar>(
        r#"
        SELECT id, key, encrypted_value, is_preview_only
        FROM env_vars
        WHERE project_id = ?
        ORDER BY key ASC
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    res.html(html! {
        div class="max-w-3xl mx-auto px-6 py-10" {
            // Back link
            a href=(format!("/projects/{}", id)) class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
                "← " (&project.name)
            }

            // Header
            div class="mt-6 mb-10" {
                h1 class="text-xl font-medium text-stone-100" { "Settings" }
            }

            // Environment Variables Section
            (ui::card(html! {
                div class="flex items-center justify-between mb-6" {
                    div {
                        h2 class="text-sm font-medium text-stone-300" { "Environment Variables" }
                        p class="text-xs text-stone-500 mt-1" { "Secrets are encrypted at rest and injected at deploy time." }
                    }
                }

                // Add new env var form
                form
                    method="POST"
                    action=(format!("/projects/{}/settings/env", id))
                    class="flex items-center gap-3 mb-6"
                {
                    div class="flex-1" {
                        (ui::input("key", "KEY"))
                    }
                    div class="flex-1" {
                        (ui::input_password("value", "value"))
                    }
                    (ui::checkbox("preview_only", "Preview only", false))
                    (ui::button_submit("Add", ButtonVariant::Primary, ButtonSize::Medium))
                }

                // Existing env vars list
                @if env_vars.is_empty() {
                    p class="text-sm text-stone-500 text-center py-4" { "No environment variables configured." }
                } @else {
                    div class="space-y-2" {
                        @for env_var in &env_vars {
                            (env_var_row(&project.id, env_var, &state.encryption_key))
                        }
                    }
                }
            }))

            // Danger Zone
            div class="mt-10" {
                (ui::card(html! {
                    h2 class="text-sm font-medium text-red-400 mb-4" { "Danger Zone" }

                    div class="flex items-center justify-between" {
                        div {
                            p class="text-sm text-stone-300" { "Delete this project" }
                            p class="text-xs text-stone-500 mt-1" { "This action cannot be undone. All deployments will be stopped." }
                        }
                        form method="POST" action=(format!("/projects/{}/delete", id)) {
                            (ui::button_submit("Delete project", ButtonVariant::Danger, ButtonSize::Small))
                        }
                    }
                }))
            }
        }
    })
}

fn env_var_row(project_id: &str, env_var: &EnvVar, key: &[u8; 32]) -> rejoice::Markup {
    // Decrypt value for display (masked)
    let decrypted = crypto::decrypt(&env_var.encrypted_value, key).unwrap_or_default();
    let masked = mask_value(&decrypted);

    html! {
        div class="flex items-center gap-3 py-2 px-3 -mx-3 rounded-lg hover:bg-stone-800/30" {
            // Key name
            code class="text-sm font-mono text-stone-200 w-48 truncate" { (&env_var.key) }

            // Masked value
            code class="text-sm font-mono text-stone-500 flex-1 truncate" { (masked) }

            // Preview only badge
            @if env_var.is_preview_only {
                (ui::badge("Preview", BadgeVariant::Default))
            }

            // Delete button
            form
                method="POST"
                action=(format!("/projects/{}/settings/env/{}/delete", project_id, env_var.id))
                class="flex-shrink-0"
            {
                button
                    type="submit"
                    class="text-stone-500 hover:text-red-400 transition-colors p-1"
                    title="Delete"
                {
                    (ui::icon::trash(16))
                }
            }
        }
    }
}

fn mask_value(value: &str) -> String {
    if value.len() <= 4 {
        "*".repeat(value.len().max(4))
    } else {
        format!("{}...", "*".repeat(4))
    }
}


