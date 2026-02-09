//! Route for re-checking a custom domain's certificate status.

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

/// POST /projects/:id/settings/domains/:domain_id/check - Re-check certificate status
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

    let fly_app_name = match project {
        Ok(Some(p)) => match p.fly_app_name {
            Some(name) => name,
            None => {
                return res.redirect(&format!(
                    "/projects/{}/settings?error=no_fly_app",
                    id
                ))
            }
        },
        _ => {
            return res.redirect(&format!(
                "/projects/{}/settings?error=project_error",
                id
            ))
        }
    };

    // Check certificate status via Fly API (triggers re-validation)
    match state
        .fly
        .check_certificate(&fly_app_name, &domain.hostname)
        .await
    {
        Ok(cert) => {
            let status = cert.status_display();

            let _ = query(
                r#"
                UPDATE custom_domains
                SET status = ?,
                    configured = ?,
                    acme_dns_configured = ?,
                    certificate_authority = ?,
                    dns_validation_hostname = ?,
                    dns_validation_target = ?,
                    checked_at = CURRENT_TIMESTAMP
                WHERE id = ?
                "#,
            )
            .bind(status)
            .bind(cert.configured)
            .bind(cert.acme_dns_configured)
            .bind(&cert.certificate_authority)
            .bind(&cert.dns_validation_hostname)
            .bind(&cert.dns_validation_target)
            .bind(&domain_id)
            .execute(&state.db)
            .await;
        }
        Err(e) => {
            eprintln!(
                "Failed to check certificate for {}: {}",
                domain.hostname, e
            );
        }
    }

    res.redirect(&format!("/projects/{}/settings", id))
}
