use crate::markdown::code_block_with_filename;
use rejoice::{html, Markup, Req, Res};

pub async fn page(req: Req, res: Res) -> Res {
    let _ = req;

    res.html(html! {
        // Navigation
        nav class="fixed top-0 left-0 right-0 z-50 border-b backdrop-blur-xl" 
            style="border-color: var(--line); background: linear-gradient(180deg, rgba(10,9,8,0.95), rgba(10,9,8,0.85));" {
            div class="max-w-6xl mx-auto px-6 h-16 flex items-center justify-between" {
                a href="/" class="text-2xl" {
                    span class="hero-title-accent" { "Rejoice" }
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
                    div class="text-center mb-20" {
                        p class="text-sm uppercase tracking-widest mb-4" style="color: var(--ember);" { "Simplicity" }
                        h2 class="text-3xl md:text-4xl mb-6" style="color: var(--ink-bright); font-family: 'Instrument Serif', Georgia, serif; font-style: italic;" {
                            "Elegant by design"
                        }
                        p class="text-lg max-w-xl mx-auto" style="color: var(--ink-soft);" {
                            "Write clean, expressive code. Every file in your routes directory becomes a page."
                        }
                    }

                    div class="grid lg:grid-cols-2 gap-8" {
                        div class="min-w-0" {
                            (code_block_with_filename(r#"use rejoice::{Req, Res, html};

pub async fn page(req: Req, res: Res) -> Res {
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
                    div class="text-center mb-20" {
                        p class="text-sm uppercase tracking-widest mb-4" style="color: var(--ember);" { "Features" }
                        h2 class="text-3xl md:text-4xl mb-6" style="color: var(--ink-bright); font-family: 'Instrument Serif', Georgia, serif; font-style: italic;" {
                            "Everything you need"
                        }
                        p class="text-lg max-w-xl mx-auto" style="color: var(--ink-soft);" {
                            "A complete toolkit for building modern web applications."
                        }
                    }

                    div class="grid md:grid-cols-2 lg:grid-cols-3 gap-7" {
                        (feature_card("file-routes", "File-based Routing", "Drop a file in src/routes/ and it becomes a route. Nested layouts and dynamic parameters included."))
                        (feature_card("type-safe", "Type-safe Templates", "Compile-time HTML with Maud. Catch errors before runtime, enjoy fearless refactoring."))
                        (feature_card("islands", "SolidJS Islands", "Add interactivity where you need it. Server-render everything else for blazing speed."))
                        (feature_card("tailwind", "Tailwind CSS v4", "Utility-first CSS that scans your Rust and TSX files automatically."))
                        (feature_card("database", "SQLite Ready", "Optional database support with connection pooling. Just add a flag."))
                        (feature_card("live-reload", "Live Reload", "Instant feedback. Changes to Rust, TSX, or CSS reflect immediately."))
                    }
                }
            }

            // Getting started
            section class="py-32 px-6 relative overflow-hidden" style="background: var(--surface-1);" {
                div class="absolute top-0 left-1/2 -translate-x-1/2 w-[1000px] h-[500px] rounded-full blur-3xl" style="background: var(--ember); opacity: 0.03;" {}

                div class="relative z-10 max-w-2xl mx-auto" {
                    div class="text-center mb-14" {
                        p class="text-sm uppercase tracking-widest mb-4" style="color: var(--ember);" { "Quick Start" }
                        h2 class="text-3xl md:text-4xl mb-6" style="color: var(--ink-bright); font-family: 'Instrument Serif', Georgia, serif; font-style: italic;" {
                            "Begin in seconds"
                        }
                        p class="text-lg" style="color: var(--ink-soft);" {
                            "Three commands to your first app."
                        }
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
                        
                        p style="color: var(--ink-ghost);" {
                            "Crafted with "
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

fn feature_card(icon: &str, title: &str, description: &str) -> Markup {
    let icon_svg = match icon {
        "file-routes" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 00-1.883 2.542l.857 6a2.25 2.25 0 002.227 1.932H19.05a2.25 2.25 0 002.227-1.932l.857-6a2.25 2.25 0 00-1.883-2.542m-16.5 0V6A2.25 2.25 0 016 3.75h3.879a1.5 1.5 0 011.06.44l2.122 2.12a1.5 1.5 0 001.06.44H18A2.25 2.25 0 0120.25 9v.776" />"#,
        "type-safe" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z" />"#,
        "islands" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />"#,
        "tailwind" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9.53 16.122a3 3 0 00-5.78 1.128 2.25 2.25 0 01-2.4 2.245 4.5 4.5 0 008.4-2.245c0-.399-.078-.78-.22-1.128zm0 0a15.998 15.998 0 003.388-1.62m-5.043-.025a15.994 15.994 0 011.622-3.395m3.42 3.42a15.995 15.995 0 004.764-4.648l3.876-5.814a1.151 1.151 0 00-1.597-1.597L14.146 6.32a15.996 15.996 0 00-4.649 4.763m3.42 3.42a6.776 6.776 0 00-3.42-3.42" />"#,
        "database" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125" />"#,
        "live-reload" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />"#,
        _ => "",
    };

    html! {
        div class="card p-8 rounded-2xl transition-all duration-500" 
            style="border: 1px solid var(--line);" {
            div class="relative z-10" {
                div class="card-icon w-12 h-12 rounded-xl flex items-center justify-center mb-6"
                    style="background: var(--ember-whisper); border: 1px solid var(--line); color: var(--ember);" {
                    svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" {
                        (rejoice::PreEscaped(icon_svg))
                    }
                }
                h3 class="text-lg font-semibold mb-3" style="color: var(--ink-bright); letter-spacing: -0.01em;" { (title) }
                p class="text-sm leading-relaxed" style="color: var(--ink-soft);" { (description) }
            }
        }
    }
}
