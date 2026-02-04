use rejoice::{html, Markup};

pub fn back_link(href: &str, label: &str) -> Markup {
    html! {
        a href=(href) class="text-sm text-stone-500 hover:text-stone-300 no-underline" {
            "← " (label)
        }
    }
}

pub fn page_header(title: &str, actions: Option<Markup>) -> Markup {
    html! {
        div class="flex items-center justify-between mb-8" {
            h1 class="text-xl font-medium text-stone-100" { (title) }
            @if let Some(content) = actions {
                (content)
            }
        }
    }
}

pub fn empty_state(message: &str) -> Markup {
    html! {
        div class="text-center py-16" {
            p class="text-stone-500" { (message) }
        }
    }
}
