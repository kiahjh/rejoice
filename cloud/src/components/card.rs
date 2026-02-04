use rejoice::{html, Markup};

pub fn card(children: Markup) -> Markup {
    html! {
        div class="bg-stone-900 border border-stone-800 rounded-xl p-5" {
            (children)
        }
    }
}

pub fn card_interactive(href: &str, children: Markup) -> Markup {
    html! {
        a
            href=(href)
            class="block bg-stone-900 border border-stone-800 rounded-xl p-5 \
                   no-underline transition-colors cursor-pointer \
                   hover:bg-stone-800/80 hover:border-stone-700"
        {
            (children)
        }
    }
}

pub fn project_card(href: &str, name: &str, repo: &str, is_deployed: bool) -> Markup {
    html! {
        a
            href=(href)
            class="block bg-stone-900 border border-stone-800 rounded-xl p-5 \
                   no-underline transition-colors cursor-pointer \
                   hover:bg-stone-800/80 hover:border-stone-700"
        {
            div class="flex items-start justify-between gap-4" {
                div class="min-w-0 flex-1" {
                    h3 class="text-base font-medium text-stone-100 truncate" { (name) }
                    p class="mt-1 text-sm text-stone-500 truncate" { (repo) }
                }
                @if is_deployed {
                    (status_dot(StatusVariant::Success))
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
}

impl BadgeVariant {
    fn classes(&self) -> &'static str {
        match self {
            BadgeVariant::Default => "bg-stone-800 text-stone-400",
            BadgeVariant::Success => "bg-emerald-900/50 text-emerald-400",
            BadgeVariant::Warning => "bg-amber-900/50 text-amber-400",
            BadgeVariant::Error => "bg-red-900/50 text-red-400",
        }
    }
}

pub fn badge(label: &str, variant: BadgeVariant) -> Markup {
    html! {
        span class=(format!("inline-flex items-center px-2 py-0.5 text-xs font-medium rounded {}", variant.classes())) {
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
}

impl StatusVariant {
    fn classes(&self) -> &'static str {
        match self {
            StatusVariant::Success => "bg-emerald-500",
            StatusVariant::Warning => "bg-amber-500",
            StatusVariant::Error => "bg-red-500",
            StatusVariant::Neutral => "bg-stone-500",
        }
    }
}

pub fn status_dot(variant: StatusVariant) -> Markup {
    html! {
        span class=(format!("inline-block w-2 h-2 rounded-full {}", variant.classes())) {}
    }
}
