use axum::{Extension, Router};
use sqlx::{Pool, Sqlite};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub struct App {
    port: u16,
    pool: Pool<Sqlite>,
}

impl App {
    pub fn new(port: u16, pool: Pool<Sqlite>) -> Self {
        Self { port, pool }
    }

    pub async fn run(self) {
        let app = Router::new().layer(
            ServiceBuilder::new().layer(Extension(self.pool)).layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .allow_origin(Any),
            ),
        );

        let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", self.port))
            .await
            .unwrap();
        println!("Listening on port {} ✨", self.port);
        axum::serve(listener, app).await.unwrap();
    }
}
