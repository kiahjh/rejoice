use axum::{
    Json, Router,
    body::Body,
    http::{Request, Response, header},
    routing::get,
};
use colored::Colorize;
use serde::Serialize;
use std::path::Path;
use std::task::{Context, Poll};
use tower::{Layer, Service, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

pub struct App {
    port: u16,
    router: Router<()>,
}

impl App {
    pub fn new(port: u16, router: Router<crate::NoState>) -> Self {
        Self::with_state(port, router, crate::NoState)
    }

    pub fn with_state<S: Clone + Send + Sync + 'static>(
        port: u16,
        router: Router<S>,
        state: S,
    ) -> Self {
        let dev_mode = std::env::var("REJOICE_DEV").is_ok();
        let studio_mode = std::env::var("REJOICE_STUDIO").is_ok();
        let has_islands = Path::new("dist/islands.js").exists();
        let has_styles = Path::new("dist/styles.css").exists();

        // Serve static files from dist/ directory (built JS/CSS)
        let static_dir = Path::new("dist");
        let mut router = if static_dir.exists() {
            router.nest_service("/static", ServeDir::new(static_dir))
        } else {
            router
        };

        // Serve public/ directory at root (for images, fonts, etc.)
        let public_dir = Path::new("public");
        if public_dir.exists() {
            router = router.fallback_service(ServeDir::new(public_dir));
        }

        // Add Studio endpoints in dev mode
        if dev_mode {
            router = router.route("/__studio/registry", get(studio_registry_handler));
            router = router.route("/__studio/enums", get(studio_enums_handler));
        }

        // Add Studio host page when in studio mode
        if studio_mode {
            router = router.route("/__studio", get(studio_host_handler));
            router = router.route("/__studio/preview/{component}", get(studio_preview_handler));
        }

        router = router.layer(
            ServiceBuilder::new().layer(
                CorsLayer::new()
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .allow_origin(Any),
            ),
        );

        // Add script/style injection middleware
        router = router.layer(ScriptInjectionLayer {
            dev_mode,
            studio_mode,
            has_islands,
            has_styles,
        });

        // Attach state to router, converting Router<S> to Router<()>
        let router = router.with_state(state);

        Self { port, router }
    }

    pub async fn run(self) {
        let listener = tokio::net::TcpListener::bind(&format!("127.0.0.1:{}", self.port))
            .await
            .unwrap();

        let dev_mode = std::env::var("REJOICE_DEV").is_ok();
        if dev_mode {
            println!(
                "{} {} {}",
                "✓".green().bold(),
                "Server running at".white(),
                format!("http://localhost:{}", self.port).cyan().underline()
            );
        } else {
            println!("Listening on http://localhost:{}", self.port);
        }

        axum::serve(listener, self.router).await.unwrap();
    }
}

// Studio registry endpoint handler

/// JSON-serializable version of PropMeta
#[derive(Serialize)]
struct PropMetaJson {
    name: &'static str,
    ty: &'static str,
    required: bool,
    default: Option<&'static str>,
    doc: Option<&'static str>,
}

/// JSON-serializable version of ComponentMeta
#[derive(Serialize)]
struct ComponentMetaJson {
    name: &'static str,
    file: &'static str,
    line: u32,
    column: u32,
    doc: Option<&'static str>,
    props: Vec<PropMetaJson>,
}

/// Response for the registry endpoint
#[derive(Serialize)]
struct RegistryResponse {
    components: Vec<ComponentMetaJson>,
}

async fn studio_host_handler() -> axum::response::Html<String> {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Rejoice Studio</title>
</head>
<body>
    <script>{}</script>
</body>
</html>"#,
        STUDIO_HOST_SCRIPT
    );
    axum::response::Html(html)
}

async fn studio_preview_handler(
    axum::extract::Path(component): axum::extract::Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> axum::response::Html<String> {
    use maud::Render;

    // Extract background color (default to white)
    let bg_color = params
        .get("__bg")
        .cloned()
        .unwrap_or_else(|| "#ffffff".to_string());

    // Extract props from query params (they come as prop_name=json_value)
    let props: std::collections::HashMap<String, String> = params
        .iter()
        .filter_map(|(k, v)| {
            k.strip_prefix("prop_")
                .map(|name| (name.to_string(), v.clone()))
        })
        .collect();

    // Try to render using registered preview function
    let content = if let Some(markup) = crate::studio::render_preview(&component, &props) {
        // Successfully rendered component
        markup.render().into_string()
    } else if let Some(meta) = crate::studio::get_component(&component) {
        // Component exists but no preview function registered yet
        // This happens if the component hasn't been rendered yet in the app
        format!(
            r#"<div class="preview-placeholder">
                <div class="component-badge">{}</div>
                <p>Preview will be available once this component is rendered in your app.</p>
                <p class="hint">Visit a page that uses this component, then try again.</p>
            </div>"#,
            meta.name
        )
    } else {
        // Component not found at all
        format!(
            r#"<div class="error">Component "{}" not found in registry</div>"#,
            component
        )
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{} Preview - Rejoice Studio</title>
    <!-- Load Tailwind for utility classes -->
    <script src="https://cdn.tailwindcss.com"></script>
    <!-- Load common fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous">
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap">
    <!-- Load app styles for component styling (CSS vars, component classes, etc) -->
    <link rel="stylesheet" href="/static/styles.css">
    <style>
        /* Override body background - don't inherit app's background */
        body {{
            font-family: 'Inter', system-ui, sans-serif;
            background: {} !important;
            background-image: none !important;
            margin: 0;
            padding: 40px;
            min-height: 100vh;
            /* Center the component */
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        /* Also override html in case styles are applied there */
        html {{
            background: {} !important;
            background-image: none !important;
        }}
        .preview-placeholder {{
            text-align: center;
            padding: 60px 40px;
            background: #f8fafc;
            border: 1px solid #e2e8f0;
            border-radius: 12px;
            max-width: 500px;
            margin: 40px auto;
        }}
        .preview-placeholder .component-badge {{
            display: inline-block;
            padding: 8px 16px;
            background: linear-gradient(135deg, #6ee7b7, #34d399);
            border-radius: 8px;
            font-weight: 600;
            color: #064e3b;
            margin-bottom: 16px;
        }}
        .preview-placeholder p {{
            color: #64748b;
            margin: 8px 0;
        }}
        .preview-placeholder .hint {{
            font-size: 12px;
            color: #94a3b8;
        }}
        .error {{
            text-align: center;
            padding: 60px;
            color: #ef4444;
            font-size: 16px;
        }}
    </style>
</head>
<body>
    {}
</body>
</html>"#,
        component, bg_color, bg_color, content
    );

    axum::response::Html(html)
}

async fn studio_registry_handler() -> Json<RegistryResponse> {
    let components = crate::studio::get_all_components();
    let json_components: Vec<ComponentMetaJson> = components
        .into_iter()
        .map(|c| ComponentMetaJson {
            name: c.name,
            file: c.file,
            line: c.line,
            column: c.column,
            doc: c.doc,
            props: c
                .props
                .iter()
                .map(|p| PropMetaJson {
                    name: p.name,
                    ty: p.ty,
                    required: p.required,
                    default: p.default,
                    doc: p.doc,
                })
                .collect(),
        })
        .collect();

    Json(RegistryResponse {
        components: json_components,
    })
}

async fn studio_enums_handler() -> Json<std::collections::HashMap<String, Vec<String>>> {
    Json(crate::studio::get_all_prop_enums())
}

const LIVE_RELOAD_SCRIPT: &str = concat!(
    "<script>",
    include_str!("assets/live_reload.js"),
    "</script>"
);

const STUDIO_BRIDGE_SCRIPT: &str = concat!(
    "<script>",
    include_str!("assets/studio/studio-bridge.js"),
    "</script>"
);

const STUDIO_HOST_SCRIPT: &str = include_str!("assets/studio/studio-host.js");

const ISLAND_SCRIPT: &str = r#"<script type="module" src="/static/islands.js"></script>"#;
const STYLES_LINK: &str = r#"<link rel="stylesheet" href="/static/styles.css">"#;

#[derive(Clone)]
pub struct ScriptInjectionLayer {
    dev_mode: bool,
    studio_mode: bool,
    has_islands: bool,
    has_styles: bool,
}

impl<S> Layer<S> for ScriptInjectionLayer {
    type Service = ScriptInjectionMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ScriptInjectionMiddleware {
            inner,
            dev_mode: self.dev_mode,
            studio_mode: self.studio_mode,
            has_islands: self.has_islands,
            has_styles: self.has_styles,
        }
    }
}

#[derive(Clone)]
pub struct ScriptInjectionMiddleware<S> {
    inner: S,
    dev_mode: bool,
    studio_mode: bool,
    has_islands: bool,
    has_styles: bool,
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
        let studio_mode = self.studio_mode;
        let has_islands = self.has_islands;
        let has_styles = self.has_styles;

        // Check if this request has __studio_bridge query param
        let inject_bridge = req
            .uri()
            .query()
            .map(|q| q.contains("__studio_bridge"))
            .unwrap_or(false);

        // Check if this is the Studio host page (don't inject live reload there)
        let is_studio_host = req.uri().path() == "/__studio";

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

            // Build the scripts to inject before </body>
            let mut scripts = String::new();
            if has_islands && !is_studio_host {
                scripts.push_str(ISLAND_SCRIPT);
            }
            if dev_mode && !is_studio_host {
                // Don't inject live reload into Studio host - it manages its own iframe
                scripts.push_str(LIVE_RELOAD_SCRIPT);
            }
            // Inject bridge script when inside studio iframe
            if studio_mode && inject_bridge {
                scripts.push_str(STUDIO_BRIDGE_SCRIPT);
            }

            // Build the styles to inject in <head>
            let mut head_inject = String::new();
            if has_styles {
                head_inject.push_str(STYLES_LINK);
            }

            if scripts.is_empty() && head_inject.is_empty() {
                return Ok(response);
            }

            // Read the body and inject
            let (parts, body) = response.into_parts();
            let bytes = axum::body::to_bytes(body, usize::MAX)
                .await
                .unwrap_or_default();
            let html = String::from_utf8_lossy(&bytes);

            // Inject styles in <head>, or prepend if no <head>
            let mut modified = if !head_inject.is_empty() {
                if html.contains("</head>") {
                    html.replace("</head>", &format!("{}</head>", head_inject))
                } else {
                    format!("{}{}", head_inject, html)
                }
            } else {
                html.to_string()
            };

            // Inject scripts before </body>, or append if no </body>
            if !scripts.is_empty() {
                modified = if modified.contains("</body>") {
                    modified.replace("</body>", &format!("{}</body>", scripts))
                } else {
                    format!("{}{}", modified, scripts)
                };
            }

            let new_body = Body::from(modified);
            Ok(Response::from_parts(parts, new_body))
        })
    }
}
