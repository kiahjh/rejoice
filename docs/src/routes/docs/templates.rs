use crate::markdown::render_markdown;
use rejoice::{html, Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;
    let content = include_str!("../../../content/templates.md");
    res.html(html! { (render_markdown(content)) })
}
