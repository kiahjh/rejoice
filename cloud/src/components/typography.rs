use rejoice::{html, Markup};

/// Section heading (h2 equivalent)
pub fn heading(text: &str) -> Markup {
    html! {
        h2 class="text-lg font-semibold text-[var(--text-primary)] tracking-tight" { (text) }
    }
}

/// Small section heading (h3 equivalent)
pub fn subheading(text: &str) -> Markup {
    html! {
        h3 class="text-sm font-medium text-[var(--text-primary)]" { (text) }
    }
}

/// Regular body text
pub fn text(content: &str) -> Markup {
    html! {
        p class="text-sm text-[var(--text-secondary)] leading-relaxed" { (content) }
    }
}

/// Muted/secondary text
pub fn text_muted(content: &str) -> Markup {
    html! {
        p class="text-sm text-[var(--text-muted)]" { (content) }
    }
}

/// Very subtle text
pub fn text_faint(content: &str) -> Markup {
    html! {
        p class="text-sm text-[var(--text-faint)]" { (content) }
    }
}

/// Small helper text
pub fn caption(content: &str) -> Markup {
    html! {
        p class="text-xs text-[var(--text-muted)]" { (content) }
    }
}

/// Inline code
pub fn code(content: &str) -> Markup {
    html! {
        code class="px-1.5 py-0.5 text-xs font-mono bg-[var(--bg-surface)] text-[var(--text-secondary)] rounded" {
            (content)
        }
    }
}

/// Accent/highlighted text
pub fn accent(content: &str) -> Markup {
    html! {
        span class="text-[var(--accent-light)] font-medium" { (content) }
    }
}

/// Large display text for heroes
pub fn display(content: &str) -> Markup {
    html! {
        h1 class="text-4xl md:text-5xl font-bold text-[var(--text-primary)] tracking-tight leading-tight" {
            (content)
        }
    }
}

/// Subtitle for display text
pub fn lead(content: &str) -> Markup {
    html! {
        p class="text-lg text-[var(--text-secondary)] leading-relaxed" {
            (content)
        }
    }
}

/// Link with arrow indicator
pub fn link_arrow(href: &str, text: &str) -> Markup {
    html! {
        a
            href=(href)
            class="inline-flex items-center gap-1.5 text-sm text-[var(--accent)] hover:text-[var(--accent-light)] no-underline transition-colors group"
        {
            (text)
            span class="transition-transform group-hover:translate-x-0.5" { "→" }
        }
    }
}

/// External link
pub fn link_external(href: &str, text: &str) -> Markup {
    html! {
        a
            href=(href)
            target="_blank"
            rel="noopener noreferrer"
            class="inline-flex items-center gap-1 text-sm text-[var(--text-secondary)] hover:text-[var(--text-primary)] no-underline transition-colors"
        {
            (text)
            svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-50" {
                path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" {}
                polyline points="15 3 21 3 21 9" {}
                line x1="10" y1="14" x2="21" y2="3" {}
            }
        }
    }
}
