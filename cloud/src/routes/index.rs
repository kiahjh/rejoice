use crate::components::{self as ui, icon};
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
        // Hero section with distinctive visual
        div class="relative overflow-hidden" {
            // Decorative grid pattern
            div class="absolute inset-0 opacity-[0.02]" {
                div class="absolute inset-0" style="background-image: linear-gradient(var(--text-primary) 1px, transparent 1px), linear-gradient(90deg, var(--text-primary) 1px, transparent 1px); background-size: 60px 60px;" {}
            }
            
            // Accent gradient orb
            div class="absolute top-0 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[600px] opacity-30" {
                div class="absolute inset-0 rounded-full bg-gradient-to-b from-amber-500/20 via-amber-600/10 to-transparent blur-3xl" {}
            }
            
            div class="relative max-w-3xl mx-auto px-6 pt-24 pb-20 text-center" {
                // Badge
                div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-[var(--bg-surface)] border border-[var(--border-default)] mb-8 animate-fade-in" {
                    span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse" {}
                    span class="text-xs font-medium text-[var(--text-muted)]" { "Now in beta" }
                }
                
                // Main headline
                h1 class="text-5xl md:text-6xl font-bold text-[var(--text-primary)] tracking-tight leading-[1.1] animate-slide-up" {
                    "Deploy "
                    span class="gradient-text" { "Rejoice" }
                    " apps"
                    br;
                    "in seconds"
                }
                
                // Subheadline  
                p class="mt-6 text-lg text-[var(--text-secondary)] max-w-lg mx-auto leading-relaxed animate-slide-up" style="animation-delay: 0.1s" {
                    "Connect your GitHub repository. "
                    "Push to deploy. "
                    span class="text-[var(--text-muted)]" { "That's it." }
                }
                
                // CTA
                div class="mt-10 animate-slide-up" style="animation-delay: 0.2s" {
                    (ui::github_button("/auth/github"))
                }
                
                // Trust indicators
                p class="mt-6 text-xs text-[var(--text-faint)] animate-slide-up" style="animation-delay: 0.3s" {
                    "No credit card required"
                    span class="mx-2 opacity-30" { "·" }
                    "Free tier available"
                }
            }
        }
        
        // Features section
        div class="relative border-t border-[var(--border-subtle)]" {
            div class="max-w-4xl mx-auto px-6 py-20" {
                // Section header
                div class="text-center mb-12" {
                    h2 class="text-sm font-medium text-[var(--accent)] uppercase tracking-wider" { "How it works" }
                }
                
                // Feature cards
                div class="grid md:grid-cols-3 gap-8 stagger-children" {
                    // Feature 1
                    (feature_card(
                        icon::rocket(24),
                        "Push to deploy",
                        "Every git push to main triggers a new deployment. Zero configuration required.",
                        "1"
                    ))
                    
                    // Feature 2
                    (feature_card(
                        icon::database(24),
                        "SQLite included",
                        "Your database lives with your app on a persistent volume. No external services.",
                        "2"
                    ))
                    
                    // Feature 3
                    (feature_card(
                        icon::git_branch(24),
                        "Preview deploys",
                        "Every pull request gets its own URL with a copy of your production database.",
                        "3"
                    ))
                }
            }
        }
        
        // How it works section with code
        div class="relative border-t border-[var(--border-subtle)] bg-[var(--bg-base)]" {
            div class="max-w-4xl mx-auto px-6 py-20" {
                div class="grid md:grid-cols-2 gap-12 items-center" {
                    // Left: Text content
                    div {
                        h2 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" {
                            "Built for Rust developers"
                        }
                        p class="mt-4 text-[var(--text-secondary)] leading-relaxed" {
                            "Rejoice Cloud understands your Rejoice project structure. "
                            "We detect your client assets, database needs, and port configuration automatically."
                        }
                        
                        ul class="mt-8 space-y-4" {
                            (check_item("Automatic Dockerfile generation"))
                            (check_item("Environment variable encryption"))
                            (check_item("GitHub commit status checks"))
                            (check_item("Real-time build logs"))
                        }
                    }
                    
                    // Right: Terminal mockup
                    div class="relative" {
                        // Terminal window
                        div class="bg-[var(--bg-elevated)] rounded-xl border border-[var(--border-default)] shadow-2xl overflow-hidden" {
                            // Window header
                            div class="flex items-center gap-2 px-4 py-3 border-b border-[var(--border-subtle)] bg-[var(--bg-surface)]" {
                                span class="w-3 h-3 rounded-full bg-red-500/80" {}
                                span class="w-3 h-3 rounded-full bg-amber-500/80" {}
                                span class="w-3 h-3 rounded-full bg-emerald-500/80" {}
                                span class="ml-3 text-xs text-[var(--text-faint)] font-mono" { "terminal" }
                            }
                            // Terminal content
                            pre class="p-4 text-sm font-mono text-[var(--text-secondary)] overflow-x-auto" {
                                code {
                                    span class="text-[var(--text-faint)]" { "$ " }
                                    span class="text-[var(--accent-light)]" { "git push origin main" }
                                    "\n\n"
                                    span class="text-[var(--text-muted)]" { "# Rejoice Cloud detects the push\n" }
                                    span class="text-emerald-400" { "✓" }
                                    " Building project...\n"
                                    span class="text-emerald-400" { "✓" }
                                    " Running cargo build --release\n"
                                    span class="text-emerald-400" { "✓" }
                                    " Deploying to fly.io\n"
                                    span class="text-emerald-400" { "✓" }
                                    " Live at "
                                    span class="text-[var(--accent)]" { "myapp.rejoice.sh" }
                                }
                            }
                        }
                        
                        // Decorative glow behind terminal
                        div class="absolute -inset-4 -z-10 bg-gradient-to-br from-amber-500/10 via-transparent to-amber-500/5 rounded-2xl blur-xl" {}
                    }
                }
            }
        }
        
        // CTA section
        div class="relative border-t border-[var(--border-subtle)]" {
            div class="max-w-2xl mx-auto px-6 py-20 text-center" {
                h2 class="text-3xl font-bold text-[var(--text-primary)] tracking-tight" {
                    "Ready to deploy?"
                }
                p class="mt-4 text-[var(--text-secondary)]" {
                    "Get your Rejoice app online in under 5 minutes."
                }
                div class="mt-8" {
                    (ui::github_button("/auth/github"))
                }
            }
        }
    })
}

fn feature_card(icon: rejoice::Markup, title: &str, description: &str, number: &str) -> rejoice::Markup {
    html! {
        div class="group relative" {
            // Number badge
            div class="absolute -top-3 -left-3 w-7 h-7 rounded-lg bg-[var(--bg-surface)] border border-[var(--border-default)] flex items-center justify-center text-xs font-mono text-[var(--text-faint)]" {
                (number)
            }
            
            // Card
            div class="h-full p-6 rounded-xl bg-[var(--bg-elevated)] border border-[var(--border-subtle)] transition-all duration-300 group-hover:border-[var(--border-default)] group-hover:shadow-lg" {
                // Icon
                div class="w-10 h-10 rounded-lg bg-gradient-to-br from-amber-500/20 to-amber-600/10 border border-amber-500/20 flex items-center justify-center text-amber-400 mb-4" {
                    (icon)
                }
                
                h3 class="text-base font-medium text-[var(--text-primary)]" { (title) }
                p class="mt-2 text-sm text-[var(--text-muted)] leading-relaxed" { (description) }
            }
        }
    }
}

fn check_item(text: &str) -> rejoice::Markup {
    html! {
        li class="flex items-center gap-3" {
            span class="flex-shrink-0 w-5 h-5 rounded-full bg-emerald-500/20 flex items-center justify-center" {
                svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" class="text-emerald-400" {
                    polyline points="20 6 9 17 4 12" {}
                }
            }
            span class="text-sm text-[var(--text-secondary)]" { (text) }
        }
    }
}
