use axum::{Extension, Router};
use sqlx::{Pool, Sqlite};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub struct App {
    port: u16,
    router: Router,
}

impl App {
    pub fn new(port: u16, pool: Pool<Sqlite>) -> Self {
        let router = Router::new().layer(
            ServiceBuilder::new().layer(Extension(pool)).layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .allow_origin(Any),
            ),
        );

        Self { port, router }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", self.port))
            .await
            .unwrap();
        println!("Listening on port {} ✨", self.port);
        axum::serve(listener, self.router).await.unwrap();
    }
}
