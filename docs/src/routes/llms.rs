use rejoice::{Req, Res};

pub async fn page(req: Req, res: Res) -> Res {
    let _ = req;

    // Serve the LLM_DOCS.md file as plain text
    let content = include_str!("../../../LLM_DOCS.md");

    // Set header first (returns &Res), then call raw (consumes Res)
    res.set_header("Content-Type", "text/plain; charset=utf-8");
    res.raw(content.as_bytes().to_vec())
}
