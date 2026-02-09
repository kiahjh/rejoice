use crate::components::{self as ui, icon, BadgeVariant, ButtonSize, ButtonVariant};
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
        div class="max-w-4xl mx-auto px-6 py-10" {
            // Back link
            (ui::back_link(&format!("/projects/{}", id), &project.name))

            // Header
            div class="mt-6 mb-10" {
                div class="flex items-center gap-3" {
                    div class="w-10 h-10 rounded-xl bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-muted)]" {
                        (icon::settings(20))
                    }
                    div {
                        h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { "Settings" }
                        p class="text-sm text-[var(--text-muted)]" { "Configure your project settings" }
                    }
                }
            }

            // Main content
            div class="space-y-8" {
                // Environment Variables Section
                (ui::card(html! {
                    (ui::card_header(
                        "Environment Variables",
                        Some("Encrypted at rest and injected at deploy time. Changes require a new deployment.")
                    ))

                    // Add new env var form
                    form
                        method="POST"
                        action=(format!("/projects/{}/settings/env", id))
                        class="mb-6"
                    {
                        div class="grid grid-cols-[1fr_1fr_auto_auto] gap-3 items-end" {
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5" { "Key" }
                                input
                                    type="text"
                                    name="key"
                                    placeholder="VARIABLE_NAME"
                                    autocomplete="off"
                                    class="w-full h-10 px-3.5 text-sm font-mono uppercase \
                                           bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                                           text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                                           outline-none transition-all duration-150 \
                                           hover:border-[var(--border-strong)] \
                                           focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)]";
                            }
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5" { "Value" }
                                (ui::input_password("value", "Secret value"))
                            }
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5 invisible" { "Preview" }
                                div class="h-10 flex items-center" {
                                    (ui::checkbox("preview_only", "Preview only", false))
                                }
                            }
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5 invisible" { "Add" }
                                (ui::button_submit("Add", ButtonVariant::Primary, ButtonSize::Medium))
                            }
                        }
                    }

                    (ui::card_divider())

                    // Existing env vars list
                    @if env_vars.is_empty() {
                        div class="py-8 text-center" {
                            div class="flex justify-center text-[var(--text-faint)] mb-3" {
                                (icon::key(32))
                            }
                            p class="text-sm text-[var(--text-muted)]" { "No environment variables configured" }
                            p class="text-xs text-[var(--text-faint)] mt-1" { "Add variables above to get started." }
                        }
                    } @else {
                        div class="space-y-1" {
                            // Header row
                            div class="grid grid-cols-[1fr_1fr_auto_auto] gap-3 px-3 py-2 text-xs font-medium text-[var(--text-faint)] uppercase tracking-wider" {
                                span { "Key" }
                                span { "Value" }
                                span { "Scope" }
                                span { "" }
                            }
                            
                            @for env_var in &env_vars {
                                (env_var_row(&project.id, env_var, &state.encryption_key))
                            }
                        }
                    }
                }))

                // Danger Zone
                (ui::card(html! {
                    div class="flex items-start gap-3" {
                        div class="flex-shrink-0 w-8 h-8 rounded-lg bg-red-500/10 flex items-center justify-center text-red-400" {
                            (icon::trash(16))
                        }
                        div class="flex-1" {
                            h2 class="text-sm font-medium text-red-400" { "Danger Zone" }
                            p class="text-xs text-[var(--text-muted)] mt-1" { 
                                "Permanently delete this project and all its deployments. This action cannot be undone."
                            }
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
        div class="group grid grid-cols-[1fr_1fr_auto_auto] gap-3 items-center px-3 py-2.5 rounded-lg hover:bg-[var(--bg-surface)] transition-colors" {
            // Key name
            code class="text-sm font-mono text-[var(--text-primary)] truncate" { (&env_var.key) }

            // Masked value
            code class="text-sm font-mono text-[var(--text-faint)] truncate" { (masked) }

            // Preview only badge or All badge
            @if env_var.is_preview_only {
                (ui::badge("Preview", BadgeVariant::Accent))
            } @else {
                (ui::badge("All", BadgeVariant::Default))
            }

            // Delete button
            form
                method="POST"
                action=(format!("/projects/{}/settings/env/{}/delete", project_id, env_var.id))
            {
                button
                    type="submit"
                    class="p-1.5 rounded-md text-[var(--text-faint)] hover:text-red-400 hover:bg-red-500/10 transition-colors opacity-0 group-hover:opacity-100"
                    title="Delete"
                {
                    (icon::trash(14))
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
