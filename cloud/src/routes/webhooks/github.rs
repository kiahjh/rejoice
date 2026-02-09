//! GitHub webhook handler.
//!
//! Receives webhook events from GitHub and triggers appropriate actions:
//! - `push` to main branch → trigger deployment
//! - `pull_request` opened/updated → trigger preview deployment (future)
//! - `installation` → handle app install/uninstall

use crate::deployer::DeployConfig;
use crate::AppState;
use hmac::{Hmac, Mac};
use rejoice::db::{query, query_as, FromRow, Pool, Sqlite};
use rejoice::{Req, Res};
use serde::Deserialize;
use sha2::Sha256;
use uuid::Uuid;

const WEBHOOK_SECRET: &str = rejoice::env!("GITHUB_WEBHOOK_SECRET");

/// Verify the webhook signature from GitHub.
fn verify_signature(payload: &[u8], signature: &str) -> bool {
    // Signature format: "sha256=HEXDIGEST"
    let Some(hex_digest) = signature.strip_prefix("sha256=") else {
        return false;
    };

    let Ok(expected) = hex::decode(hex_digest) else {
        return false;
    };

    let mut mac = Hmac::<Sha256>::new_from_slice(WEBHOOK_SECRET.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload);

    mac.verify_slice(&expected).is_ok()
}

/// GitHub push event payload (simplified).
#[derive(Deserialize, Debug)]
struct PushEvent {
    #[serde(rename = "ref")]
    git_ref: String,
    after: String, // commit SHA
    repository: PushRepository,
    installation: Option<InstallationRef>,
    head_commit: Option<HeadCommit>,
}

#[derive(Deserialize, Debug)]
struct PushRepository {
    full_name: String,
}

#[derive(Deserialize, Debug)]
struct InstallationRef {
    id: i64,
}

#[derive(Deserialize, Debug)]
struct HeadCommit {
    message: String,
}

/// POST /webhooks/github - Handle incoming GitHub webhooks
pub async fn post(state: AppState, req: Req, res: Res) -> Res {
    // Get the signature header
    let signature = match req.headers.get("x-hub-signature-256") {
        Some(sig) => sig.to_str().unwrap_or(""),
        None => {
            eprintln!("Webhook missing signature header");
            return res.unauthorized("Missing signature");
        }
    };

    // Get the event type
    let event_type = match req.headers.get("x-github-event") {
        Some(event) => event.to_str().unwrap_or("unknown"),
        None => {
            eprintln!("Webhook missing event header");
            return res.bad_request("Missing event type");
        }
    };

    // Get raw body for signature verification
    let body_bytes = req.body.as_bytes();

    // Verify signature
    if !verify_signature(&body_bytes, signature) {
        eprintln!("Webhook signature verification failed");
        return res.unauthorized("Invalid signature");
    }

    println!("Received GitHub webhook: {}", event_type);

    // Handle different event types
    match event_type {
        "push" => handle_push_event(&state, &body_bytes).await,
        "ping" => {
            println!("GitHub webhook ping received - webhook is configured correctly!");
            res.json(&serde_json::json!({"status": "pong"}))
        }
        "installation" => {
            println!("GitHub App installation event received");
            res.json(&serde_json::json!({"status": "ok"}))
        }
        _ => {
            println!("Ignoring unhandled event type: {}", event_type);
            res.json(&serde_json::json!({"status": "ignored", "event": event_type}))
        }
    }
}

/// Handle a push event - trigger deployment if pushing to main branch.
async fn handle_push_event(state: &AppState, body: &[u8]) -> Res {
    let res = Res::new();

    let push: PushEvent = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to parse push event: {}", e);
            return res.bad_request("Invalid push event payload");
        }
    };

    println!(
        "Push event: {} -> {} ({})",
        push.repository.full_name,
        push.git_ref,
        &push.after[..7]
    );

    // Only deploy on push to main or master branch
    let branch = push.git_ref.strip_prefix("refs/heads/").unwrap_or(&push.git_ref);
    if branch != "main" && branch != "master" {
        println!("Ignoring push to non-default branch: {}", branch);
        return res.json(&serde_json::json!({
            "status": "ignored",
            "reason": "not default branch"
        }));
    }

    // Get installation ID
    let installation_id = match push.installation {
        Some(inst) => inst.id,
        None => {
            eprintln!("Push event missing installation ID");
            return res.bad_request("Missing installation ID");
        }
    };

    // Find project by repo name and installation ID
    #[derive(FromRow)]
    struct Project {
        id: String,
        name: String,
        fly_app_name: Option<String>,
    }

    let project = query_as::<_, Project>(
        "SELECT id, name, fly_app_name FROM projects WHERE github_repo = ? AND github_installation_id = ?",
    )
    .bind(&push.repository.full_name)
    .bind(installation_id)
    .fetch_optional(&state.db)
    .await;

    let project = match project {
        Ok(Some(p)) => p,
        Ok(None) => {
            println!(
                "No project found for repo {} with installation {}",
                push.repository.full_name, installation_id
            );
            return res.json(&serde_json::json!({
                "status": "ignored",
                "reason": "no matching project"
            }));
        }
        Err(e) => {
            eprintln!("Database error finding project: {}", e);
            return res.internal_error("Database error");
        }
    };

    println!("Found project '{}' ({}), triggering deployment...", project.name, project.id);

    // Get environment variables for this project
    #[derive(FromRow)]
    struct EnvVar {
        key: String,
        encrypted_value: Vec<u8>,
    }

    let env_vars_result = query_as::<_, EnvVar>(
        "SELECT key, encrypted_value FROM env_vars WHERE project_id = ? AND is_preview_only = FALSE",
    )
    .bind(&project.id)
    .fetch_all(&state.db)
    .await;

    let env_vars: Vec<(String, String)> = match env_vars_result {
        Ok(vars) => vars
            .into_iter()
            .filter_map(|v| {
                crate::crypto::decrypt(&v.encrypted_value, &state.encryption_key)
                    .ok()
                    .map(|value| (v.key, value))
            })
            .collect(),
        Err(_) => vec![],
    };

    let is_first_deploy = project.fly_app_name.is_none();
    let fly_app_name = project
        .fly_app_name
        .clone()
        .unwrap_or_else(|| format!("rejoice-{}", &project.id[..8]));

    // Get authenticated clone URL
    let clone_url = {
        let parts: Vec<&str> = push.repository.full_name.split('/').collect();
        if parts.len() == 2 {
            match state
                .github_app
                .get_authenticated_clone_url(installation_id, parts[0], parts[1])
                .await
            {
                Ok(url) => Some(url),
                Err(e) => {
                    eprintln!("Failed to get authenticated clone URL: {}", e);
                    None
                }
            }
        } else {
            None
        }
    };

    // Create deployment record
    let deployment_id = Uuid::new_v4().to_string();
    let commit_message = push
        .head_commit
        .as_ref()
        .map(|c| c.message.lines().next().unwrap_or("").to_string())
        .unwrap_or_default();

    let insert_result = query(
        r#"
        INSERT INTO deployments (id, project_id, git_sha, git_branch, git_message, status)
        VALUES (?, ?, ?, ?, ?, 'building')
        "#,
    )
    .bind(&deployment_id)
    .bind(&project.id)
    .bind(&push.after)
    .bind(branch)
    .bind(&commit_message)
    .execute(&state.db)
    .await;

    if let Err(e) = insert_result {
        eprintln!("Failed to create deployment record: {}", e);
        return res.internal_error("Failed to create deployment");
    }

    // Update project with fly_app_name if first deploy
    if is_first_deploy {
        let _ = query("UPDATE projects SET fly_app_name = ? WHERE id = ?")
            .bind(&fly_app_name)
            .execute(&state.db)
            .await;
    }

    // Spawn the deployment in a background task
    let deploy_config = DeployConfig {
        github_repo: push.repository.full_name.clone(),
        branch: branch.to_string(),
        fly_app_name: fly_app_name.clone(),
        fly_token: state.fly_token.clone(),
        fly_org: state.fly_org.clone(),
        env_vars,
        is_first_deploy,
        clone_url,
        github_installation_id: Some(installation_id),
    };

    let db = state.db.clone();
    let dep_id = deployment_id.clone();
    let github_app = state.github_app.clone();

    tokio::spawn(async move {
        run_deployment(deploy_config, db, dep_id, github_app).await;
    });

    res.json(&serde_json::json!({
        "status": "deploying",
        "deployment_id": deployment_id,
        "project": project.name
    }))
}

/// Run the deployment in the background and update the database with results.
async fn run_deployment(
    config: DeployConfig,
    db: Pool<Sqlite>,
    deployment_id: String,
    github_app: crate::github::GitHubApp,
) {
    let fly_app_name = config.fly_app_name.clone();

    // Run the deployment with streaming logs
    let result =
        crate::deployer::deploy(config, Some(db.clone()), Some(deployment_id.clone()), Some(github_app)).await;

    // Update deployment record with results
    if result.success {
        let _ = query(
            r#"
            UPDATE deployments 
            SET status = 'success', 
                url = ?,
                build_logs = ?,
                finished_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(&result.url)
        .bind(&result.logs)
        .bind(&deployment_id)
        .execute(&db)
        .await;

        // Ensure fly_app_name is set on the project (safety net)
        let _ = query(
            "UPDATE projects SET fly_app_name = ? WHERE id = (SELECT project_id FROM deployments WHERE id = ?) AND (fly_app_name IS NULL OR fly_app_name = '')",
        )
        .bind(&fly_app_name)
        .bind(&deployment_id)
        .execute(&db)
        .await;
    } else {
        let _ = query(
            r#"
            UPDATE deployments 
            SET status = 'failed', 
                error_message = ?,
                build_logs = ?,
                finished_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(&result.error)
        .bind(&result.logs)
        .bind(&deployment_id)
        .execute(&db)
        .await;
    }
}
