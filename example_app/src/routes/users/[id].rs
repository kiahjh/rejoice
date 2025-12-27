use axum::extract::Path;
use rejoice::{Markup, html};

pub async fn page(Path(id): Path<String>) -> Markup {
    html! {
        h1 { "User " (id) }
        p { "This is the profile page for user " (id) "." }
        a href="/users" { "Back to users" }
    }
}
