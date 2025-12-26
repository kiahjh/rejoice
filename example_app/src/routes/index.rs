use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Home</h1>")
}
