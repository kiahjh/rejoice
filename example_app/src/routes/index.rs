use rejoice::{DOCTYPE, Markup, html};

pub async fn handler() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Home" }
            }
            body {
                h1 { "Welcome to Rejoice!" }
                p { "A simple and delightful web framework for Rust." }
                nav {
                    a href="/about" { "About" }
                    " | "
                    a href="/users" { "Users" }
                }
            }
        }
    }
}
