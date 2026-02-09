//! Route for deleting a custom domain from a project.

use crate::AppState;
use rejoice::db::{query, query_as, FromRow};
use rejoice::{Req, Res};

#[derive(FromRow)]
struct DomainInfo {
    hostname: String,
    project_id: String,
}

#[derive(FromRow)]
struct ProjectInfo {
    fly_app_name: Option<String>,
}

/// POST /projects/:id/settings/domains/:domain_id/delete - Remove a custom domain
pub async fn post(
    state: AppState,
    req: Req,
    res: Res,
    id: String,
    domain_id: String,
) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user and get domain info
    let domain = query_as::<_, DomainInfo>(
        r#"
        SELECT cd.hostname, cd.project_id
        FROM custom_domains cd
        JOIN projects p ON cd.project_id = p.id
        JOIN users u ON p.user_id = u.id
        WHERE cd.id = ? AND p.id = ? AND u.github_id = ?
        "#,
    )
    .bind(&domain_id)
    .bind(&id)
    .bind(github_id)
    .fetch_optional(&state.db)
    .await;

    let domain = match domain {
        Ok(Some(d)) => d,
        Ok(None) => return res.not_found("Domain not found"),
        Err(_) => return res.internal_error("Database error"),
    };

    // Get the Fly app name
    let project = query_as::<_, ProjectInfo>(
        "SELECT fly_app_name FROM projects WHERE id = ?",
    )
    .bind(&domain.project_id)
    .fetch_optional(&state.db)
    .await;

    // Delete certificate from Fly.io
    if let Ok(Some(project)) = project {
        if let Some(fly_app_name) = project.fly_app_name {
            if let Err(e) = state
                .fly
                .delete_certificate(&fly_app_name, &domain.hostname)
                .await
            {
                eprintln!(
                    "Warning: Failed to delete certificate from Fly for {}: {}",
                    domain.hostname, e
                );
                // Continue with local deletion even if Fly API fails
            }
        }
    }

    // Delete from our database
    let _ = query("DELETE FROM custom_domains WHERE id = ?")
        .bind(&domain_id)
        .execute(&state.db)
        .await;

    res.redirect(&format!(
        "/projects/{}/settings?domain_deleted=true",
        id
    ))
}
