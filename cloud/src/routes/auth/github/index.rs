use rejoice::{Req, Res};

use crate::AppState;

const GITHUB_CLIENT_ID: &str = rejoice::env!("GITHUB_CLIENT_ID");

pub async fn get(_state: AppState, _req: Req, res: Res) -> Res {
    let redirect_uri = "http://localhost:3333/auth/github/callback";
    let scope = "read:user user:email";

    let github_auth_url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope={}",
        GITHUB_CLIENT_ID,
        urlencoding::encode(redirect_uri),
        urlencoding::encode(scope)
    );

    res.redirect(&github_auth_url)
}
