use axum::{extract::Path, response::Html};

pub async fn handler(Path(id): Path<String>) -> Html<String> {
    Html(format!("<h1>Some user {}</h1>", id))
}
