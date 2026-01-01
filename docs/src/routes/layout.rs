use rejoice::{html, island, Children, Req, Res, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    let _ = req;

    res.html(html! {
        (DOCTYPE)
        html lang="en" class="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Rejoice - A delightful Rust web framework" }
                meta name="description" content="A simple and delightful little web framework for Rust with file-based routing, SolidJS islands, and Tailwind CSS.";
                link rel="icon" href="/favicon.ico";
                // Fonts - Inter for UI, JetBrains Mono for code
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous";
                link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Instrument+Serif:ital@0;1&family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap";
            }
            body class="min-h-screen antialiased font-sans" style="font-family: 'Inter', system-ui, sans-serif;" {
                (children)
                // Code highlighter island - runs once to highlight all code blocks
                (island!(CodeHighlighter))
            }
        }
    })
}
