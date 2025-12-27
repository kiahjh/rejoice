use rejoice::{Children, Markup, html};

pub async fn layout(children: Children) -> Markup {
    html! {
        div class="flex flex-row" {
            div class="p-8 bg-gray-200 mr-8" {
                p { "Users" }
            }
            main class="flex-grow" {
                (children)
            }
        }
    }
}
