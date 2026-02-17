use crate::components::{self as ui, ButtonSize, ButtonVariant};
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, Children, Req, Res, DOCTYPE};

#[derive(FromRow)]
struct User {
    github_username: String,
}

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    let user = if let Some(github_id) = req.cookies.get("session") {
        if let Ok(github_id) = github_id.parse::<i64>() {
            query_as::<_, User>("SELECT github_username FROM users WHERE github_id = ?")
                .bind(github_id)
                .fetch_optional(&state.db)
                .await
                .ok()
                .flatten()
        } else {
            None
        }
    } else {
        None
    };

    res.html(html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Rejoice Cloud" }
                link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>◈</text></svg>";
            }
            body class="min-h-screen antialiased relative flex flex-col" {
                // Header with subtle bottom border glow
                header class="sticky top-0 z-50 backdrop-blur-xl bg-[var(--bg-deep)]/80 border-b border-[var(--border-subtle)]" {
                    div class="max-w-5xl mx-auto px-6" {
                        div class="h-16 flex items-center justify-between" {
                            // Logo with icon
                            a href="/" class="flex items-center gap-2.5 text-[var(--text-primary)] font-medium no-underline group" {
                                // Logo mark - abstract cloud/rejoice shape
                                span class="flex items-center justify-center w-8 h-8 rounded-lg bg-gradient-to-br from-amber-500 to-amber-700 text-white text-sm shadow-lg shadow-amber-900/20 group-hover:shadow-amber-900/40 transition-shadow" {
                                    "◈"
                                }
                                span class="tracking-tight" { "Rejoice Cloud" }
                            }

                            // Nav
                            nav class="flex items-center gap-1" {
                                @if let Some(user) = &user {
                                    // Navigation links
                                    a 
                                        href="/projects" 
                                        class="px-3 py-2 text-sm text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)] rounded-lg transition-all duration-150 no-underline"
                                    {
                                        "Projects"
                                    }
                                    
                                    // Divider
                                    span class="w-px h-5 bg-[var(--border-subtle)] mx-3" {}
                                    
                                    // User menu
                                    div class="flex items-center gap-3" {
                                        // Avatar placeholder
                                        div class="flex items-center gap-2.5 px-2 py-1.5 rounded-lg" {
                                            div class="w-7 h-7 rounded-full bg-gradient-to-br from-[var(--bg-surface)] to-[var(--bg-hover)] border border-[var(--border-default)] flex items-center justify-center text-xs text-[var(--text-muted)] font-medium" {
                                                (user.github_username.chars().next().unwrap_or('?').to_uppercase().to_string())
                                            }
                                            span class="text-sm text-[var(--text-muted)]" { (&user.github_username) }
                                        }
                                        (ui::button_link("/auth/logout", "Log out", ButtonVariant::Ghost, ButtonSize::Small))
                                    }
                                } @else {
                                    (ui::button_link("/auth/github", "Sign in", ButtonVariant::Secondary, ButtonSize::Small))
                                }
                            }
                        }
                    }
                }

                // Main content with relative positioning for proper stacking
                main class="relative z-10 flex-1" {
                    (children)
                }

                // Footer
                footer class="relative z-10 border-t border-[var(--border-subtle)] mt-auto" {
                    div class="max-w-5xl mx-auto px-6 py-8" {
                        div class="flex items-center justify-between" {
                            p class="text-sm text-[var(--text-faint)]" {
                                "Built with "
                                a href="https://rejoice.sh" target="_blank" class="text-[var(--text-muted)] hover:text-[var(--accent)] no-underline transition-colors" { "Rejoice" }
                            }
                            div class="flex items-center gap-4" {
                                a href="https://github.com/rejoice-sh/rejoice" target="_blank" class="text-[var(--text-faint)] hover:text-[var(--text-muted)] no-underline transition-colors" {
                                    (ui::icon::github(18))
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
