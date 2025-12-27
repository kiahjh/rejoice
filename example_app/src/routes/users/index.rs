use rejoice::{Markup, html};

pub async fn page() -> Markup {
    html! {
        h1 { "Users" }
        ul {
            li { a href="/users/1" { "User 1" } }
            li { a href="/users/2" { "User 2" } }
            li { a href="/users/3" { "User 3" } }
        }
        a href="/" { "Back home" }
    }
}
