use crate::markdown::code_block_with_filename;
use rejoice::{html, island, json, Req, Res};

pub async fn page(req: Req, res: Res) -> Res {
    let _ = req;

    let nested_layout_tree = json!([
        {
            "name": "src/routes/",
            "type": "folder",
            "children": [
                { "name": "layout.rs", "type": "file", "comment": "Root layout (wraps everything)" },
                { "name": "index.rs", "type": "file", "comment": "/" },
                { "name": "about.rs", "type": "file", "comment": "/about" },
                {
                    "name": "dashboard/",
                    "type": "folder",
                    "children": [
                        { "name": "layout.rs", "type": "file", "comment": "Dashboard layout (wraps /dashboard/*)" },
                        { "name": "index.rs", "type": "file", "comment": "/dashboard" },
                        { "name": "settings.rs", "type": "file", "comment": "/dashboard/settings" }
                    ]
                }
            ]
        }
    ]);

    res.html(html! {
        h1 { "Layouts" }

        p {
            "Layouts wrap your pages with shared UI like navigation, headers, and footers. They nest automatically based on directory structure."
        }

        h2 { "Basic Layout" }

        p { "Create a " code { "layout.rs" } " file in any routes directory:" }

        (code_block_with_filename(r#"use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    res.html(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "My App" }
            }
            body {
                nav {
                    a href="/" { "Home" }
                    a href="/about" { "About" }
                }
                main { (children) }
                footer { "Built with Rejoice" }
            }
        }
    })
}"#, "rust", Some("src/routes/layout.rs")))

        p { "The " code { "children" } " parameter contains the rendered page content." }

        h2 { "Root Layout" }

        p {
            "A layout at " code { "src/routes/layout.rs" } " wraps " strong { "all" } " pages in your app. This is where you typically put:"
        }

        ul {
            li { "The HTML document structure (" code { "<!DOCTYPE html>" } ", " code { "<html>" } ", " code { "<head>" } ", " code { "<body>" } ")" }
            li { "Global navigation" }
            li { "Meta tags" }
            li { "Global styles" }
        }

        h2 { "Nested Layouts" }

        p { "Layouts nest based on directory structure:" }

        (island!(FileTree, { items: nested_layout_tree }))

        p { "For " code { "/dashboard/settings" } ":" }

        ol {
            li { code { "settings.rs" } " renders the page content" }
            li { code { "dashboard/layout.rs" } " wraps it with dashboard UI" }
            li { code { "layout.rs" } " wraps everything with the document structure" }
        }

        h2 { "Layout with State" }

        p { "If your app uses state, layouts receive it as the first parameter:" }

        (code_block_with_filename(r#"use crate::AppState;
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    // Access state.db, state.config, etc.
    res.html(html! {
        (DOCTYPE)
        html {
            head { title { "My App" } }
            body { (children) }
        }
    })
}"#, "rust", Some("src/routes/layout.rs")))

        h2 { "Layout Bypass" }

        p { "Non-HTML responses automatically bypass layouts. This is useful for:" }

        ul {
            li { "Redirects" }
            li { "API endpoints returning JSON" }
            li { "File downloads" }
        }

        (code_block_with_filename(r#"pub async fn page(req: Req, res: Res) -> Res {
    let session = req.cookies.get("session_id");
    
    if session.is_none() {
        // This redirect bypasses all layouts
        return res.redirect("/login");
    }
    
    // This HTML gets wrapped in layouts
    res.html(html! {
        h1 { "Dashboard" }
    })
}"#, "rust", None))

        h2 { "Authentication Pattern" }

        p { "Use layouts to protect groups of routes:" }

        (code_block_with_filename(r#"// src/routes/admin/layout.rs
use crate::AppState;
use rejoice::{Children, Req, Res, html};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    let session = req.cookies.get("session_id");
    
    // Check authentication
    if !is_admin(&state, session).await {
        return res.redirect("/login");
    }
    
    // Render admin layout
    res.html(html! {
        div class="admin-layout" {
            aside { /* Admin sidebar */ }
            main { (children) }
        }
    })
}"#, "rust", Some("src/routes/admin/layout.rs")))

        p { "All pages under " code { "src/routes/admin/" } " will be protected." }

        h2 { "Next Steps" }

        ul {
            li { a href="/docs/templates" { "HTML Templates" } " — Learn Maud syntax" }
            li { a href="/docs/request" { "Request Object" } " — Read cookies and headers" }
            li { a href="/docs/response" { "Response Object" } " — Redirects and cookies" }
        }
    })
}
