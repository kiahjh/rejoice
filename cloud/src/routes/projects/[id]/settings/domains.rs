//! Route for adding a custom domain to a project.

use crate::AppState;
use rejoice::db::{query, query_as, FromRow};
use rejoice::{Req, Res};
use uuid::Uuid;

#[derive(FromRow)]
struct Project {
    id: String,
    fly_app_name: Option<String>,
}

/// POST /projects/:id/settings/domains - Add a custom domain
pub async fn post(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user
    let project = query_as::<_, Project>(
        r#"
        SELECT p.id, p.fly_app_name
        FROM projects p
        JOIN users u ON p.user_id = u.id
        WHERE p.id = ? AND u.github_id = ?
        "#,
    )
    .bind(&id)
    .bind(github_id)
    .fetch_optional(&state.db)
    .await;

    let project = match project {
        Ok(Some(p)) => p,
        Ok(None) => return res.not_found("Project not found"),
        Err(_) => return res.internal_error("Database error"),
    };

    let fly_app_name = match &project.fly_app_name {
        Some(name) => name.clone(),
        None => {
            return res.redirect(&format!(
                "/projects/{}/settings?error=deploy_first",
                project.id
            ))
        }
    };

    // Parse form data
    let form = req.body.as_form::<DomainForm>();
    let form = match form {
        Ok(f) => f,
        Err(_) => {
            return res.redirect(&format!(
                "/projects/{}/settings?error=invalid_form",
                project.id
            ))
        }
    };

    // Validate hostname
    let hostname = form.hostname.trim().to_lowercase();
    if !is_valid_hostname(&hostname) {
        return res.redirect(&format!(
            "/projects/{}/settings?error=invalid_hostname",
            project.id
        ));
    }

    // Check if domain already exists
    let existing = query_as::<_, DomainCheck>(
        "SELECT id FROM custom_domains WHERE hostname = ?",
    )
    .bind(&hostname)
    .fetch_optional(&state.db)
    .await;

    if let Ok(Some(_)) = existing {
        return res.redirect(&format!(
            "/projects/{}/settings?error=domain_exists",
            project.id
        ));
    }

    // Add certificate via Fly.io GraphQL API
    let cert_result = state.fly.add_certificate(&fly_app_name, &hostname).await;

    match cert_result {
        Ok(cert) => {
            let domain_id = Uuid::new_v4().to_string();
            let status = cert
                .client_status
                .as_deref()
                .map(|s| {
                    if s == "Ready" {
                        "ready"
                    } else {
                        "pending"
                    }
                })
                .unwrap_or("pending");

            let _ = query(
                r#"
                INSERT INTO custom_domains (
                    id, project_id, hostname, fly_cert_id, status,
                    dns_validation_hostname, dns_validation_target,
                    configured, acme_dns_configured, certificate_authority,
                    checked_at
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
                "#,
            )
            .bind(&domain_id)
            .bind(&project.id)
            .bind(&hostname)
            .bind(&cert.id)
            .bind(status)
            .bind(&cert.dns_validation_hostname)
            .bind(&cert.dns_validation_target)
            .bind(cert.configured)
            .bind(cert.acme_dns_configured)
            .bind(&cert.certificate_authority)
            .execute(&state.db)
            .await;

            res.redirect(&format!(
                "/projects/{}/settings?domain_added=true",
                project.id
            ))
        }
        Err(e) => {
            eprintln!("Failed to add certificate for {}: {}", hostname, e);
            res.redirect(&format!(
                "/projects/{}/settings?error=cert_failed",
                project.id
            ))
        }
    }
}

#[derive(serde::Deserialize)]
struct DomainForm {
    hostname: String,
}

#[derive(FromRow)]
struct DomainCheck {
    #[allow(dead_code)]
    id: String,
}

/// Basic hostname validation.
fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }

    // Must contain at least one dot (not just "localhost")
    if !hostname.contains('.') {
        return false;
    }

    // Must not start or end with a dot or hyphen
    if hostname.starts_with('.') || hostname.ends_with('.') {
        return false;
    }

    // Each label must be valid
    for label in hostname.split('.') {
        if label.is_empty() || label.len() > 63 {
            return false;
        }
        if label.starts_with('-') || label.ends_with('-') {
            return false;
        }
        if !label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_hostnames() {
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("www.example.com"));
        assert!(is_valid_hostname("sub.domain.example.com"));
        assert!(is_valid_hostname("my-app.example.com"));
        assert!(is_valid_hostname("a.co"));
    }

    #[test]
    fn test_invalid_hostnames() {
        assert!(!is_valid_hostname(""));
        assert!(!is_valid_hostname("localhost"));
        assert!(!is_valid_hostname(".example.com"));
        assert!(!is_valid_hostname("example.com."));
        assert!(!is_valid_hostname("-example.com"));
        assert!(!is_valid_hostname("example-.com"));
        assert!(!is_valid_hostname("exam ple.com"));
        assert!(!is_valid_hostname("example..com"));
        let long_label = "a".repeat(64) + ".com";
        assert!(!is_valid_hostname(&long_label));
    }
}
