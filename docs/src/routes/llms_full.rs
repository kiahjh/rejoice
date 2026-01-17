use rejoice::{Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;

    // Serve the full llms documentation as plain text
    let content = include_str!("../../../llms-full.txt");

    res.set_header("Content-Type", "text/plain; charset=utf-8")
        .raw(content.as_bytes().to_vec())
}
