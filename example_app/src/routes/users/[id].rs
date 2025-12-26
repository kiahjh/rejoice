use axum::extract::Path;
use rejoice::{html, Markup};

pub async fn handler(Path(id): Path<String>) -> Markup {
    html! {
        h1 { "User " (id) }
        p { "This is the profile page for user " (id) "." }
        a href="/users" { "Back to users" }
    }
}
