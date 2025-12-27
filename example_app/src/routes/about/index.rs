use rejoice::{Markup, html};

pub async fn page() -> Markup {
    html! {
        h1 { "About" }
        p { "This is an example app built with Rejoice." }
        a href="/" { "Back home" }
    }
}
