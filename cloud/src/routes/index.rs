use crate::components::{self as ui};
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, Req, Res};

#[derive(FromRow)]
struct User {
    #[allow(dead_code)]
    id: String,
}

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let user = if let Some(github_id) = req.cookies.get("session") {
        if let Ok(github_id) = github_id.parse::<i64>() {
            query_as::<_, User>("SELECT id FROM users WHERE github_id = ?")
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

    if user.is_some() {
        return res.redirect("/projects");
    }

    res.html(html! {
        div class="max-w-2xl mx-auto px-6 pt-24 pb-16" {
            // Headline
            h1 class="text-4xl font-semibold text-stone-100 tracking-tight text-center" {
                "Deploy Rejoice apps"
            }
            
            p class="mt-4 text-lg text-stone-400 text-center" {
                "Connect your GitHub repository. We handle the rest."
            }
            
            // CTA
            div class="mt-10 flex justify-center" {
                (ui::github_button("/auth/github"))
            }
        }
        
        // Features
        div class="max-w-3xl mx-auto px-6 py-16" {
            div class="grid grid-cols-1 md:grid-cols-3 gap-8" {
                div {
                    h3 class="text-sm font-medium text-stone-200" { "Push to deploy" }
                    p class="mt-2 text-sm text-stone-500 leading-relaxed" {
                        "Every git push triggers a new deployment automatically."
                    }
                }
                div {
                    h3 class="text-sm font-medium text-stone-200" { "SQLite included" }
                    p class="mt-2 text-sm text-stone-500 leading-relaxed" {
                        "Your database lives with your app. No configuration needed."
                    }
                }
                div {
                    h3 class="text-sm font-medium text-stone-200" { "Preview deploys" }
                    p class="mt-2 text-sm text-stone-500 leading-relaxed" {
                        "Every pull request gets its own URL to review changes."
                    }
                }
            }
        }
    })
}
