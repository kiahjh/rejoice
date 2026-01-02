use crate::markdown::code_block_with_filename;
use rejoice::{html, island, json, Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;

    let nested_routes_tree = json!([
        {
            "name": "src/routes/",
            "type": "folder",
            "children": [
                { "name": "index.rs", "type": "file", "comment": "/" },
                { "name": "about.rs", "type": "file", "comment": "/about" },
                {
                    "name": "blog/",
                    "type": "folder",
                    "children": [
                        { "name": "index.rs", "type": "file", "comment": "/blog" },
                        { "name": "[slug].rs", "type": "file", "comment": "/blog/:slug" },
                        {
                            "name": "archive/",
                            "type": "folder",
                            "children": [
                                { "name": "index.rs", "type": "file", "comment": "/blog/archive" }
                            ]
                        }
                    ]
                }
            ]
        }
    ]);

    res.html(html! {
        h1 { "Routing" }

        p {
            "Rejoice uses file-based routing. Every " code { ".rs" } " file in " code { "src/routes/" } " becomes a route automatically."
        }

        h2 { "File-to-URL Mapping" }

        table {
            thead {
                tr {
                    th { "File Path" }
                    th { "URL" }
                }
            }
            tbody {
                tr { td { code { "src/routes/index.rs" } } td { code { "/" } } }
                tr { td { code { "src/routes/about.rs" } } td { code { "/about" } } }
                tr { td { code { "src/routes/contact.rs" } } td { code { "/contact" } } }
                tr { td { code { "src/routes/blog/index.rs" } } td { code { "/blog" } } }
                tr { td { code { "src/routes/blog/post.rs" } } td { code { "/blog/post" } } }
                tr { td { code { "src/routes/users/[id].rs" } } td { code { "/users/:id" } } }
            }
        }

        h3 { "Naming Convention" }

        p { "File names with underscores are converted to hyphens in URLs:" }

        table {
            thead {
                tr {
                    th { "File" }
                    th { "URL" }
                }
            }
            tbody {
                tr { td { code { "src/routes/about_us.rs" } } td { code { "/about-us" } } }
                tr { td { code { "src/routes/contact_form.rs" } } td { code { "/contact-form" } } }
            }
        }

        h2 { "Basic Route" }

        p { "Route files export functions named after HTTP methods like " code { "get" } " or " code { "post" } ":" }

        (code_block_with_filename(r#"use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}"#, "rust", Some("src/routes/index.rs")))

        h2 { "Index Routes" }

        p { "Files named " code { "index.rs" } " handle the directory's root path:" }

        ul {
            li { code { "src/routes/index.rs" } " → " code { "/" } }
            li { code { "src/routes/blog/index.rs" } " → " code { "/blog" } }
            li { code { "src/routes/users/index.rs" } " → " code { "/users" } }
        }

        h2 { "Dynamic Routes" }

        p { "Use square brackets for dynamic path segments:" }

        p { strong { code { "src/routes/users/[id].rs" } } " handles " code { "/users/:id" } ":" }

        (code_block_with_filename(r#"use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res, id: String) -> Res {
    res.html(html! {
        h1 { "User " (id) }
    })
}"#, "rust", Some("src/routes/users/[id].rs")))

        p { "The parameter is passed as the last argument to your handler function." }

        h3 { "Examples" }

        table {
            thead {
                tr {
                    th { "File" }
                    th { "URL" }
                    th { "Parameter" }
                }
            }
            tbody {
                tr { td { code { "[id].rs" } } td { code { "/users/123" } } td { code { "id = \"123\"" } } }
                tr { td { code { "[slug].rs" } } td { code { "/blog/hello-world" } } td { code { "slug = \"hello-world\"" } } }
                tr { td { code { "[category].rs" } } td { code { "/products/electronics" } } td { code { "category = \"electronics\"" } } }
            }
        }

        h2 { "Route Function Signatures" }

        h3 { "Stateless Routes" }

        p { "For apps without shared state (using " code { "routes!()" } "):" }

        (code_block_with_filename(r#"// Basic route
pub async fn get(req: Req, res: Res) -> Res

// Dynamic route
pub async fn get(req: Req, res: Res, id: String) -> Res"#, "rust", None))

        h3 { "Stateful Routes" }

        p { "For apps with shared state (using " code { "routes!(AppState)" } "):" }

        (code_block_with_filename(r#"// Basic route
pub async fn get(state: AppState, req: Req, res: Res) -> Res

// Dynamic route  
pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res"#, "rust", None))

        h2 { "Nested Directories" }

        p { "Create nested routes by adding subdirectories:" }

        (island!(FileTree, { items: nested_routes_tree }))

        h2 { "Next Steps" }

        ul {
            li { a href="/docs/layouts" { "Layouts" } " — Wrap routes with shared UI" }
            li { a href="/docs/request" { "Request Object" } " — Access request data" }
            li { a href="/docs/response" { "Response Object" } " — Build responses" }
        }
    })
}
