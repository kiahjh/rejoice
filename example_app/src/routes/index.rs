use rejoice::{DOCTYPE, Markup, html, island};

pub async fn handler() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Home" }
            }
            body class="min-h-screen flex flex-col justify-center items-center" {
                h1 class="text-2xl font-bold" { "Welcome to Rejoice!" }
                p class="my-2" { "A simple and delightful web framework for Rust." }

                h2 { "Interactive Counter (SolidJS Island)" }
                (island!(Counter, { initial: 5 }))

                nav {
                    a href="/about" { "About" }
                    " | "
                    a href="/users" { "Users" }
                }
            }
        }
    }
}
