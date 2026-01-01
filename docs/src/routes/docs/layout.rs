use rejoice::{html, Children, Req, Res};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    let current_path = req.uri.path();

    res.html(html! {
        // Navigation
        nav class="fixed top-0 left-0 right-0 z-50 border-b backdrop-blur-xl"
            style="border-color: var(--line); background: linear-gradient(180deg, rgba(10,9,8,0.95), rgba(10,9,8,0.85));" {
            div class="px-6 h-16 flex items-center justify-between" {
                div class="flex items-center gap-4" {
                    // Mobile menu button
                    button
                        class="md:hidden p-2 rounded-lg transition-colors cursor-pointer hover:bg-white/5"
                        style="color: var(--ink-soft);"
                        onclick="document.getElementById('mobile-sidebar').classList.toggle('hidden')"
                    {
                        svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" {
                            path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" {}
                        }
                    }
                    a href="/" class="text-2xl" {
                        span class="hero-title-accent" { "Rejoice" }
                    }
                }
                div class="flex items-center gap-6" {
                    a href="/docs" class="text-sm font-medium" style="color: var(--ember-bright);" { "Docs" }
                    a href="/llms" class="nav-link text-sm font-medium py-1" style="color: var(--ink-soft);" { "LLMs" }
                    a href="https://github.com/kiahjh/rejoice" target="_blank" class="nav-link text-sm font-medium py-1" style="color: var(--ink-soft);" {
                        "GitHub"
                    }
                }
            }
        }

        // Mobile sidebar overlay
        div id="mobile-sidebar" class="hidden md:hidden fixed inset-0 z-40 pt-16" {
            div
                class="absolute inset-0 backdrop-blur-sm"
                style="background: rgba(10, 9, 8, 0.9);"
                onclick="document.getElementById('mobile-sidebar').classList.add('hidden')"
            {}
            aside class="absolute left-0 top-0 bottom-0 w-72 pt-16 overflow-y-auto" 
                style="background: var(--void); border-right: 1px solid var(--line);" {
                nav class="p-6" {
                    (sidebar_content(current_path))
                }
            }
        }

        // Main content with sidebar
        div class="flex min-h-screen pt-16" style="background: var(--void);" {
            // Desktop sidebar
            aside class="hidden md:block w-72 fixed left-0 top-16 bottom-0 overflow-y-auto"
                style="background: var(--void); border-right: 1px solid var(--line);" {
                nav class="p-6" {
                    (sidebar_content(current_path))
                }
            }

            // Content area
            main class="flex-1 md:ml-72 min-w-0" {
                div class="max-w-3xl mx-auto px-8 py-16" {
                    article class="prose" {
                        (children)
                    }
                }
            }
        }
    })
}

fn sidebar_content(current_path: &str) -> rejoice::Markup {
    html! {
        (sidebar_section("Getting Started", &[
            ("/docs", "Introduction"),
            ("/docs/installation", "Installation"),
            ("/docs/project-structure", "Project Structure"),
        ], current_path))

        (sidebar_section("Core Concepts", &[
            ("/docs/routing", "Routing"),
            ("/docs/layouts", "Layouts"),
            ("/docs/templates", "HTML Templates"),
        ], current_path))

        (sidebar_section("Request & Response", &[
            ("/docs/request", "Request Object"),
            ("/docs/response", "Response Object"),
        ], current_path))

        (sidebar_section("Features", &[
            ("/docs/islands", "SolidJS Islands"),
            ("/docs/tailwind", "Tailwind CSS"),
            ("/docs/database", "Database"),
            ("/docs/static-assets", "Static Assets"),
        ], current_path))

        (sidebar_section("Reference", &[
            ("/docs/cli", "CLI Commands"),
            ("/docs/deployment", "Deployment"),
        ], current_path))
    }
}

fn sidebar_section(title: &str, links: &[(&str, &str)], current_path: &str) -> rejoice::Markup {
    html! {
        div class="mb-8" {
            h3 class="text-xs font-medium uppercase tracking-widest mb-4 px-3" style="color: var(--ink-ghost);" {
                (title)
            }
            ul class="space-y-1" {
                @for (href, label) in links {
                    @let is_active = current_path == *href;
                    li {
                        a
                            href=(*href)
                            class={
                                "sidebar-link block px-3 py-2 rounded-lg text-sm transition-all duration-200"
                                @if is_active { " active" }
                            }
                            style={
                                @if is_active {
                                    "background: var(--ember-whisper); color: var(--ember-bright);"
                                } @else {
                                    "color: var(--ink-soft);"
                                }
                            }
                        {
                            (*label)
                        }
                    }
                }
            }
        }
    }
}
