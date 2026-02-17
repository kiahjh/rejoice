use crate::crypto;
use crate::AppState;
use rejoice::db::{query, query_scalar};
use rejoice::{Req, Res};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct AddEnvVarForm {
    key: String,
    value: String,
    #[serde(default)]
    preview_only: Option<String>, // Checkbox sends "on" or absent
}

pub async fn post(state: AppState, req: Req, res: Res, id: String) -> Res {
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

    // Parse form
    let form: AddEnvVarForm = match req.body.as_form() {
        Ok(f) => f,
        Err(_) => return res.bad_request("Invalid form data"),
    };

    // Validate key
    let key = form.key.trim().to_uppercase();
    if key.is_empty() {
        return res.redirect(&format!("/projects/{}/settings?error=empty_key", id));
    }

    // Validate key format (alphanumeric + underscore, must start with letter)
    if !is_valid_env_key(&key) {
        return res.redirect(&format!("/projects/{}/settings?error=invalid_key", id));
    }

    // Encrypt the value
    let encrypted = match crypto::encrypt(&form.value, &state.encryption_key) {
        Ok(e) => e,
        Err(_) => return res.internal_error("Encryption failed"),
    };

    let is_preview_only = form.preview_only.is_some();
    let env_id = Uuid::new_v4().to_string();

    // Insert or update (upsert)
    let result = query(
        r#"
        INSERT INTO env_vars (id, project_id, key, encrypted_value, is_preview_only)
        VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(project_id, key) DO UPDATE SET
            encrypted_value = excluded.encrypted_value,
            is_preview_only = excluded.is_preview_only,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(&env_id)
    .bind(&id)
    .bind(&key)
    .bind(&encrypted)
    .bind(is_preview_only)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => res.redirect(&format!("/projects/{}/settings", id)),
        Err(_) => res.internal_error("Failed to save environment variable"),
    }
}

fn is_valid_env_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }

    let mut chars = key.chars();

    // First char must be a letter
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => {}
        _ => return false,
    }

    // Rest must be alphanumeric or underscore
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
