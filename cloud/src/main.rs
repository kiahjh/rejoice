use std::time::Duration;

use rejoice::{
    App,
    db::{Pool, PoolConfig, Sqlite, create_pool},
};

pub mod builder;
pub mod components;
pub mod crypto;
pub mod deployer;
pub mod fly;
pub mod github;

rejoice::routes!(AppState);

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub encryption_key: [u8; 32],
    pub fly: fly::FlyClient,
    pub fly_token: String,
    pub fly_org: String,
    pub github_app: github::GitHubApp,
}

#[tokio::main]
async fn main() {
    let pool = create_pool(PoolConfig {
        db_url: rejoice::env!("DATABASE_URL").to_string(),
        max_connections: 5,
        acquire_timeout: Duration::from_secs(3),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
    })
    .await;

    // Parse encryption key from environment
    let encryption_key = crypto::parse_key(rejoice::env!("ENCRYPTION_KEY"))
        .expect("ENCRYPTION_KEY must be a valid 32-byte base64-encoded key");

    // Create Fly.io API client
    let fly = fly::FlyClient::new(
        rejoice::env!("FLY_API_TOKEN").to_string(),
        rejoice::env!("FLY_ORG_SLUG").to_string(),
    );

    let fly_token = rejoice::env!("FLY_API_TOKEN").to_string();
    let fly_org = rejoice::env!("FLY_ORG_SLUG").to_string();

    // Create GitHub App client
    let github_app_private_key = std::fs::read_to_string(rejoice::env!("GITHUB_APP_PRIVATE_KEY_PATH"))
        .expect("Failed to read GitHub App private key file");
    let github_app = github::GitHubApp::new(
        rejoice::env!("GITHUB_APP_ID").to_string(),
        &github_app_private_key,
    )
    .expect("Failed to create GitHub App client");

    let state = AppState {
        db: pool,
        encryption_key,
        fly,
        fly_token,
        fly_org,
        github_app,
    };

    let app = App::with_state(3333, create_router(), state);
    app.run().await;
}
