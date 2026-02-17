use rejoice::db::query;
use rejoice::{Req, Res};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;

const GITHUB_CLIENT_ID: &str = rejoice::env!("GITHUB_CLIENT_ID");
const GITHUB_CLIENT_SECRET: &str = rejoice::env!("GITHUB_CLIENT_SECRET");

#[derive(Deserialize)]
struct CallbackQuery {
    code: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GitHubUser {
    id: i64,
    login: String,
    email: Option<String>,
}

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Parse the ?code= query parameter
    let query_params: CallbackQuery = match req.uri.query() {
        Some(q) => match serde_urlencoded::from_str(q) {
            Ok(parsed) => parsed,
            Err(_) => return res.bad_request("Missing or invalid code parameter"),
        },
        None => return res.bad_request("Missing code parameter"),
    };

    // Exchange code for access token
    let client = reqwest::Client::new();
    let token_response = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", GITHUB_CLIENT_ID),
            ("client_secret", GITHUB_CLIENT_SECRET),
            ("code", &query_params.code),
        ])
        .send()
        .await;

    let token_response = match token_response {
        Ok(resp) => resp,
        Err(_) => return res.internal_error("Failed to contact GitHub"),
    };

    let token: TokenResponse = match token_response.json().await {
        Ok(t) => t,
        Err(_) => return res.internal_error("Failed to parse GitHub token response"),
    };

    // Fetch user info
    let user_response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token.access_token))
        .header("User-Agent", "Rejoice-Cloud")
        .send()
        .await;

    let user_response = match user_response {
        Ok(resp) => resp,
        Err(_) => return res.internal_error("Failed to fetch GitHub user"),
    };

    let github_user: GitHubUser = match user_response.json().await {
        Ok(u) => u,
        Err(_) => return res.internal_error("Failed to parse GitHub user"),
    };

    // Upsert user in database
    let user_id = Uuid::new_v4().to_string();
    let result = query(
        r#"
        INSERT INTO users (id, github_id, github_username, email)
        VALUES (?, ?, ?, ?)
        ON CONFLICT(github_id) DO UPDATE SET
            github_username = excluded.github_username,
            email = excluded.email
        "#,
    )
    .bind(&user_id)
    .bind(github_user.id)
    .bind(&github_user.login)
    .bind(&github_user.email)
    .execute(&state.db)
    .await;

    if let Err(e) = result {
        eprintln!("Failed to upsert user: {}", e);
        return res.internal_error("Failed to save user");
    }

    // Set session cookie with user ID
    // For now, just use the github_id as the session token (we'll improve this later)
    res.set_cookie("session", &github_user.id.to_string())
        .redirect("/")
}
