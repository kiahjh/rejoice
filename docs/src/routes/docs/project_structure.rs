use crate::markdown::code_block_with_filename;
use rejoice::{html, island, json, Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;

    // Define the file tree structure as JSON for the island
    let file_tree = json!([
        {
            "name": "my-app/",
            "type": "folder",
            "children": [
                { "name": "build.rs", "type": "file", "comment": "Build script for route generation" },
                { "name": "Cargo.toml", "type": "file", "comment": "Rust dependencies" },
                { "name": "package.json", "type": "file", "comment": "Node.js dependencies" },
                { "name": "vite.config.ts", "type": "file", "comment": "Vite configuration" },
                { "name": "tsconfig.json", "type": "file", "comment": "TypeScript configuration" },
                { "name": ".env", "type": "file", "comment": "Environment variables (optional)" },
                { "name": ".gitignore", "type": "file" },
                {
                    "name": "client/",
                    "type": "folder",
                    "comment": "Client-side code",
                    "children": [
                        { "name": "styles.css", "type": "file", "comment": "Tailwind CSS entry point" },
                        { "name": "Counter.tsx", "type": "file", "comment": "Example SolidJS component" },
                        { "name": "islands.tsx", "type": "file", "comment": "Auto-generated (do not edit)" }
                    ]
                },
                {
                    "name": "public/",
                    "type": "folder",
                    "comment": "Static assets served at root URL",
                    "children": [
                        { "name": "favicon.ico", "type": "file" },
                        { "name": "logo.png", "type": "file" }
                    ]
                },
                {
                    "name": "dist/",
                    "type": "folder",
                    "comment": "Built client assets (auto-generated)",
                    "children": [
                        { "name": "islands.js", "type": "file" },
                        { "name": "styles.css", "type": "file" }
                    ]
                },
                {
                    "name": "src/",
                    "type": "folder",
                    "children": [
                        { "name": "main.rs", "type": "file", "comment": "Application entry point" },
                        { "name": "routes.rs", "type": "file", "comment": "Auto-generated route modules" },
                        {
                            "name": "routes/",
                            "type": "folder",
                            "comment": "File-based routes",
                            "children": [
                                { "name": "layout.rs", "type": "file", "comment": "Root layout" },
                                { "name": "index.rs", "type": "file", "comment": "GET /" }
                            ]
                        }
                    ]
                }
            ]
        }
    ]);

    res.html(html! {
        h1 { "Project Structure" }

        p {
            "A Rejoice project follows a predictable structure that makes it easy to find and organize your code."
        }

        h2 { "Directory Layout" }

        (island!(FileTree, { items: file_tree }))

        h2 { "Key Files" }

        h3 { code { "src/main.rs" } }

        p {
            "The entry point for your application. It creates the app and starts the server:"
        }

        (code_block_with_filename(r#"use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}"#, "rust", Some("src/main.rs")))

        h3 { code { "build.rs" } }

        p {
            "The build script that generates routes at compile time:"
        }

        (code_block_with_filename(r#"fn main() {
    rejoice::codegen::generate_routes();
}"#, "rust", Some("build.rs")))

        p { "This scans your " code { "src/routes/" } " directory and generates the router code." }

        h3 { code { "src/routes/" } }

        p { "This directory contains your route files. Each file becomes a route:" }

        table {
            thead {
                tr {
                    th { "File" }
                    th { "URL" }
                }
            }
            tbody {
                tr {
                    td { code { "routes/index.rs" } }
                    td { code { "/" } }
                }
                tr {
                    td { code { "routes/about.rs" } }
                    td { code { "/about" } }
                }
                tr {
                    td { code { "routes/blog/index.rs" } }
                    td { code { "/blog" } }
                }
                tr {
                    td { code { "routes/blog/[slug].rs" } }
                    td { code { "/blog/:slug" } }
                }
                tr {
                    td { code { "routes/layout.rs" } }
                    td { "(wraps all routes)" }
                }
            }
        }

        h3 { code { "client/" } }

        p { "Client-side code lives here:" }

        ul {
            li { strong { "styles.css" } " — Your Tailwind CSS entry point" }
            li { strong { "*.tsx" } " — SolidJS island components" }
            li { strong { "islands.tsx" } " — Auto-generated registry (don't edit this)" }
        }

        h3 { code { "public/" } }

        p { "Static assets are served directly from this directory:" }

        ul {
            li { code { "public/logo.png" } " → " code { "/logo.png" } }
            li { code { "public/images/hero.jpg" } " → " code { "/images/hero.jpg" } }
        }

        h3 { code { "dist/" } }

        p {
            "Built client assets are output here. This is auto-generated during build and should be in your "
            code { ".gitignore" }
            "."
        }

        h2 { "Auto-generated Files" }

        p { "Some files are generated automatically and should not be edited:" }

        ul {
            li { strong { "src/routes.rs" } " — Route module declarations" }
            li { strong { "client/islands.tsx" } " — Island component registry" }
            li { strong { "dist/" } " — Built client assets" }
        }

        p { "These are regenerated when you run " code { "rejoice dev" } " or " code { "rejoice build" } "." }

        h2 { "Next Steps" }

        ul {
            li { a href="/docs/routing" { "Routing" } " — Learn how file-based routing works" }
            li { a href="/docs/layouts" { "Layouts" } " — Understand nested layouts" }
        }
    })
}
