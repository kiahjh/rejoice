//! GitHub App installation callback.
//!
//! After a user installs the GitHub App on their account, GitHub redirects here
//! with the installation_id. We save this to enable repo access for the user.

use rejoice::db::query;
use rejoice::{Req, Res};
use serde::Deserialize;

use crate::AppState;

#[derive(Deserialize)]
struct CallbackQuery {
    installation_id: i64,
    // setup_action is "install" or "update"
    #[allow(dead_code)]
    setup_action: Option<String>,
}

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Must be logged in
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Parse query parameters
    let query_params: CallbackQuery = match req.uri.query() {
        Some(q) => match serde_urlencoded::from_str(q) {
            Ok(parsed) => parsed,
            Err(_) => return res.bad_request("Missing or invalid installation_id"),
        },
        None => return res.bad_request("Missing installation_id"),
    };

    // Save installation_id to user record
    let result = query(
        "UPDATE users SET github_app_installation_id = ? WHERE github_id = ?",
    )
    .bind(query_params.installation_id)
    .bind(github_id)
    .execute(&state.db)
    .await;

    if let Err(e) = result {
        eprintln!("Failed to save installation ID: {}", e);
        return res.internal_error("Failed to save GitHub App installation");
    }

    // Redirect to projects page with success message
    res.redirect("/projects?github_app_installed=true")
}
