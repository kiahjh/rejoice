use rejoice::{html, Markup};

pub fn heading(text: &str) -> Markup {
    html! {
        h2 class="text-sm font-medium text-stone-300" { (text) }
    }
}

pub fn text(content: &str) -> Markup {
    html! {
        p class="text-sm text-stone-400" { (content) }
    }
}

pub fn text_muted(content: &str) -> Markup {
    html! {
        p class="text-sm text-stone-500" { (content) }
    }
}
