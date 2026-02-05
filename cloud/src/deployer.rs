//! Deployment pipeline for Rejoice applications.
//!
//! This module handles the full deployment flow:
//! 1. Clone the GitHub repository
//! 2. Analyze project structure (client/, migrations/, etc.)
//! 3. Generate Dockerfile and fly.toml
//! 4. Run `flyctl deploy` to build and deploy
//! 5. Report status back

use crate::builder::{generate_dockerfile, generate_fly_toml, ProjectInfo};
use rejoice::db::{query, Pool, Sqlite};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Result of a deployment operation.
#[derive(Debug)]
pub struct DeployResult {
    /// Whether deployment succeeded
    pub success: bool,
    /// Deployment URL (if successful)
    pub url: Option<String>,
    /// Build/deploy logs
    pub logs: String,
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Configuration for a deployment.
pub struct DeployConfig {
    /// GitHub repository (e.g., "username/repo")
    pub github_repo: String,
    /// Branch to deploy
    pub branch: String,
    /// Fly app name
    pub fly_app_name: String,
    /// Fly API token
    pub fly_token: String,
    /// Fly organization slug
    pub fly_org: String,
    /// Environment variables to set on the app
    pub env_vars: Vec<(String, String)>,
    /// Whether this is the first deployment (need to create app)
    pub is_first_deploy: bool,
    /// Optional authenticated clone URL (for private repos)
    /// If provided, uses this instead of public https://github.com/... URL
    pub clone_url: Option<String>,
    /// GitHub App installation ID (for posting commit status)
    pub github_installation_id: Option<i64>,
}

/// Context for GitHub status updates
pub struct GitHubStatusContext {
    pub github_app: crate::github::GitHubApp,
    pub installation_id: i64,
    pub owner: String,
    pub repo: String,
    pub sha: String,
}

/// Deploy a Rejoice application to Fly.io.
/// If `db` and `deployment_id` are provided, logs will be streamed to the database.
/// If `github_app` is provided, commit status will be posted to GitHub.
pub async fn deploy(
    config: DeployConfig,
    db: Option<Pool<Sqlite>>,
    deployment_id: Option<String>,
    github_app: Option<crate::github::GitHubApp>,
) -> DeployResult {
    let mut logs = String::new();
    
    // Helper to update logs in database
    let update_logs = |db: &Option<Pool<Sqlite>>, dep_id: &Option<String>, logs: &str| {
        let db = db.clone();
        let dep_id = dep_id.clone();
        let logs = logs.to_string();
        async move {
            if let (Some(db), Some(id)) = (db, dep_id) {
                let _ = query("UPDATE deployments SET build_logs = ? WHERE id = ?")
                    .bind(&logs)
                    .bind(&id)
                    .execute(&db)
                    .await;
            }
        }
    };

    // Create temp directory for the build
    let temp_dir = match create_temp_dir().await {
        Ok(dir) => dir,
        Err(e) => {
            return DeployResult {
                success: false,
                url: None,
                logs,
                error: Some(format!("Failed to create temp directory: {}", e)),
            }
        }
    };

    logs.push_str(&format!("Created temp directory: {}\n", temp_dir.display()));
    update_logs(&db, &deployment_id, &logs).await;

    // Clone the repository
    logs.push_str(&format!(
        "Cloning {} (branch: {})...\n",
        config.github_repo, config.branch
    ));
    update_logs(&db, &deployment_id, &logs).await;

    match clone_repo(&config.github_repo, &config.branch, &temp_dir, config.clone_url.as_deref()).await {
        Ok(output) => {
            logs.push_str(&output);
            update_logs(&db, &deployment_id, &logs).await;
        }
        Err(e) => {
            cleanup_temp_dir(&temp_dir).await;
            return DeployResult {
                success: false,
                url: None,
                logs,
                error: Some(format!("Failed to clone repository: {}", e)),
            };
        }
    }

    // Get commit info and update deployment record
    let commit_sha = if let Some((sha, message)) = get_commit_info(&temp_dir).await {
        if let (Some(db), Some(dep_id)) = (&db, &deployment_id) {
            let _ = query("UPDATE deployments SET git_sha = ?, git_message = ? WHERE id = ?")
                .bind(&sha)
                .bind(&message)
                .bind(dep_id)
                .execute(db)
                .await;
        }
        Some(sha)
    } else {
        None
    };

    // Post pending status to GitHub
    if let (Some(github_app), Some(installation_id), Some(sha)) = (&github_app, config.github_installation_id, &commit_sha) {
        let parts: Vec<&str> = config.github_repo.split('/').collect();
        if parts.len() == 2 {
            let target_url = deployment_id.as_ref().map(|dep_id| {
                // TODO: Replace with actual cloud URL
                format!("http://localhost:3333/projects/{}/deployments/{}", "project_id", dep_id)
            });
            let _ = github_app.create_commit_status(
                installation_id,
                parts[0],
                parts[1],
                sha,
                crate::github::CommitStatusState::Pending,
                target_url.as_deref(),
                Some("Deployment in progress..."),
                "Rejoice Cloud",
            ).await;
        }
    }

    // Analyze project structure
    logs.push_str("\nAnalyzing project structure...\n");
    update_logs(&db, &deployment_id, &logs).await;
    let project_info = analyze_project(&temp_dir).await;
    logs.push_str(&format!(
        "  - Has client: {}\n  - Has public: {}\n  - Has database: {}\n  - Package: {}\n  - Port: {}\n",
        project_info.has_client,
        project_info.has_public,
        project_info.has_database,
        project_info.package_name.as_deref().unwrap_or("unknown"),
        project_info.port
    ));
    update_logs(&db, &deployment_id, &logs).await;

    // Generate Dockerfile
    logs.push_str("\nGenerating Dockerfile...\n");
    update_logs(&db, &deployment_id, &logs).await;
    let package_name = project_info
        .package_name
        .as_deref()
        .unwrap_or("app");
    let dockerfile = generate_dockerfile(
        package_name,
        project_info.has_client,
        project_info.has_database,
        project_info.has_public,
        project_info.port,
    );
    if let Err(e) = fs::write(temp_dir.join("Dockerfile"), &dockerfile).await {
        cleanup_temp_dir(&temp_dir).await;
        return DeployResult {
            success: false,
            url: None,
            logs,
            error: Some(format!("Failed to write Dockerfile: {}", e)),
        };
    }

    // Generate fly.toml
    logs.push_str("Generating fly.toml...\n");
    update_logs(&db, &deployment_id, &logs).await;
    let fly_toml = generate_fly_toml(&config.fly_app_name, project_info.has_database, project_info.port);
    if let Err(e) = fs::write(temp_dir.join("fly.toml"), &fly_toml).await {
        cleanup_temp_dir(&temp_dir).await;
        return DeployResult {
            success: false,
            url: None,
            logs,
            error: Some(format!("Failed to write fly.toml: {}", e)),
        };
    }

    // Set environment variables as secrets (if any)
    if !config.env_vars.is_empty() {
        logs.push_str("\nSetting secrets...\n");
        for (key, _value) in &config.env_vars {
            logs.push_str(&format!("  - {}\n", key));
        }
        update_logs(&db, &deployment_id, &logs).await;

        match set_fly_secrets(&config.fly_app_name, &config.fly_token, &config.env_vars, &temp_dir)
            .await
        {
            Ok(output) => {
                logs.push_str(&output);
                update_logs(&db, &deployment_id, &logs).await;
            }
            Err(e) => {
                // Secrets might fail if app doesn't exist yet, that's ok
                logs.push_str(&format!("  Warning: {}\n", e));
                update_logs(&db, &deployment_id, &logs).await;
            }
        }
    }

    // Create Fly app if this is the first deploy
    if config.is_first_deploy {
        logs.push_str("\nCreating Fly app...\n");
        update_logs(&db, &deployment_id, &logs).await;
        match create_fly_app(&config.fly_app_name, &config.fly_token, &config.fly_org).await {
            Ok(output) => {
                logs.push_str(&output);
                update_logs(&db, &deployment_id, &logs).await;
            }
            Err(e) => {
                // App might already exist, which is fine
                let already_exists = e.contains("already exists") 
                    || e.contains("already been taken")
                    || e.contains("Name has already been taken");
                if !already_exists {
                    cleanup_temp_dir(&temp_dir).await;
                    return DeployResult {
                        success: false,
                        url: None,
                        logs,
                        error: Some(format!("Failed to create Fly app: {}", e)),
                    };
                }
                logs.push_str("  App already exists, continuing...\n");
                update_logs(&db, &deployment_id, &logs).await;
            }
        }
    }

    // Run flyctl deploy
    logs.push_str("\nDeploying to Fly.io...\n");
    logs.push_str("This may take a few minutes...\n\n");
    update_logs(&db, &deployment_id, &logs).await;

    let result = match run_flyctl_deploy_streaming(&config, &temp_dir, &db, &deployment_id, &logs).await {
        Ok(output) => {
            logs.push_str(&output);
            logs.push_str("\n\nDeployment complete!\n");

            cleanup_temp_dir(&temp_dir).await;

            DeployResult {
                success: true,
                url: Some(format!("https://{}.fly.dev", config.fly_app_name)),
                logs,
                error: None,
            }
        }
        Err(e) => {
            logs.push_str(&e);
            logs.push_str("\n\nDeployment failed.\n");
            cleanup_temp_dir(&temp_dir).await;

            DeployResult {
                success: false,
                url: None,
                logs,
                error: Some("Deployment failed - see build logs for details".to_string()),
            }
        }
    };

    // Post final status to GitHub
    if let (Some(github_app), Some(installation_id), Some(sha)) = (&github_app, config.github_installation_id, &commit_sha) {
        let parts: Vec<&str> = config.github_repo.split('/').collect();
        if parts.len() == 2 {
            let (state, description) = if result.success {
                (crate::github::CommitStatusState::Success, "Deployment successful")
            } else {
                (crate::github::CommitStatusState::Failure, "Deployment failed")
            };
            let target_url = result.url.as_deref();
            let _ = github_app.create_commit_status(
                installation_id,
                parts[0],
                parts[1],
                sha,
                state,
                target_url,
                Some(description),
                "Rejoice Cloud",
            ).await;
        }
    }

    result
}

/// Create a temporary directory for the build.
async fn create_temp_dir() -> Result<PathBuf, std::io::Error> {
    let temp_base = std::env::temp_dir().join("rejoice-cloud-builds");
    fs::create_dir_all(&temp_base).await?;

    let dir_name = format!("build-{}", uuid::Uuid::new_v4());
    let temp_dir = temp_base.join(dir_name);
    fs::create_dir_all(&temp_dir).await?;

    Ok(temp_dir)
}

/// Clean up temporary directory.
async fn cleanup_temp_dir(path: &Path) {
    let _ = fs::remove_dir_all(path).await;
}

/// Clone a GitHub repository.
/// Tries the specified branch first, then falls back to common defaults (master, main).
/// If `clone_url` is provided, uses it instead of the default public URL (for private repos).
async fn clone_repo(repo: &str, branch: &str, dest: &Path, clone_url: Option<&str>) -> Result<String, String> {
    let url = clone_url
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("https://github.com/{}.git", repo));
    
    // Try branches in order: specified branch, then master, then main
    let branches_to_try = if branch == "master" {
        vec!["master", "main"]
    } else if branch == "main" {
        vec!["main", "master"]
    } else {
        vec![branch, "master", "main"]
    };

    let mut last_error = String::new();

    for try_branch in branches_to_try {
        let output = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "--branch",
                try_branch,
                &url,
                dest.to_str().unwrap(),
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| format!("Failed to run git: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            return Ok(format!("Cloned branch: {}\n{}{}", try_branch, stdout, stderr));
        }

        last_error = stderr.to_string();
        
        // Clean up the dest directory if it was partially created
        let _ = tokio::fs::remove_dir_all(dest).await;
        let _ = tokio::fs::create_dir_all(dest).await;
    }

    Err(format!("Git clone failed (tried branches: master, main): {}", last_error))
}

/// Analyze the project structure to determine build requirements.
async fn analyze_project(path: &Path) -> ProjectInfo {
    let mut info = ProjectInfo::default();

    // Check for client/ directory
    info.has_client = path.join("client").is_dir();

    // Check for public/ directory
    info.has_public = path.join("public").is_dir();

    // Check for database usage
    info.has_database = path.join("migrations").is_dir()
        || fs::read_to_string(path.join(".env"))
            .await
            .map(|s| s.contains("DATABASE_URL"))
            .unwrap_or(false)
        || fs::read_to_string(path.join("Cargo.toml"))
            .await
            .map(|s| s.contains("sqlite"))
            .unwrap_or(false);

    // Get package name from Cargo.toml
    if let Ok(cargo_toml) = fs::read_to_string(path.join("Cargo.toml")).await {
        for line in cargo_toml.lines() {
            if line.starts_with("name") {
                if let Some(name) = line.split('=').nth(1) {
                    info.package_name = Some(
                        name.trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string(),
                    );
                    break;
                }
            }
        }
    }

    // Detect port from main.rs
    // Look for App::new(PORT, ...) or App::with_state(PORT, ...)
    if let Ok(main_rs) = fs::read_to_string(path.join("src/main.rs")).await {
        if let Some(port) = detect_port_from_main(&main_rs) {
            info.port = port;
        }
    }

    info
}

/// Try to detect the port from main.rs content.
/// Looks for patterns like `App::new(8080,` or `App::with_state(3000,`
fn detect_port_from_main(content: &str) -> Option<u16> {
    // Look for App::new(PORT or App::with_state(PORT
    for pattern in ["App::new(", "App::with_state("] {
        if let Some(idx) = content.find(pattern) {
            let after_pattern = &content[idx + pattern.len()..];
            // Find the first comma or closing paren
            let end = after_pattern.find(|c| c == ',' || c == ')').unwrap_or(0);
            let port_str = after_pattern[..end].trim();
            
            // Try to parse as a number
            if let Ok(port) = port_str.parse::<u16>() {
                return Some(port);
            }
        }
    }
    None
}

/// Set Fly secrets (environment variables).
async fn set_fly_secrets(
    app_name: &str,
    token: &str,
    env_vars: &[(String, String)],
    cwd: &Path,
) -> Result<String, String> {
    if env_vars.is_empty() {
        return Ok(String::new());
    }

    // Format: KEY1=VALUE1 KEY2=VALUE2 ...
    let secrets: Vec<String> = env_vars
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();

    let mut args = vec!["secrets", "set", "-a", app_name];
    for secret in &secrets {
        args.push(secret);
    }

    let output = Command::new("flyctl")
        .args(&args)
        .env("FLY_API_TOKEN", token)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run flyctl: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        Ok(format!("{}{}", stdout, stderr))
    } else {
        Err(format!("flyctl secrets failed: {}", stderr))
    }
}

/// Create a Fly app using flyctl.
async fn create_fly_app(app_name: &str, token: &str, org: &str) -> Result<String, String> {
    let output = Command::new("flyctl")
        .args(["apps", "create", app_name, "-o", org, "-y"])
        .env("FLY_API_TOKEN", token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Failed to run flyctl: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    if output.status.success() {
        Ok(combined)
    } else {
        Err(combined)
    }
}

/// Run flyctl deploy with streaming output.
async fn run_flyctl_deploy_streaming(
    config: &DeployConfig,
    cwd: &Path,
    db: &Option<Pool<Sqlite>>,
    deployment_id: &Option<String>,
    existing_logs: &str,
) -> Result<String, String> {
    let args = vec![
        "deploy",
        "--remote-only", // Use Fly's remote builder
        "--wait-timeout",
        "300", // 5 minute timeout
    ];

    let mut child = Command::new("flyctl")
        .args(&args)
        .env("FLY_API_TOKEN", &config.fly_token)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to run flyctl: {}", e))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let mut output = String::new();
    let mut full_logs = existing_logs.to_string();

    // Read stdout and stderr concurrently, updating DB as we go
    if let (Some(stdout), Some(stderr)) = (stdout, stderr) {
        let mut stdout_reader = BufReader::new(stdout).lines();
        let mut stderr_reader = BufReader::new(stderr).lines();
        let mut stdout_done = false;
        let mut stderr_done = false;

        loop {
            // Exit when both streams are done
            if stdout_done && stderr_done {
                break;
            }

            tokio::select! {
                line = stdout_reader.next_line(), if !stdout_done => {
                    match line {
                        Ok(Some(line)) => {
                            output.push_str(&line);
                            output.push('\n');
                            full_logs.push_str(&line);
                            full_logs.push('\n');
                            
                            // Update database with new logs
                            if let (Some(db), Some(id)) = (db, deployment_id) {
                                let _ = query("UPDATE deployments SET build_logs = ? WHERE id = ?")
                                    .bind(&full_logs)
                                    .bind(id)
                                    .execute(db)
                                    .await;
                            }
                        }
                        Ok(None) => stdout_done = true,
                        Err(_) => stdout_done = true,
                    }
                }
                line = stderr_reader.next_line(), if !stderr_done => {
                    match line {
                        Ok(Some(line)) => {
                            output.push_str(&line);
                            output.push('\n');
                            full_logs.push_str(&line);
                            full_logs.push('\n');
                            
                            // Update database with new logs
                            if let (Some(db), Some(id)) = (db, deployment_id) {
                                let _ = query("UPDATE deployments SET build_logs = ? WHERE id = ?")
                                    .bind(&full_logs)
                                    .bind(id)
                                    .execute(db)
                                    .await;
                            }
                        }
                        Ok(None) => stderr_done = true,
                        Err(_) => stderr_done = true,
                    }
                }
            }
        }
    }

    let status = child.wait().await.map_err(|e| format!("Failed to wait for flyctl: {}", e))?;

    if status.success() {
        Ok(output)
    } else {
        Err(output)
    }
}

/// Get the latest commit info from a cloned repo.
pub async fn get_commit_info(repo_path: &Path) -> Option<(String, String)> {
    let output = Command::new("git")
        .args(["log", "-1", "--format=%H|%s"])
        .current_dir(repo_path)
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = output_str.trim().splitn(2, '|').collect();

    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_port_from_main() {
        // App::new with port
        assert_eq!(
            detect_port_from_main("let app = App::new(8080, create_router());"),
            Some(8080)
        );
        
        // App::with_state with port
        assert_eq!(
            detect_port_from_main("let app = App::with_state(3000, create_router(), state);"),
            Some(3000)
        );
        
        // Multiline
        assert_eq!(
            detect_port_from_main(r#"
                let app = App::new(
                    9000,
                    create_router()
                );
            "#),
            Some(9000)
        );
        
        // No port found (variable)
        assert_eq!(
            detect_port_from_main("let app = App::new(port, create_router());"),
            None
        );
    }

    #[tokio::test]
    async fn test_analyze_project_minimal() {
        let temp = std::env::temp_dir().join("test-rejoice-minimal");
        let _ = fs::create_dir_all(&temp).await;
        fs::write(
            temp.join("Cargo.toml"),
            r#"[package]
name = "test-app"
version = "0.1.0"
"#,
        )
        .await
        .unwrap();

        let info = analyze_project(&temp).await;

        assert!(!info.has_client);
        assert!(!info.has_database);
        assert_eq!(info.package_name, Some("test-app".to_string()));
        assert_eq!(info.port, 8080); // default

        let _ = fs::remove_dir_all(&temp).await;
    }

    #[tokio::test]
    async fn test_analyze_project_with_custom_port() {
        let temp = std::env::temp_dir().join("test-rejoice-custom-port");
        let _ = fs::create_dir_all(&temp).await;
        let _ = fs::create_dir_all(temp.join("src")).await;
        
        fs::write(
            temp.join("Cargo.toml"),
            r#"[package]
name = "custom-port-app"
version = "0.1.0"
"#,
        )
        .await
        .unwrap();
        
        fs::write(
            temp.join("src/main.rs"),
            r#"
use rejoice::App;

fn main() {
    let app = App::new(3000, create_router());
    app.run();
}
"#,
        )
        .await
        .unwrap();

        let info = analyze_project(&temp).await;

        assert_eq!(info.port, 3000);

        let _ = fs::remove_dir_all(&temp).await;
    }
}
