use crate::AppState;
use rejoice::db::query;
use rejoice::{Req, Res};

pub async fn post(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user and delete it
    // The CASCADE on foreign keys will delete related env_vars and deployments
    let result = query(
        r#"
        DELETE FROM projects
        WHERE id = ? AND user_id = (
            SELECT id FROM users WHERE github_id = ?
        )
        "#,
    )
    .bind(&id)
    .bind(github_id)
    .execute(&state.db)
    .await;

    match result {
        Ok(r) if r.rows_affected() > 0 => res.redirect("/projects"),
        Ok(_) => res.forbidden("Not authorized to delete this project"),
        Err(_) => res.internal_error("Failed to delete project"),
    }
}
