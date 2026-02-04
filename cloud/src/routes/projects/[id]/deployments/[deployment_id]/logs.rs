//! Endpoint for polling deployment logs.

use crate::AppState;
use rejoice::db::{query_scalar, FromRow};
use rejoice::{Req, Res};

/// GET /projects/:id/deployments/:deployment_id/logs
/// Returns the current logs as JSON. Client polls this endpoint.
pub async fn get(
    state: AppState,
    req: Req,
    res: Res,
    id: String,
    deployment_id: String,
) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.json(&LogsResponse {
            logs: None,
            status: "unauthorized".to_string(),
            finished: true,
        }),
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
        return res.json(&LogsResponse {
            logs: None,
            status: "forbidden".to_string(),
            finished: true,
        });
    }

    // Fetch deployment logs and status
    #[derive(FromRow)]
    struct DeploymentLogs {
        build_logs: Option<String>,
        status: String,
    }

    let deployment = rejoice::db::query_as::<_, DeploymentLogs>(
        "SELECT build_logs, status FROM deployments WHERE id = ? AND project_id = ?",
    )
    .bind(&deployment_id)
    .bind(&id)
    .fetch_optional(&state.db)
    .await;

    match deployment {
        Ok(Some(d)) => {
            let finished = !matches!(d.status.as_str(), "pending" | "building" | "deploying");
            res.json(&LogsResponse {
                logs: d.build_logs,
                status: d.status,
                finished,
            })
        }
        Ok(None) => res.json(&LogsResponse {
            logs: None,
            status: "not_found".to_string(),
            finished: true,
        }),
        Err(_) => res.json(&LogsResponse {
            logs: None,
            status: "error".to_string(),
            finished: true,
        }),
    }
}

#[derive(serde::Serialize)]
struct LogsResponse {
    logs: Option<String>,
    status: String,
    finished: bool,
}
