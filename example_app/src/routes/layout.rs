use rejoice::{Children, DOCTYPE, Markup, html};

pub async fn layout(children: Children) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Rejoice App" }
            }
            body class="min-h-screen bg-gray-50" {
                header class="bg-blue-600 text-white p-4" {
                    nav class="max-w-4xl mx-auto flex gap-4" {
                        a href="/" class="font-bold" { "Home" }
                        a href="/about" { "About" }
                        a href="/users" { "Users" }
                    }
                }
                main class="max-w-4xl mx-auto p-4" {
                    (children)
                }
                footer class="bg-gray-200 p-4 mt-8" {
                    p class="max-w-4xl mx-auto text-center text-gray-600" {
                        "Built with Rejoice"
                    }
                }
            }
        }
    }
}
