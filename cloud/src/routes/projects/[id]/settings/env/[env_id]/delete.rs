use crate::AppState;
use rejoice::db::{query, query_scalar};
use rejoice::{Req, Res};

pub async fn post(state: AppState, req: Req, res: Res, id: String, env_id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user
    let owns_project: bool = query_scalar(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM projects p
            JOIN users u ON p.user_id = u.id
            WHERE p.id = ? AND u.github_id = ?
        )
        "#,
    )
    .bind(&id)
    .bind(github_id)
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

    if !owns_project {
        return res.forbidden("Not authorized");
    }

    // Delete the env var (ensuring it belongs to this project)
    let result = query(
        r#"
        DELETE FROM env_vars
        WHERE id = ? AND project_id = ?
        "#,
    )
    .bind(&env_id)
    .bind(&id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => res.redirect(&format!("/projects/{}/settings", id)),
        Err(_) => res.internal_error("Failed to delete environment variable"),
    }
}
