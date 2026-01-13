use crate::components::{FeatureCard, FeatureIcon, Logo, SectionHeader, VersionBadge};
use crate::markdown::code_block_with_filename;
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;

    res.html(html! {
        // Navigation
        nav class="fixed top-0 left-0 right-0 z-50 border-b backdrop-blur-xl" 
            style="border-color: var(--line); background: linear-gradient(180deg, rgba(10,9,8,0.95), rgba(10,9,8,0.85));" {
            div class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between" {
                a href="/" class="text-2xl flex items-center gap-1.5" {
                    (Logo::new("w-7 h-7"))
                    span class="hero-title-accent" { "Rejoice" }
                    (VersionBadge::new(env!("CARGO_PKG_VERSION")))
                }
                div class="flex items-center gap-8" {
                    a href="/docs" class="nav-link text-sm font-medium py-1" style="color: var(--ink-soft);" { "Docs" }
                    a href="/llms" class="nav-link text-sm font-medium py-1" style="color: var(--ink-soft);" { "LLMs" }
                    a href="https://github.com/kiahjh/rejoice" target="_blank" class="nav-link text-sm font-medium py-1" style="color: var(--ink-soft);" {
                        "GitHub"
                    }
                }
            }
        }

        main class="vignette" {
            // Hero
            section class="relative min-h-screen flex items-center justify-center overflow-hidden" style="background: var(--void);" {
                div class="ambient-orb ambient-orb-1" {}
                div class="ambient-orb ambient-orb-2" {}
                div class="ambient-orb ambient-orb-3" {}

                div class="relative z-10 max-w-4xl mx-auto px-6 text-center" {
                    // Badge
                    div class="inline-flex items-center gap-3 px-5 py-2.5 rounded-full text-sm mb-12"
                        style="background: var(--ember-whisper); border: 1px solid var(--line); color: var(--ember-bright);" {
                        span class="badge-dot w-2 h-2 rounded-full" style="background: var(--ember);" {}
                        span class="tracking-wide" { "Now with Tailwind CSS v4" }
                    }

                    // Title
                    h1 class="mb-8" {
                        span class="block text-5xl md:text-7xl font-semibold mb-4" style="color: var(--ink-bright); letter-spacing: -0.03em; font-family: Inter, sans-serif;" { 
                            "Build web apps" 
                        }
                        span class="hero-title-accent text-5xl md:text-7xl" { "with joy" }
                    }

                    // Subtitle
                    p class="text-xl md:text-2xl max-w-2xl mx-auto mb-14" style="color: var(--ink-soft);" {
                        "A simple and delightful Rust web framework. "
                        "File-based routing, type-safe templates, and islands."
                    }

                    // Buttons
                    div class="flex flex-col sm:flex-row gap-5 justify-center" {
                        a href="/docs" class="btn-primary px-10 py-4 rounded-xl font-semibold text-sm tracking-wide" 
                            style="color: var(--void);" {
                            span class="relative z-10 flex items-center justify-center gap-3" {
                                "Get Started"
                                svg class="w-4 h-4" fill="none" stroke="currentColor" stroke-width="2.5" viewBox="0 0 24 24" {
                                    path stroke-linecap="round" stroke-linejoin="round" d="M13 7l5 5m0 0l-5 5m5-5H6" {}
                                }
                            }
                        }
                        a href="https://github.com/kiahjh/rejoice" target="_blank" 
                            class="btn-secondary px-10 py-4 rounded-xl font-medium text-sm tracking-wide"
                            style="color: var(--ink);" {
                            span class="relative z-10 flex items-center justify-center gap-3" {
                                svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" {
                                    path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" {}
                                }
                                "View on GitHub"
                            }
                        }
                    }
                }

                // Scroll indicator
                div class="scroll-indicator absolute bottom-12 left-1/2 -translate-x-1/2" style="color: var(--ink-ghost);" {
                    svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" {
                        path stroke-linecap="round" stroke-linejoin="round" d="M19 14l-7 7m0 0l-7-7m7 7V3" {}
                    }
                }
            }

            // Code example section
            section class="py-32 px-6 overflow-hidden" style="background: var(--surface-1);" {
                div class="max-w-5xl mx-auto" {
                    (SectionHeader::new("Simplicity", "Elegant by design")
                        .subtitle(Some("Write clean, expressive code. Every file in your routes directory becomes a page.")))

                    div class="grid lg:grid-cols-2 gap-8" {
                        div class="min-w-0" {
                            (code_block_with_filename(r#"use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, world!" }
        p { "Welcome to Rejoice." }
    })
}"#, "rust", Some("src/routes/index.rs")))
                        }
                        div class="min-w-0" {
                            (code_block_with_filename(r#"use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    res.html(html! {
        (DOCTYPE)
        html {
            head { title { "My App" } }
            body { (children) }
        }
    })
}"#, "rust", Some("src/routes/layout.rs")))
                        }
                    }
                }
            }

            // Features section
            section class="py-32 px-6" style="background: var(--void);" {
                div class="max-w-6xl mx-auto" {
                    (SectionHeader::new("Features", "Everything you need")
                        .subtitle(Some("A complete toolkit for building modern web applications.")))

                    div class="grid md:grid-cols-2 lg:grid-cols-3 gap-7" {
                        (FeatureCard::new(FeatureIcon::FileRoutes, "File-based Routing", "Drop a file in src/routes/ and it becomes a route. Nested layouts and dynamic parameters included."))
                        (FeatureCard::new(FeatureIcon::TypeSafe, "Type-safe Templates", "Compile-time HTML with Maud. Catch errors before runtime, enjoy fearless refactoring."))
                        (FeatureCard::new(FeatureIcon::Islands, "SolidJS Islands", "Add interactivity where you need it. Server-render everything else for blazing speed."))
                        (FeatureCard::new(FeatureIcon::Tailwind, "Tailwind CSS v4", "Utility-first CSS that scans your Rust and TSX files automatically."))
                        (FeatureCard::new(FeatureIcon::Database, "SQLite Ready", "Optional database support with connection pooling. Just add a flag."))
                        (FeatureCard::new(FeatureIcon::LiveReload, "Live Reload", "Instant feedback. Changes to Rust, TSX, or CSS reflect immediately."))
                    }
                }
            }

            // Getting started
            section class="py-32 px-6 relative overflow-hidden" style="background: var(--surface-1);" {
                div class="absolute top-0 left-1/2 -translate-x-1/2 w-[1000px] h-[500px] rounded-full blur-3xl" style="background: var(--ember); opacity: 0.03;" {}

                div class="relative z-10 max-w-2xl mx-auto" {
                    div class="mb-14" {
                        (SectionHeader::new("Quick Start", "Begin in seconds")
                            .subtitle(Some("Three commands to your first app.")))
                    }

                    (code_block_with_filename(r#"# Install the CLI
cargo install rejoice

# Create a new project
rejoice init my-app && cd my-app

# Start developing
rejoice dev"#, "bash", None))

                    p class="text-center mt-12" style="color: var(--ink-soft);" {
                        "Your app is running at "
                        code { "localhost:8080" }
                    }
                }
            }

            // Footer
            footer class="py-20 px-6" style="border-top: 1px solid var(--line); background: var(--void);" {
                div class="max-w-6xl mx-auto" {
                    div class="flex flex-col items-center gap-8" {
                        // Ornamental divider
                        div class="flex items-center gap-4" style="color: var(--ink-ghost);" {
                            div class="w-12 h-px" style="background: linear-gradient(90deg, transparent, var(--line));" {}
                            span class="text-xl" style="font-family: 'Instrument Serif', Georgia, serif;" { "~" }
                            div class="w-12 h-px" style="background: linear-gradient(90deg, var(--line), transparent);" {}
                        }
                        
                        p class="flex items-center gap-1" style="color: var(--ink-ghost);" {
                            "Crafted with "
                            (Logo::new("w-4 h-4"))
                            span class="hero-title-accent" { "Rejoice" }
                        }
                        
                        div class="flex items-center gap-8" {
                            a href="/docs" class="nav-link text-sm py-1" style="color: var(--ink-soft);" { "Documentation" }
                            a href="https://github.com/kiahjh/rejoice" target="_blank" class="nav-link text-sm py-1" style="color: var(--ink-soft);" { "GitHub" }
                        }
                    }
                }
            }
        }
    })
}
