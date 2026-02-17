use rejoice::{html, Markup};

#[derive(Default, Clone, Copy)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(Default, Clone, Copy)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            // Primary - warm gradient with glow
            ButtonVariant::Primary => "\
                bg-gradient-to-b from-amber-500 to-amber-600 \
                text-white font-medium \
                shadow-md shadow-amber-900/25 \
                hover:from-amber-400 hover:to-amber-500 \
                hover:shadow-lg hover:shadow-amber-900/30 \
                active:from-amber-600 active:to-amber-700 \
                btn-shine",
            // Secondary - subtle surface with border
            ButtonVariant::Secondary => "\
                bg-[var(--bg-surface)] \
                text-[var(--text-secondary)] \
                border border-[var(--border-default)] \
                hover:bg-[var(--bg-hover)] hover:text-[var(--text-primary)] hover:border-[var(--border-strong)] \
                active:bg-[var(--bg-surface)]",
            // Ghost - minimal, just hover state
            ButtonVariant::Ghost => "\
                text-[var(--text-muted)] \
                hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)] \
                active:bg-[var(--bg-elevated)]",
            // Danger - red accent
            ButtonVariant::Danger => "\
                bg-[var(--error-bg)] \
                text-red-400 \
                border border-red-900/50 \
                hover:bg-red-950/50 hover:text-red-300 hover:border-red-800/50 \
                active:bg-red-950/70",
        }
    }
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "h-8 px-3 text-sm gap-1.5 rounded-lg",
            ButtonSize::Medium => "h-9 px-4 text-sm gap-2 rounded-lg",
            ButtonSize::Large => "h-11 px-5 text-base gap-2.5 rounded-xl",
        }
    }
}

fn base_classes() -> &'static str {
    "inline-flex items-center justify-center font-medium transition-all duration-150 cursor-pointer select-none"
}

pub fn button(label: &str, variant: ButtonVariant, size: ButtonSize) -> Markup {
    html! {
        button class=(format!("{} {} {}", base_classes(), variant.classes(), size.classes())) {
            (label)
        }
    }
}

pub fn button_with_icon(
    label: &str,
    icon: Markup,
    variant: ButtonVariant,
    size: ButtonSize,
) -> Markup {
    html! {
        button class=(format!("{} {} {}", base_classes(), variant.classes(), size.classes())) {
            span class="opacity-70" { (icon) }
            (label)
        }
    }
}

pub fn button_link(href: &str, label: &str, variant: ButtonVariant, size: ButtonSize) -> Markup {
    html! {
        a href=(href) class=(format!("{} {} {} no-underline", base_classes(), variant.classes(), size.classes())) {
            (label)
        }
    }
}

pub fn button_link_with_icon(
    href: &str,
    label: &str,
    icon: Markup,
    variant: ButtonVariant,
    size: ButtonSize,
) -> Markup {
    html! {
        a href=(href) class=(format!("{} {} {} no-underline", base_classes(), variant.classes(), size.classes())) {
            span class="opacity-70" { (icon) }
            (label)
        }
    }
}

pub fn github_button(href: &str) -> Markup {
    html! {
        a
            href=(href)
            class="group inline-flex items-center justify-center gap-2.5 h-12 px-6 \
                   text-sm font-medium rounded-xl \
                   bg-white text-[#0d0d0d] \
                   shadow-lg shadow-black/20 \
                   hover:bg-gray-100 hover:shadow-xl hover:shadow-black/25 \
                   active:bg-gray-200 active:shadow-md \
                   transition-all duration-150 no-underline cursor-pointer"
        {
            span class="group-hover:scale-110 transition-transform duration-150" {
                (super::icon::github(20))
            }
            "Continue with GitHub"
        }
    }
}

pub fn nav_link(href: &str, label: &str, is_active: bool) -> Markup {
    let state = if is_active {
        "text-[var(--text-primary)] bg-[var(--bg-surface)]"
    } else {
        "text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)]"
    };

    html! {
        a href=(href) class=(format!("px-3 py-2 text-sm font-medium rounded-lg transition-all duration-150 no-underline cursor-pointer {}", state)) {
            (label)
        }
    }
}

pub fn nav_link_with_icon(href: &str, label: &str, icon: Markup, is_active: bool) -> Markup {
    let state = if is_active {
        "text-[var(--text-primary)] bg-[var(--bg-surface)]"
    } else {
        "text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-surface)]"
    };

    html! {
        a href=(href) class=(format!("inline-flex items-center gap-2 px-3 py-2 text-sm font-medium rounded-lg transition-all duration-150 no-underline cursor-pointer {}", state)) {
            (icon)
            (label)
        }
    }
}

pub fn button_submit(label: &str, variant: ButtonVariant, size: ButtonSize) -> Markup {
    html! {
        button
            type="submit"
            class=(format!("{} {} {}", base_classes(), variant.classes(), size.classes()))
        {
            (label)
        }
    }
}
