use crate::markdown::code_block_with_filename;
use rejoice::{html, island, json, Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;

    let public_tree = json!([
        {
            "name": "public/",
            "type": "folder",
            "children": [
                { "name": "favicon.ico", "type": "file" },
                { "name": "robots.txt", "type": "file" },
                { "name": "sitemap.xml", "type": "file" },
                {
                    "name": "images/",
                    "type": "folder",
                    "children": [
                        { "name": "logo.png", "type": "file" },
                        { "name": "og-image.jpg", "type": "file" }
                    ]
                },
                {
                    "name": "fonts/",
                    "type": "folder",
                    "children": [
                        { "name": "inter.woff2", "type": "file" },
                        { "name": "fonts.css", "type": "file" }
                    ]
                }
            ]
        }
    ]);

    let production_tree = json!([
        {
            "name": "my-app/",
            "type": "folder",
            "children": [
                { "name": "my-app", "type": "file", "comment": "Binary" },
                {
                    "name": "dist/",
                    "type": "folder",
                    "comment": "Built client assets",
                    "children": [
                        { "name": "islands.js", "type": "file" },
                        { "name": "styles.css", "type": "file" }
                    ]
                },
                {
                    "name": "public/",
                    "type": "folder",
                    "comment": "Static files",
                    "children": []
                }
            ]
        }
    ]);

    res.html(html! {
        h1 { "Static Assets" }

        p { "The " code { "public/" } " directory serves static files at the root URL path." }

        h2 { "File Mapping" }

        table {
            thead {
                tr {
                    th { "File Path" }
                    th { "URL" }
                }
            }
            tbody {
                tr {
                    td { code { "public/logo.png" } }
                    td { code { "/logo.png" } }
                }
                tr {
                    td { code { "public/images/hero.jpg" } }
                    td { code { "/images/hero.jpg" } }
                }
                tr {
                    td { code { "public/favicon.ico" } }
                    td { code { "/favicon.ico" } }
                }
                tr {
                    td { code { "public/fonts/custom.woff2" } }
                    td { code { "/fonts/custom.woff2" } }
                }
            }
        }

        h2 { "Usage in Templates" }

        p { "Reference static assets with absolute paths:" }

        (code_block_with_filename(r#"html! {
    head {
        link rel="icon" href="/favicon.ico";
        link rel="stylesheet" href="/fonts/fonts.css";
    }
    body {
        img src="/logo.png" alt="Logo";
        img src="/images/hero.jpg" alt="Hero image";
    }
}"#, "rust", None))

        h2 { "Route Priority" }

        p { "Defined routes take precedence over static files. If you have both:" }

        ul {
            li { code { "src/routes/about.rs" } }
            li { code { "public/about.html" } }
        }

        p { "The route wins, and " code { "/about" } " serves the Rust route." }

        h2 { "Built Assets" }

        p { "Client assets (JavaScript, CSS) are output to " code { "dist/" } " and served automatically:" }

        table {
            thead {
                tr {
                    th { "Asset" }
                    th { "URL" }
                }
            }
            tbody {
                tr {
                    td { code { "dist/islands.js" } }
                    td { "Auto-injected" }
                }
                tr {
                    td { code { "dist/styles.css" } }
                    td { "Auto-injected" }
                }
            }
        }

        p { "You don't need to reference these manually—they're injected into all HTML responses." }

        h2 { "Common Static Files" }

        p { "Typical " code { "public/" } " contents:" }

        (island!(FileTree, { items: public_tree }))

        h2 { "Custom Fonts" }

        p { "Add custom fonts to " code { "public/fonts/" } ":" }

        (code_block_with_filename(r#"@font-face {
  font-family: 'Inter';
  src: url('/fonts/inter.woff2') format('woff2');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}"#, "css", Some("public/fonts/fonts.css")))

        p { "Reference in your layout:" }

        (code_block_with_filename(r#"html! {
    head {
        link rel="stylesheet" href="/fonts/fonts.css";
    }
    body class="font-['Inter']" {
        // Content with custom font
    }
}"#, "rust", None))

        h2 { "Development" }

        p {
            "During " code { "rejoice dev" } ", the " code { "public/" }
            " directory is watched. Changes to static files trigger a browser reload."
        }

        h2 { "Production" }

        p { "In production, include the " code { "public/" } " directory alongside your binary:" }

        (island!(FileTree, { items: production_tree }))

        h2 { "Next Steps" }

        ul {
            li { a href="/docs/tailwind" { "Tailwind CSS" } " — Style your pages" }
            li { a href="/docs/deployment" { "Deployment" } " — Production setup" }
        }
    })
}
