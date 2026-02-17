use crate::AppState;
use rejoice::{Req, Res};

pub async fn get(_state: AppState, _req: Req, res: Res) -> Res {
    res.delete_cookie("session").redirect("/")
}
