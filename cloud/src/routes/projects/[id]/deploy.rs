use crate::components as ui;
use crate::deployer::{self, DeployConfig};
use crate::AppState;
use rejoice::db::{query, query_as, FromRow, Pool, Sqlite};
use rejoice::{html, Req, Res};
use uuid::Uuid;

#[derive(FromRow)]
struct Project {
    name: String,
    github_repo: String,
    fly_app_name: Option<String>,
}

/// GET /projects/:id/deploy - Show deploy confirmation page
pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user
    let project = query_as::<_, Project>(
        r#"
        SELECT p.id, p.name, p.github_repo, p.fly_app_name
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

    let is_first_deploy = project.fly_app_name.is_none();

    res.html(html! {
        div class="max-w-xl mx-auto px-6 py-10" {
            // Back link
            a href=(format!("/projects/{}", id)) class="text-sm text-stone-500 hover:text-stone-300 no-underline cursor-pointer" {
                "← " (&project.name)
            }

            // Header
            div class="mt-6 mb-8" {
                h1 class="text-xl font-medium text-stone-100" { "Deploy" }
                p class="text-sm text-stone-500 mt-2" {
                    @if is_first_deploy {
                        "This will create a new Fly.io app and deploy your project for the first time."
                    } @else {
                        "This will deploy the latest version from the main branch."
                    }
                }
            }

            // Deployment info
            (ui::card(html! {
                h2 class="text-sm font-medium text-stone-300 mb-4" { "Deployment Details" }

                dl class="space-y-3 text-sm" {
                    div class="flex justify-between" {
                        dt class="text-stone-500" { "Repository" }
                        dd class="text-stone-300" { (&project.github_repo) }
                    }
                    div class="flex justify-between" {
                        dt class="text-stone-500" { "Branch" }
                        dd class="text-stone-300" { "main" }
                    }
                    @if is_first_deploy {
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Fly App" }
                            dd class="text-stone-300 font-mono text-xs" {
                                (format!("rejoice-{}", &id[..8]))
                            }
                        }
                    } @else {
                        div class="flex justify-between" {
                            dt class="text-stone-500" { "Fly App" }
                            dd class="text-stone-300 font-mono text-xs" {
                                (project.fly_app_name.as_deref().unwrap_or("unknown"))
                            }
                        }
                    }
                }
            }))

            // Deploy button
            form method="POST" action=(format!("/projects/{}/deploy", id)) class="mt-6" {
                div class="flex gap-3" {
                    a
                        href=(format!("/projects/{}", id))
                        class="flex-1 inline-flex items-center justify-center font-medium rounded-lg \
                               transition-colors cursor-pointer h-10 px-4 text-sm \
                               bg-stone-800 text-stone-200 ring-1 ring-inset ring-stone-700 hover:bg-stone-700 \
                               no-underline"
                    {
                        "Cancel"
                    }
                    button
                        type="submit"
                        class="flex-1 inline-flex items-center justify-center font-medium rounded-lg \
                               transition-colors cursor-pointer h-10 px-4 text-sm \
                               bg-amber-600 text-white hover:bg-amber-500"
                    {
                        @if is_first_deploy {
                            "Create & Deploy"
                        } @else {
                            "Deploy Now"
                        }
                    }
                }
            }

            // Warning for first deploy
            @if is_first_deploy {
                div class="mt-6 p-4 bg-amber-950/30 border border-amber-900/50 rounded-lg" {
                    p class="text-sm text-amber-200" {
                        "Note: First deployment may take several minutes as we build your application."
                    }
                }
            }
        }
    })
}

/// POST /projects/:id/deploy - Trigger deployment
pub async fn post(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user and get env vars
    let project = query_as::<_, Project>(
        r#"
        SELECT p.id, p.name, p.github_repo, p.fly_app_name
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
        Ok(None) => return res.forbidden("Not authorized"),
        Err(_) => return res.internal_error("Database error"),
    };

    // Get environment variables for this project
    #[derive(FromRow)]
    struct EnvVar {
        key: String,
        encrypted_value: Vec<u8>,
    }

    let env_vars_result = query_as::<_, EnvVar>(
        "SELECT key, encrypted_value FROM env_vars WHERE project_id = ?",
    )
    .bind(&id)
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
        .unwrap_or_else(|| format!("rejoice-{}", &id[..8]));

    // Create deployment record
    let deployment_id = Uuid::new_v4().to_string();

    let insert_result = query(
        r#"
        INSERT INTO deployments (id, project_id, git_sha, git_branch, git_message, status)
        VALUES (?, ?, 'pending', 'main', 'Deployment in progress...', 'building')
        "#,
    )
    .bind(&deployment_id)
    .bind(&id)
    .execute(&state.db)
    .await;

    if insert_result.is_err() {
        return res.internal_error("Failed to create deployment record");
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
        github_repo: project.github_repo.clone(),
        branch: "main".to_string(),
        fly_app_name: fly_app_name.clone(),
        fly_token: state.fly_token.clone(),
        fly_org: state.fly_org.clone(),
        env_vars,
        is_first_deploy,
    };

    let db = state.db.clone();
    let dep_id = deployment_id.clone();

    tokio::spawn(async move {
        run_deployment(deploy_config, db, dep_id).await;
    });

    // Redirect to deployment detail page immediately
    res.redirect(&format!(
        "/projects/{}/deployments/{}",
        id, deployment_id
    ))
}

/// Run the deployment in the background and update the database with results.
async fn run_deployment(config: DeployConfig, db: Pool<Sqlite>, deployment_id: String) {
    // Run the deployment with streaming logs
    let result = deployer::deploy(config, Some(db.clone()), Some(deployment_id.clone())).await;

    // Update deployment record with results
    if result.success {
        let _ = query(
            r#"
            UPDATE deployments 
            SET status = 'success', 
                url = ?,
                build_logs = ?,
                git_sha = COALESCE(NULLIF(git_sha, 'pending'), ?),
                finished_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
        )
        .bind(&result.url)
        .bind(&result.logs)
        .bind(&result.logs.lines().take(1).collect::<String>()) // Use first line as placeholder SHA
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
