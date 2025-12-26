use rejoice::{DOCTYPE, Markup, html, island};

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

                h2 { "Interactive Counter (SolidJS Island)" }
                (island!(Counter, { "initial": 5 }))

                nav {
                    a href="/about" { "About" }
                    " | "
                    a href="/users" { "Users" }
                }
            }
        }
    }
}
