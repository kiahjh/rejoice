use rejoice::{Markup, html};

pub async fn handler() -> Markup {
    html! {
        h1 { "About" }
        p { "This is an example app built with Rejoice." }
        a href="/" { "Back home" }
    }
}
