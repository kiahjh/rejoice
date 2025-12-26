use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Users List</h1>")
}
