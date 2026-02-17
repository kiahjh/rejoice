//! GitHub webhook handler.
//!
//! Receives webhook events from GitHub and triggers appropriate actions:
//! - `push` to main branch → trigger production deployment
//! - `pull_request` opened/updated → trigger preview deployment
//! - `pull_request` closed → destroy preview environment
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

/// GitHub pull_request event payload (simplified).
#[derive(Deserialize, Debug)]
struct PullRequestEvent {
    action: String, // "opened", "synchronize", "closed", "reopened"
    number: i64,
    pull_request: PullRequestInfo,
    repository: PushRepository,
    installation: Option<InstallationRef>,
}

#[derive(Deserialize, Debug)]
struct PullRequestInfo {
    head: PullRequestHead,
    #[allow(dead_code)]
    base: PullRequestBase,
    title: String,
    merged: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct PullRequestHead {
    sha: String,
    #[serde(rename = "ref")]
    git_ref: String, // branch name
}

#[derive(Deserialize, Debug)]
struct PullRequestBase {
    #[serde(rename = "ref")]
    #[allow(dead_code)]
    git_ref: String, // e.g., "main"
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
        "pull_request" => handle_pull_request_event(&state, &body_bytes).await,
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
        is_preview: false,
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

// =============================================================================
// Pull Request Event Handling (Preview Deployments)
// =============================================================================

/// Handle a pull_request event - create/update/destroy preview deployments.
async fn handle_pull_request_event(state: &AppState, body: &[u8]) -> Res {
    let res = Res::new();

    let pr_event: PullRequestEvent = match serde_json::from_slice(body) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to parse pull_request event: {}", e);
            return res.bad_request("Invalid pull_request event payload");
        }
    };

    println!(
        "Pull request event: {} #{} on {} ({})",
        pr_event.action,
        pr_event.number,
        pr_event.repository.full_name,
        pr_event.pull_request.head.git_ref
    );

    let installation_id = match &pr_event.installation {
        Some(inst) => inst.id,
        None => {
            eprintln!("Pull request event missing installation ID");
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
    .bind(&pr_event.repository.full_name)
    .bind(installation_id)
    .fetch_optional(&state.db)
    .await;

    let project = match project {
        Ok(Some(p)) => p,
        Ok(None) => {
            println!(
                "No project found for repo {} with installation {}",
                pr_event.repository.full_name, installation_id
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

    // Only create previews if the project has been deployed at least once
    if project.fly_app_name.is_none() {
        println!("Project '{}' has never been deployed, skipping preview", project.name);
        return res.json(&serde_json::json!({
            "status": "ignored",
            "reason": "project not yet deployed"
        }));
    }

    match pr_event.action.as_str() {
        "opened" | "synchronize" | "reopened" => {
            handle_preview_deploy(state, &pr_event, &project.id, &project.name, installation_id).await
        }
        "closed" => {
            handle_preview_close(state, &pr_event, &project.id, &project.name, installation_id).await
        }
        _ => {
            println!("Ignoring pull_request action: {}", pr_event.action);
            res.json(&serde_json::json!({
                "status": "ignored",
                "reason": format!("unhandled action: {}", pr_event.action)
            }))
        }
    }
}

/// Handle preview deployment for a PR open or update.
async fn handle_preview_deploy(
    state: &AppState,
    pr_event: &PullRequestEvent,
    project_id: &str,
    project_name: &str,
    installation_id: i64,
) -> Res {
    let res = Res::new();
    let pr_number = pr_event.number;
    let branch = &pr_event.pull_request.head.git_ref;
    let sha = &pr_event.pull_request.head.sha;
    let pr_title = &pr_event.pull_request.title;

    // Generate preview app name: pr-{N}-rejoice-{project_id_prefix}
    // Fly app names must be <= 30 chars, lowercase, alphanumeric + hyphens
    let preview_app_name = preview_app_name(pr_number, project_id);

    println!(
        "Creating/updating preview for PR #{} on project '{}' (app: {})",
        pr_number, project_name, preview_app_name
    );

    // Check if preview environment already exists
    #[derive(FromRow)]
    struct ExistingPreview {
        id: String,
        #[allow(dead_code)]
        fly_app_name: String,
        github_comment_id: Option<i64>,
    }

    let existing = query_as::<_, ExistingPreview>(
        "SELECT id, fly_app_name, github_comment_id FROM preview_environments WHERE project_id = ? AND pr_number = ? AND status = 'active'",
    )
    .bind(project_id)
    .bind(pr_number)
    .fetch_optional(&state.db)
    .await;

    let (preview_id, is_first_preview) = match existing {
        Ok(Some(env)) => (env.id, false),
        Ok(None) => {
            // Create new preview environment record
            let preview_id = Uuid::new_v4().to_string();
            let insert = query(
                r#"
                INSERT INTO preview_environments (id, project_id, pr_number, pr_branch, fly_app_name, url)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&preview_id)
            .bind(project_id)
            .bind(pr_number)
            .bind(branch)
            .bind(&preview_app_name)
            .bind(format!("https://{}.fly.dev", preview_app_name))
            .execute(&state.db)
            .await;

            if let Err(e) = insert {
                eprintln!("Failed to create preview environment record: {}", e);
                return res.internal_error("Failed to create preview record");
            }

            (preview_id, true)
        }
        Err(e) => {
            eprintln!("Database error checking preview: {}", e);
            return res.internal_error("Database error");
        }
    };

    // Get environment variables for this project (include preview-only ones!)
    #[derive(FromRow)]
    struct EnvVar {
        key: String,
        encrypted_value: Vec<u8>,
    }

    let env_vars_result = query_as::<_, EnvVar>(
        "SELECT key, encrypted_value FROM env_vars WHERE project_id = ?",
    )
    .bind(project_id)
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

    // Get authenticated clone URL
    let clone_url = {
        let parts: Vec<&str> = pr_event.repository.full_name.split('/').collect();
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
    let commit_message = format!("PR #{}: {}", pr_number, pr_title);

    let insert_result = query(
        r#"
        INSERT INTO deployments (id, project_id, git_sha, git_branch, git_message, pr_number, status)
        VALUES (?, ?, ?, ?, ?, ?, 'building')
        "#,
    )
    .bind(&deployment_id)
    .bind(project_id)
    .bind(sha)
    .bind(branch)
    .bind(&commit_message)
    .bind(pr_number)
    .execute(&state.db)
    .await;

    if let Err(e) = insert_result {
        eprintln!("Failed to create deployment record: {}", e);
        return res.internal_error("Failed to create deployment");
    }

    // Spawn the preview deployment in a background task
    let deploy_config = DeployConfig {
        github_repo: pr_event.repository.full_name.clone(),
        branch: branch.to_string(),
        fly_app_name: preview_app_name.clone(),
        fly_token: state.fly_token.clone(),
        fly_org: state.fly_org.clone(),
        env_vars,
        is_first_deploy: is_first_preview,
        clone_url,
        github_installation_id: Some(installation_id),
        is_preview: true,
    };

    let db = state.db.clone();
    let dep_id = deployment_id.clone();
    let github_app = state.github_app.clone();
    let repo_full_name = pr_event.repository.full_name.clone();
    let preview_id_clone = preview_id.clone();
    let preview_url = format!("https://{}.fly.dev", preview_app_name);

    // Get existing comment ID for updates
    let existing_comment_id = if !is_first_preview {
        query_as::<_, ExistingPreview>(
            "SELECT id, fly_app_name, github_comment_id FROM preview_environments WHERE id = ?",
        )
        .bind(&preview_id)
        .fetch_optional(&state.db)
        .await
        .ok()
        .flatten()
        .and_then(|e| e.github_comment_id)
    } else {
        None
    };

    tokio::spawn(async move {
        run_preview_deployment(
            deploy_config,
            db,
            dep_id,
            github_app,
            repo_full_name,
            pr_number,
            installation_id,
            preview_id_clone,
            preview_url,
            existing_comment_id,
        )
        .await;
    });

    res.json(&serde_json::json!({
        "status": "deploying_preview",
        "deployment_id": deployment_id,
        "project": project_name,
        "pr_number": pr_number,
        "preview_app": preview_app_name
    }))
}

/// Handle PR close - destroy the preview environment.
async fn handle_preview_close(
    state: &AppState,
    pr_event: &PullRequestEvent,
    project_id: &str,
    project_name: &str,
    installation_id: i64,
) -> Res {
    let res = Res::new();
    let pr_number = pr_event.number;

    println!(
        "Closing preview for PR #{} on project '{}'",
        pr_number, project_name
    );

    // Find the preview environment
    #[derive(FromRow)]
    struct PreviewEnv {
        id: String,
        fly_app_name: String,
        github_comment_id: Option<i64>,
    }

    let preview = query_as::<_, PreviewEnv>(
        "SELECT id, fly_app_name, github_comment_id FROM preview_environments WHERE project_id = ? AND pr_number = ? AND status = 'active'",
    )
    .bind(project_id)
    .bind(pr_number)
    .fetch_optional(&state.db)
    .await;

    let preview = match preview {
        Ok(Some(p)) => p,
        Ok(None) => {
            println!("No active preview found for PR #{}", pr_number);
            return res.json(&serde_json::json!({
                "status": "ignored",
                "reason": "no active preview"
            }));
        }
        Err(e) => {
            eprintln!("Database error finding preview: {}", e);
            return res.internal_error("Database error");
        }
    };

    // Mark as destroying
    let _ = query("UPDATE preview_environments SET status = 'destroying' WHERE id = ?")
        .bind(&preview.id)
        .execute(&state.db)
        .await;

    // Destroy the Fly app in a background task
    let fly_token = state.fly_token.clone();
    let fly_app_name = preview.fly_app_name.clone();
    let preview_id = preview.id.clone();
    let db = state.db.clone();
    let github_app = state.github_app.clone();
    let repo_full_name = pr_event.repository.full_name.clone();
    let comment_id = preview.github_comment_id;
    let merged = pr_event.pull_request.merged.unwrap_or(false);

    tokio::spawn(async move {
        destroy_preview(
            &fly_app_name,
            &fly_token,
            &preview_id,
            &db,
            &github_app,
            &repo_full_name,
            installation_id,
            pr_number,
            comment_id,
            merged,
        )
        .await;
    });

    res.json(&serde_json::json!({
        "status": "destroying_preview",
        "project": project_name,
        "pr_number": pr_number,
        "fly_app": preview.fly_app_name
    }))
}

/// Run a preview deployment and post/update a PR comment with the URL.
async fn run_preview_deployment(
    config: DeployConfig,
    db: Pool<Sqlite>,
    deployment_id: String,
    github_app: crate::github::GitHubApp,
    repo_full_name: String,
    pr_number: i64,
    installation_id: i64,
    preview_id: String,
    preview_url: String,
    existing_comment_id: Option<i64>,
) {
    let fly_app_name = config.fly_app_name.clone();
    let branch = config.branch.clone();

    // Run the deployment
    let result =
        crate::deployer::deploy(config, Some(db.clone()), Some(deployment_id.clone()), Some(github_app.clone())).await;

    // Update deployment record
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

        // Post or update PR comment with preview URL
        let parts: Vec<&str> = repo_full_name.split('/').collect();
        if parts.len() == 2 {
            let comment_body = format!(
                "### Rejoice Cloud Preview\n\n\
                 | | |\n\
                 |---|---|\n\
                 | **Status** | Deployed |\n\
                 | **URL** | {} |\n\
                 | **Branch** | `{}` |\n\
                 | **App** | `{}` |\n\n\
                 This preview will be automatically removed when the PR is closed.",
                preview_url, branch, fly_app_name
            );

            if let Some(comment_id) = existing_comment_id {
                // Update existing comment
                let _ = github_app
                    .update_pr_comment(installation_id, parts[0], parts[1], comment_id, &comment_body)
                    .await;
            } else {
                // Create new comment
                match github_app
                    .create_pr_comment(installation_id, parts[0], parts[1], pr_number, &comment_body)
                    .await
                {
                    Ok(comment_id) => {
                        // Store comment ID for future updates
                        let _ = query(
                            "UPDATE preview_environments SET github_comment_id = ? WHERE id = ?",
                        )
                        .bind(comment_id)
                        .bind(&preview_id)
                        .execute(&db)
                        .await;
                    }
                    Err(e) => eprintln!("Failed to post PR comment: {}", e),
                }
            }
        }
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

        // Post failure comment
        let parts: Vec<&str> = repo_full_name.split('/').collect();
        if parts.len() == 2 {
            let error_msg = result.error.as_deref().unwrap_or("Unknown error");
            let comment_body = format!(
                "### Rejoice Cloud Preview\n\n\
                 | | |\n\
                 |---|---|\n\
                 | **Status** | Failed |\n\
                 | **Branch** | `{}` |\n\
                 | **Error** | {} |\n\n\
                 Push a new commit to retry.",
                branch, error_msg
            );

            if let Some(comment_id) = existing_comment_id {
                let _ = github_app
                    .update_pr_comment(installation_id, parts[0], parts[1], comment_id, &comment_body)
                    .await;
            } else {
                match github_app
                    .create_pr_comment(installation_id, parts[0], parts[1], pr_number, &comment_body)
                    .await
                {
                    Ok(comment_id) => {
                        let _ = query(
                            "UPDATE preview_environments SET github_comment_id = ? WHERE id = ?",
                        )
                        .bind(comment_id)
                        .bind(&preview_id)
                        .execute(&db)
                        .await;
                    }
                    Err(e) => eprintln!("Failed to post PR failure comment: {}", e),
                }
            }
        }
    }
}

/// Destroy a preview environment: delete the Fly app and update the database.
async fn destroy_preview(
    fly_app_name: &str,
    fly_token: &str,
    preview_id: &str,
    db: &Pool<Sqlite>,
    github_app: &crate::github::GitHubApp,
    repo_full_name: &str,
    installation_id: i64,
    _pr_number: i64,
    comment_id: Option<i64>,
    merged: bool,
) {
    // Delete the Fly app using flyctl
    let output = tokio::process::Command::new("flyctl")
        .args(["apps", "destroy", fly_app_name, "-y"])
        .env("FLY_API_TOKEN", fly_token)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            println!("Destroyed Fly app: {}", fly_app_name);
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!(
                "Warning: Failed to destroy Fly app {}: {}",
                fly_app_name, stderr
            );
        }
        Err(e) => {
            eprintln!("Failed to run flyctl destroy for {}: {}", fly_app_name, e);
        }
    }

    // Update database record
    let _ = query(
        "UPDATE preview_environments SET status = 'destroyed', destroyed_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(preview_id)
    .execute(db)
    .await;

    // Update PR comment
    let parts: Vec<&str> = repo_full_name.split('/').collect();
    if parts.len() == 2 {
        if let Some(comment_id) = comment_id {
            let status_text = if merged { "merged" } else { "closed" };
            let comment_body = format!(
                "### Rejoice Cloud Preview\n\n\
                 | | |\n\
                 |---|---|\n\
                 | **Status** | Removed (PR {}) |\n\n\
                 The preview environment has been cleaned up.",
                status_text
            );
            let _ = github_app
                .update_pr_comment(installation_id, parts[0], parts[1], comment_id, &comment_body)
                .await;
        }
    }
}

/// Generate a preview app name from a PR number and project ID.
/// Exposed for testing.
fn preview_app_name(pr_number: i64, project_id: &str) -> String {
    format!("pr-{}-rejoice-{}", pr_number, &project_id[..8])
}

// =============================================================================
// Production Deployment Background Task
// =============================================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pull_request_event_deserialization() {
        let json = r#"{
            "action": "opened",
            "number": 42,
            "pull_request": {
                "head": {
                    "sha": "abc123def456",
                    "ref": "feature/my-branch"
                },
                "base": {
                    "ref": "main"
                },
                "title": "Add new feature",
                "merged": false
            },
            "repository": {
                "full_name": "user/repo"
            },
            "installation": {
                "id": 12345
            }
        }"#;

        let event: PullRequestEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.action, "opened");
        assert_eq!(event.number, 42);
        assert_eq!(event.pull_request.head.sha, "abc123def456");
        assert_eq!(event.pull_request.head.git_ref, "feature/my-branch");
        assert_eq!(event.pull_request.base.git_ref, "main");
        assert_eq!(event.pull_request.title, "Add new feature");
        assert_eq!(event.pull_request.merged, Some(false));
        assert_eq!(event.repository.full_name, "user/repo");
        assert_eq!(event.installation.unwrap().id, 12345);
    }

    #[test]
    fn test_pull_request_event_closed_merged() {
        let json = r#"{
            "action": "closed",
            "number": 7,
            "pull_request": {
                "head": {
                    "sha": "deadbeef",
                    "ref": "fix/bug"
                },
                "base": {
                    "ref": "main"
                },
                "title": "Fix critical bug",
                "merged": true
            },
            "repository": {
                "full_name": "org/project"
            },
            "installation": {
                "id": 99999
            }
        }"#;

        let event: PullRequestEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.action, "closed");
        assert_eq!(event.pull_request.merged, Some(true));
    }

    #[test]
    fn test_pull_request_event_without_merged_field() {
        // GitHub may not include "merged" for opened/synchronize events
        let json = r#"{
            "action": "synchronize",
            "number": 10,
            "pull_request": {
                "head": {
                    "sha": "abc123",
                    "ref": "feature/update"
                },
                "base": {
                    "ref": "main"
                },
                "title": "Update stuff"
            },
            "repository": {
                "full_name": "user/repo"
            }
        }"#;

        let event: PullRequestEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.action, "synchronize");
        assert_eq!(event.pull_request.merged, None);
        assert!(event.installation.is_none());
    }

    #[test]
    fn test_preview_app_name_generation() {
        let name = preview_app_name(42, "abcdef12-3456-7890-abcd-ef1234567890");
        assert_eq!(name, "pr-42-rejoice-abcdef12");
    }

    #[test]
    fn test_preview_app_name_single_digit_pr() {
        let name = preview_app_name(1, "11111111-2222-3333-4444-555555555555");
        assert_eq!(name, "pr-1-rejoice-11111111");
    }

    #[test]
    fn test_preview_app_name_large_pr_number() {
        let name = preview_app_name(9999, "aabbccdd-eeff-0011-2233-445566778899");
        assert_eq!(name, "pr-9999-rejoice-aabbccdd");
        // Fly app names must be <= 30 chars
        assert!(name.len() <= 30);
    }

    #[test]
    fn test_push_event_deserialization() {
        let json = r#"{
            "ref": "refs/heads/main",
            "after": "abc123def456789",
            "repository": {
                "full_name": "user/repo"
            },
            "installation": {
                "id": 12345
            },
            "head_commit": {
                "message": "Initial commit\n\nWith a longer description"
            }
        }"#;

        let event: PushEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.git_ref, "refs/heads/main");
        assert_eq!(event.after, "abc123def456789");
        assert_eq!(event.repository.full_name, "user/repo");
        assert_eq!(event.installation.unwrap().id, 12345);
        assert_eq!(
            event.head_commit.unwrap().message,
            "Initial commit\n\nWith a longer description"
        );
    }
}
