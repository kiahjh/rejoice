use rejoice::{html, Markup};

/// Basic card container with subtle elevation
pub fn card(children: Markup) -> Markup {
    html! {
        div class="bg-[var(--bg-elevated)] border border-[var(--border-subtle)] rounded-xl p-5 shadow-sm" {
            (children)
        }
    }
}

/// Card with prominent styling for key content
pub fn card_prominent(children: Markup) -> Markup {
    html! {
        div class="bg-gradient-to-b from-[var(--bg-surface)] to-[var(--bg-elevated)] border border-[var(--border-default)] rounded-xl p-6 shadow-md" {
            (children)
        }
    }
}

/// Interactive card that's clickable (for lists, etc)
pub fn card_interactive(href: &str, children: Markup) -> Markup {
    html! {
        a
            href=(href)
            class="group block bg-[var(--bg-elevated)] border border-[var(--border-subtle)] rounded-xl p-5 \
                   no-underline transition-all duration-200 cursor-pointer \
                   hover:bg-[var(--bg-surface)] hover:border-[var(--border-default)] \
                   hover:shadow-md hover:-translate-y-0.5"
        {
            (children)
        }
    }
}

/// Project card with status indicator
pub fn project_card(href: &str, name: &str, repo: &str, is_deployed: bool) -> Markup {
    html! {
        a
            href=(href)
            class="group block bg-[var(--bg-elevated)] border border-[var(--border-subtle)] rounded-xl p-5 \
                   no-underline transition-all duration-200 cursor-pointer \
                   hover:bg-[var(--bg-surface)] hover:border-[var(--border-default)] \
                   hover:shadow-md hover:-translate-y-0.5"
        {
            div class="flex items-start justify-between gap-4" {
                div class="min-w-0 flex-1" {
                    // Project name with arrow on hover
                    div class="flex items-center gap-2" {
                        h3 class="text-base font-medium text-[var(--text-primary)] truncate group-hover:text-[var(--accent-light)] transition-colors" {
                            (name)
                        }
                        // Arrow that appears on hover
                        span class="text-[var(--text-faint)] opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-200" {
                            "→"
                        }
                    }
                    // Repo path
                    p class="mt-1.5 text-sm text-[var(--text-muted)] truncate font-mono" {
                        (repo)
                    }
                }
                // Status indicator
                @if is_deployed {
                    (status_indicator(StatusVariant::Success, Some("Live")))
                }
            }
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum BadgeVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
    Accent,
}

impl BadgeVariant {
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => {
                "bg-[var(--bg-surface)] text-[var(--text-muted)] border-[var(--border-default)]"
            }
            BadgeVariant::Success => {
                "bg-[var(--success-bg)] text-emerald-400 border-emerald-900/30"
            }
            BadgeVariant::Warning => "bg-[var(--warning-bg)] text-amber-400 border-amber-900/30",
            BadgeVariant::Error => "bg-[var(--error-bg)] text-red-400 border-red-900/30",
            BadgeVariant::Accent => {
                "bg-[var(--accent-subtle)] text-[var(--accent-light)] border-amber-900/30"
            }
        }
    }
}

pub fn badge(label: &str, variant: BadgeVariant) -> Markup {
    html! {
        span class=(format!("inline-flex items-center px-2 py-0.5 text-xs font-medium rounded-md border {}", variant.classes())) {
            (label)
        }
    }
}

/// Badge with icon
pub fn badge_with_icon(label: &str, icon: Markup, variant: BadgeVariant) -> Markup {
    html! {
        span class=(format!("inline-flex items-center gap-1.5 px-2 py-0.5 text-xs font-medium rounded-md border {}", variant.classes())) {
            span class="opacity-70" { (icon) }
            (label)
        }
    }
}

#[derive(Clone, Copy)]
pub enum StatusVariant {
    Success,
    Warning,
    Error,
    Neutral,
    Building,
}

impl StatusVariant {
    fn dot_classes(&self) -> &'static str {
        match self {
            StatusVariant::Success => "bg-emerald-500 shadow-emerald-500/50",
            StatusVariant::Warning => "bg-amber-500 shadow-amber-500/50",
            StatusVariant::Error => "bg-red-500 shadow-red-500/50",
            StatusVariant::Neutral => "bg-[var(--text-faint)]",
            StatusVariant::Building => "bg-amber-500 shadow-amber-500/50 animate-pulse",
        }
    }

    fn text_classes(&self) -> &'static str {
        match self {
            StatusVariant::Success => "text-emerald-400",
            StatusVariant::Warning => "text-amber-400",
            StatusVariant::Error => "text-red-400",
            StatusVariant::Neutral => "text-[var(--text-muted)]",
            StatusVariant::Building => "text-amber-400",
        }
    }
}

/// Simple status dot
pub fn status_dot(variant: StatusVariant) -> Markup {
    html! {
        span class=(format!("inline-block w-2 h-2 rounded-full shadow-sm {}", variant.dot_classes())) {}
    }
}

/// Status indicator with optional label
pub fn status_indicator(variant: StatusVariant, label: Option<&str>) -> Markup {
    html! {
        div class="flex items-center gap-2" {
            span class=(format!("w-2 h-2 rounded-full shadow-sm {}", variant.dot_classes())) {}
            @if let Some(text) = label {
                span class=(format!("text-xs font-medium {}", variant.text_classes())) { (text) }
            }
        }
    }
}

/// Section header within a card
pub fn card_header(title: &str, description: Option<&str>) -> Markup {
    html! {
        div class="mb-5" {
            h2 class="text-sm font-medium text-[var(--text-primary)]" { (title) }
            @if let Some(desc) = description {
                p class="mt-1 text-xs text-[var(--text-muted)]" { (desc) }
            }
        }
    }
}

/// Divider within cards
pub fn card_divider() -> Markup {
    html! {
        div class="h-px bg-[var(--border-subtle)] my-5" {}
    }
}

/// Key-value row for details sections
pub fn detail_row(label: &str, value: Markup) -> Markup {
    html! {
        div class="flex items-center justify-between py-2" {
            dt class="text-sm text-[var(--text-muted)]" { (label) }
            dd class="text-sm text-[var(--text-secondary)]" { (value) }
        }
    }
}

/// Empty state for when there's no content
pub fn empty_state(
    icon: Option<Markup>,
    title: &str,
    description: &str,
    action: Option<Markup>,
) -> Markup {
    html! {
        div class="flex flex-col items-center justify-center py-12 px-6 text-center" {
            @if let Some(icon_markup) = icon {
                div class="mb-4 text-[var(--text-faint)]" {
                    (icon_markup)
                }
            }
            h3 class="text-base font-medium text-[var(--text-secondary)]" { (title) }
            p class="mt-1.5 text-sm text-[var(--text-muted)] max-w-sm" { (description) }
            @if let Some(action_markup) = action {
                div class="mt-6" {
                    (action_markup)
                }
            }
        }
    }
}
