use axum::{
    Router,
    body::Body,
    http::{Request, Response, header},
};
use std::path::Path;
use std::task::{Context, Poll};
use tower::{Layer, Service, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

pub struct App {
    port: u16,
    router: Router,
}

impl App {
    pub fn new(port: u16, router: Router) -> Self {
        let dev_mode = std::env::var("REJOICE_DEV").is_ok();
        let has_islands = Path::new("dist/islands.js").exists();

        // Serve static files from dist/ directory (built JS)
        let static_dir = Path::new("dist");
        let mut router = if static_dir.exists() {
            router.nest_service("/static", ServeDir::new(static_dir))
        } else {
            router
        };

        router = router.layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .allow_origin(Any),
            ),
        );

        // Add script injection middleware
        router = router.layer(ScriptInjectionLayer { dev_mode, has_islands });

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

const ISLAND_SCRIPT: &str = r#"<script type="module" src="/static/islands.js"></script>"#;

#[derive(Clone)]
pub struct ScriptInjectionLayer {
    dev_mode: bool,
    has_islands: bool,
}

impl<S> Layer<S> for ScriptInjectionLayer {
    type Service = ScriptInjectionMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ScriptInjectionMiddleware {
            inner,
            dev_mode: self.dev_mode,
            has_islands: self.has_islands,
        }
    }
}

#[derive(Clone)]
pub struct ScriptInjectionMiddleware<S> {
    inner: S,
    dev_mode: bool,
    has_islands: bool,
}

impl<S> Service<Request<Body>> for ScriptInjectionMiddleware<S>
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
        let dev_mode = self.dev_mode;
        let has_islands = self.has_islands;
        
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

            // Build the scripts to inject
            let mut scripts = String::new();
            if has_islands {
                scripts.push_str(ISLAND_SCRIPT);
            }
            if dev_mode {
                scripts.push_str(LIVE_RELOAD_SCRIPT);
            }

            if scripts.is_empty() {
                return Ok(response);
            }

            // Read the body and inject the scripts
            let (parts, body) = response.into_parts();
            let bytes = axum::body::to_bytes(body, usize::MAX)
                .await
                .unwrap_or_default();
            let html = String::from_utf8_lossy(&bytes);

            // Inject before </body> if present, otherwise append
            let modified = if html.contains("</body>") {
                html.replace("</body>", &format!("{}</body>", scripts))
            } else {
                format!("{}{}", html, scripts)
            };

            let new_body = Body::from(modified);
            Ok(Response::from_parts(parts, new_body))
        })
    }
}
