use rejoice::{html, Markup};

/// Back link for navigation
pub fn back_link(href: &str, label: &str) -> Markup {
    html! {
        a
            href=(href)
            class="inline-flex items-center gap-1.5 text-sm text-[var(--text-muted)] hover:text-[var(--text-primary)] no-underline transition-colors group"
        {
            span class="transition-transform group-hover:-translate-x-0.5" { "←" }
            (label)
        }
    }
}

/// Page header with title and optional actions
pub fn page_header(title: &str, actions: Option<Markup>) -> Markup {
    html! {
        div class="flex items-center justify-between mb-8" {
            h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { (title) }
            @if let Some(content) = actions {
                (content)
            }
        }
    }
}

/// Page header with title, subtitle, and optional actions
pub fn page_header_with_subtitle(title: &str, subtitle: &str, actions: Option<Markup>) -> Markup {
    html! {
        div class="flex items-start justify-between mb-8" {
            div {
                h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { (title) }
                p class="mt-1 text-sm text-[var(--text-muted)]" { (subtitle) }
            }
            @if let Some(content) = actions {
                div class="flex-shrink-0 ml-4" {
                    (content)
                }
            }
        }
    }
}

/// Simple empty state (text only)
pub fn simple_empty_state(message: &str) -> Markup {
    html! {
        div class="text-center py-16" {
            p class="text-[var(--text-muted)]" { (message) }
        }
    }
}

/// Section divider with optional label
pub fn section_divider(label: Option<&str>) -> Markup {
    html! {
        @if let Some(text) = label {
            div class="relative my-8" {
                div class="absolute inset-0 flex items-center" {
                    div class="w-full border-t border-[var(--border-subtle)]" {}
                }
                div class="relative flex justify-center" {
                    span class="px-3 text-xs font-medium text-[var(--text-faint)] bg-[var(--bg-deep)]" {
                        (text)
                    }
                }
            }
        } @else {
            div class="h-px bg-[var(--border-subtle)] my-8" {}
        }
    }
}

/// Container for main content area
pub fn page_container(children: Markup) -> Markup {
    html! {
        div class="max-w-4xl mx-auto px-6 py-10" {
            (children)
        }
    }
}

/// Narrower container for forms/focused content
pub fn narrow_container(children: Markup) -> Markup {
    html! {
        div class="max-w-xl mx-auto px-6 py-10" {
            (children)
        }
    }
}

/// Grid for cards/items
pub fn card_grid(children: Markup) -> Markup {
    html! {
        div class="grid gap-4 stagger-children" {
            (children)
        }
    }
}

/// Two-column grid for larger screens
pub fn two_column_grid(children: Markup) -> Markup {
    html! {
        div class="grid md:grid-cols-2 gap-4 stagger-children" {
            (children)
        }
    }
}

/// Three-column grid for feature sections
pub fn three_column_grid(children: Markup) -> Markup {
    html! {
        div class="grid md:grid-cols-3 gap-6 stagger-children" {
            (children)
        }
    }
}

/// Stack with consistent spacing
pub fn stack(children: Markup) -> Markup {
    html! {
        div class="space-y-4" {
            (children)
        }
    }
}

/// Stack with larger spacing
pub fn stack_lg(children: Markup) -> Markup {
    html! {
        div class="space-y-6" {
            (children)
        }
    }
}

/// Inline flex row with items centered
pub fn row(children: Markup) -> Markup {
    html! {
        div class="flex items-center gap-3" {
            (children)
        }
    }
}

/// Row with space between items
pub fn row_between(children: Markup) -> Markup {
    html! {
        div class="flex items-center justify-between gap-4" {
            (children)
        }
    }
}
