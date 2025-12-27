use rejoice::{Markup, html, island};

pub async fn page() -> Markup {
    html! {
        h1 class="text-4xl font-bold" { "Welcome to Rejoice!" }
        p class="my-2 text-gray-600" { "A simple and delightful web framework for Rust." }

        h2 class="text-xl font-semibold mt-6" { "Interactive Counter (SolidJS Island)" }
        (island!(Counter, { initial: 5 }))
    }
}
