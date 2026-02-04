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
}

impl ButtonVariant {
    fn classes(&self) -> &'static str {
        match self {
            // Warm copper/bronze primary
            ButtonVariant::Primary => "bg-amber-600 text-white hover:bg-amber-500",
            ButtonVariant::Secondary => {
                "bg-stone-800 text-stone-200 ring-1 ring-inset ring-stone-700 hover:bg-stone-700"
            }
            ButtonVariant::Ghost => "text-stone-400 hover:text-stone-200 hover:bg-stone-800",
            ButtonVariant::Danger => {
                "bg-red-900/50 text-red-400 ring-1 ring-inset ring-red-800 hover:bg-red-900/70"
            }
        }
    }
}

impl ButtonSize {
    fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Small => "h-8 px-3 text-sm gap-1.5",
            ButtonSize::Medium => "h-9 px-4 text-sm gap-2",
        }
    }
}

fn base_classes() -> &'static str {
    "inline-flex items-center justify-center font-medium rounded-lg transition-colors cursor-pointer"
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
            class="inline-flex items-center justify-center gap-2 h-10 px-5 \
                   text-sm font-medium rounded-lg \
                   bg-white text-stone-900 hover:bg-stone-100 \
                   transition-colors no-underline cursor-pointer"
        {
            (super::icon::github(18))
            "Continue with GitHub"
        }
    }
}

pub fn nav_link(href: &str, label: &str, is_active: bool) -> Markup {
    let state = if is_active {
        "text-stone-100"
    } else {
        "text-stone-400 hover:text-stone-200"
    };

    html! {
        a href=(href) class=(format!("text-sm font-medium transition-colors no-underline cursor-pointer {}", state)) {
            (label)
        }
    }
}

pub fn nav_link_with_icon(href: &str, label: &str, icon: Markup, is_active: bool) -> Markup {
    let state = if is_active {
        "text-stone-100"
    } else {
        "text-stone-400 hover:text-stone-200"
    };

    html! {
        a href=(href) class=(format!("inline-flex items-center gap-2 text-sm font-medium transition-colors no-underline cursor-pointer {}", state)) {
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
