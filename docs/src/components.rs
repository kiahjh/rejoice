use maud::Render;
use rejoice::{Markup, PreEscaped, PropEnum, component, html};

// =============================================================================
// Logo
// =============================================================================

/// The Rejoice logo SVG
#[component]
pub fn Logo(
    /// CSS classes for sizing (e.g., "w-7 h-7")
    size: &str,
) -> Markup {
    html! {
        svg class=(size) viewBox="0 0 24 24" fill="none" style="color: var(--ember-bright);" {
            circle cx="12" cy="12" r="2" fill="currentColor" {}
            line x1="12" y1="2" x2="12" y2="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" {}
            line x1="12" y1="18" x2="12" y2="22" stroke="currentColor" stroke-width="2" stroke-linecap="round" {}
            line x1="2" y1="12" x2="6" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round" {}
            line x1="18" y1="12" x2="22" y2="12" stroke="currentColor" stroke-width="2" stroke-linecap="round" {}
            line x1="4.93" y1="4.93" x2="7.76" y2="7.76" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" {}
            line x1="16.24" y1="16.24" x2="19.07" y2="19.07" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" {}
            line x1="4.93" y1="19.07" x2="7.76" y2="16.24" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" {}
            line x1="16.24" y1="7.76" x2="19.07" y2="4.93" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" {}
        }
    }
}

// =============================================================================
// Section Header
// =============================================================================

/// A section header with eyebrow text, title, and optional subtitle
#[component]
pub fn SectionHeader(
    /// Small uppercase text above the title
    eyebrow: &str,
    /// Main heading text
    title: &str,
    /// Optional description below the title
    #[prop(default = None)]
    subtitle: Option<&str>,
) -> Markup {
    html! {
        div class="text-center mb-20" {
            p class="text-sm uppercase tracking-widest mb-4" style="color: var(--ember);" { (eyebrow) }
            h2 class="text-3xl md:text-4xl mb-6" style="color: var(--ink-bright); font-family: 'Instrument Serif', Georgia, serif; font-style: italic;" {
                (title)
            }
            @if let Some(sub) = subtitle {
                p class="text-lg max-w-xl mx-auto" style="color: var(--ink-soft);" {
                    (sub)
                }
            }
        }
    }
}

// =============================================================================
// Feature Card
// =============================================================================

/// Icon type for feature cards
#[derive(Clone, Copy, PropEnum)]
pub enum FeatureIcon {
    FileRoutes,
    TypeSafe,
    Islands,
    Tailwind,
    Database,
    LiveReload,
}

impl FeatureIcon {
    fn svg_path(&self) -> &'static str {
        match self {
            Self::FileRoutes => {
                r#"<path stroke-linecap="round" stroke-linejoin="round" d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 00-1.883 2.542l.857 6a2.25 2.25 0 002.227 1.932H19.05a2.25 2.25 0 002.227-1.932l.857-6a2.25 2.25 0 00-1.883-2.542m-16.5 0V6A2.25 2.25 0 016 3.75h3.879a1.5 1.5 0 011.06.44l2.122 2.12a1.5 1.5 0 001.06.44H18A2.25 2.25 0 0120.25 9v.776" />"#
            }
            Self::TypeSafe => {
                r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z" />"#
            }
            Self::Islands => {
                r#"<path stroke-linecap="round" stroke-linejoin="round" d="M21 7.5l-2.25-1.313M21 7.5v2.25m0-2.25l-2.25 1.313M3 7.5l2.25-1.313M3 7.5l2.25 1.313M3 7.5v2.25m9 3l2.25-1.313M12 12.75l-2.25-1.313M12 12.75V15m0 6.75l2.25-1.313M12 21.75V19.5m0 2.25l-2.25-1.313m0-16.875L12 2.25l2.25 1.313M21 14.25v2.25l-2.25 1.313m-13.5 0L3 16.5v-2.25" />"#
            }
            Self::Tailwind => {
                r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9.53 16.122a3 3 0 00-5.78 1.128 2.25 2.25 0 01-2.4 2.245 4.5 4.5 0 008.4-2.245c0-.399-.078-.78-.22-1.128zm0 0a15.998 15.998 0 003.388-1.62m-5.043-.025a15.994 15.994 0 011.622-3.395m3.42 3.42a15.995 15.995 0 004.764-4.648l3.876-5.814a1.151 1.151 0 00-1.597-1.597L14.146 6.32a15.996 15.996 0 00-4.649 4.763m3.42 3.42a6.776 6.776 0 00-3.42-3.42" />"#
            }
            Self::Database => {
                r#"<path stroke-linecap="round" stroke-linejoin="round" d="M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125" />"#
            }
            Self::LiveReload => {
                r#"<path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />"#
            }
        }
    }
}

/// A feature card with icon, title, and description
#[component]
pub fn FeatureCard(
    /// The icon to display
    icon: FeatureIcon,
    /// Card title
    title: &str,
    /// Card description
    description: &str,
) -> Markup {
    html! {
        div class="card p-8 rounded-2xl transition-all duration-500"
            style="border: 1px solid var(--line);" {
            div class="relative z-10" {
                div class="card-icon w-12 h-12 rounded-xl flex items-center justify-center mb-6"
                    style="background: var(--ember-whisper); border: 1px solid var(--line); color: var(--ember);" {
                    svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" {
                        (PreEscaped(icon.svg_path()))
                    }
                }
                h3 class="text-lg font-semibold mb-3" style="color: var(--ink-bright); letter-spacing: -0.01em;" { (title) }
                p class="text-sm leading-relaxed" style="color: var(--ink-soft);" { (description) }
            }
        }
    }
}

// =============================================================================
// Sidebar Section (for docs)
// =============================================================================

/// A sidebar navigation section with title and links
#[component]
pub fn SidebarSection(
    /// Section title
    title: &str,
    /// List of (href, label) tuples
    links: &[(&str, &str)],
    /// Current page path for highlighting active link
    current_path: &str,
) -> Markup {
    html! {
        div class="mb-8" {
            h3 class="text-xs font-medium uppercase tracking-widest mb-4 px-3" style="color: var(--ink-ghost);" {
                (title)
            }
            ul class="space-y-1" {
                @for (href, label) in links.iter() {
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

// =============================================================================
// Version Badge
// =============================================================================

/// A small version badge
#[component]
pub fn VersionBadge(
    /// Version string to display
    version: &str,
) -> Markup {
    html! {
        span class="text-xs font-medium px-2 py-0.5 rounded-full"
            style="background: var(--ember-whisper); color: var(--ember-bright); border: 1px solid var(--line);" {
            "v" (version)
        }
    }
}

// =============================================================================
// Section (with children)
// =============================================================================

/// Background style for sections
#[derive(Clone, Copy, Default, PropEnum)]
pub enum SectionBackground {
    #[default]
    Void,
    Surface,
}

impl SectionBackground {
    fn style(&self) -> &'static str {
        match self {
            Self::Void => "background: var(--void);",
            Self::Surface => "background: var(--surface-1);",
        }
    }
}

/// A page section with optional header and children content
#[component]
pub fn Section(
    /// Section eyebrow text (small text above title)
    #[prop(default = None)]
    eyebrow: Option<&str>,
    /// Section title
    #[prop(default = None)]
    title: Option<&str>,
    /// Section subtitle
    #[prop(default = None)]
    subtitle: Option<&str>,
    /// Background style
    #[prop(default)]
    background: SectionBackground,
    /// Additional CSS classes
    #[prop(default = "")]
    extra_class: &str,
    /// Section content
    #[prop(default = html!{})]
    children: Markup,
) -> Markup {
    let has_header = eyebrow.is_some() || title.is_some();
    let section_class = format!("py-32 px-6 {}", extra_class);

    html! {
        section
            class=(section_class)
            style=(background.style())
        {
            div class="max-w-6xl mx-auto" {
                @if has_header {
                    (SectionHeader::new(
                        eyebrow.unwrap_or(""),
                        title.unwrap_or("")
                    ).subtitle(subtitle))
                }
                (children)
            }
        }
    }
}

// =============================================================================
// Card (with children)
// =============================================================================

/// A generic card component with optional title and children
#[component]
pub fn Card(
    /// Optional card title
    #[prop(default = None)]
    title: Option<&str>,
    /// Additional CSS classes
    #[prop(default = "")]
    extra_class: &str,
    /// Card content
    #[prop(default = html!{})]
    children: Markup,
) -> Markup {
    let card_class = format!("p-8 rounded-2xl {}", extra_class);
    html! {
        div
            class=(card_class)
            style="border: 1px solid var(--line);"
        {
            @if let Some(t) = title {
                h3 class="text-lg font-semibold mb-4" style="color: var(--ink-bright);" { (t) }
            }
            (children)
        }
    }
}

// =============================================================================
// Legacy function exports for backwards compatibility
// =============================================================================

/// Legacy function - use Logo component instead
pub fn logo(size: &str) -> Markup {
    Logo::new(size).render()
}
