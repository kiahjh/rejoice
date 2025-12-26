use axum::{
    Router,
    body::Body,
    http::{Request, Response, header},
};
use std::task::{Context, Poll};
use tower::{Layer, Service, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};

pub struct App {
    port: u16,
    router: Router,
}

impl App {
    pub fn new(port: u16, router: Router) -> Self {
        let dev_mode = std::env::var("REJOICE_DEV").is_ok();

        let mut router = router.layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .allow_origin(Any),
            ),
        );

        // Add live reload middleware in dev mode
        if dev_mode {
            router = router.layer(LiveReloadLayer);
        }

        Self { port, router }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", self.port))
            .await
            .unwrap();
        println!("Listening on port {}", self.port);
        axum::serve(listener, self.router).await.unwrap();
    }
}

const LIVE_RELOAD_SCRIPT: &str = concat!(
    "<script>",
    include_str!("assets/live_reload.js"),
    "</script>"
);

#[derive(Clone)]
pub struct LiveReloadLayer;

impl<S> Layer<S> for LiveReloadLayer {
    type Service = LiveReloadMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LiveReloadMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LiveReloadMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for LiveReloadMiddleware<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let response = inner.call(req).await?;

            // Check if this is an HTML response
            let is_html = response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|v| v.contains("text/html"))
                .unwrap_or(false);

            if !is_html {
                return Ok(response);
            }

            // Read the body and inject the script
            let (parts, body) = response.into_parts();
            let bytes = axum::body::to_bytes(body, usize::MAX)
                .await
                .unwrap_or_default();
            let html = String::from_utf8_lossy(&bytes);

            // Inject before </body> if present, otherwise append
            let modified = if html.contains("</body>") {
                html.replace("</body>", &format!("{}</body>", LIVE_RELOAD_SCRIPT))
            } else {
                format!("{}{}", html, LIVE_RELOAD_SCRIPT)
            };

            let new_body = Body::from(modified);
            Ok(Response::from_parts(parts, new_body))
        })
    }
}
