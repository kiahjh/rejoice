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
            body class="bg-stone-950 text-stone-100 min-h-screen antialiased" {
                // Header
                header class="border-b border-stone-900" {
                    div class="max-w-5xl mx-auto px-6 h-14 flex items-center justify-between" {
                        // Logo
                        a href="/" class="text-stone-100 font-medium no-underline" {
                            "Rejoice Cloud"
                        }

                        // Nav
                        nav class="flex items-center gap-6" {
                            @if let Some(user) = &user {
                                (ui::nav_link("/projects", "Projects", false))
                                
                                div class="flex items-center gap-4 ml-2 pl-6 border-l border-stone-800" {
                                    span class="text-sm text-stone-500" { (&user.github_username) }
                                    (ui::button_link("/auth/logout", "Log out", ButtonVariant::Ghost, ButtonSize::Small))
                                }
                            } @else {
                                (ui::button_link("/auth/github", "Sign in", ButtonVariant::Secondary, ButtonSize::Small))
                            }
                        }
                    }
                }

                // Main
                main class="relative" {
                    (children)
                }
            }
        }
    })
}
