//! GitHub App API client.
//!
//! Handles authentication and API calls for the GitHub App integration:
//! - JWT generation for App authentication
//! - Installation access token fetching
//! - Commit status updates
//! - Repository operations

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// GitHub App client for API operations.
#[derive(Clone)]
pub struct GitHubApp {
    app_id: String,
    private_key: EncodingKey,
    client: reqwest::Client,
    /// Cache of installation tokens: installation_id -> (token, expires_at)
    token_cache: Arc<RwLock<std::collections::HashMap<i64, (String, u64)>>>,
}

/// JWT claims for GitHub App authentication.
#[derive(Serialize)]
struct AppJwtClaims {
    iat: u64,    // Issued at
    exp: u64,    // Expires at
    iss: String, // Issuer (App ID)
}

/// Response from GitHub when requesting an installation token.
#[derive(Deserialize)]
struct InstallationTokenResponse {
    token: String,
    expires_at: String,
}

/// Response from GitHub's installations endpoint.
#[derive(Deserialize)]
pub struct Installation {
    pub id: i64,
    pub account: InstallationAccount,
}

#[derive(Deserialize)]
pub struct InstallationAccount {
    pub login: String,
    pub id: i64,
}

/// A repository from GitHub's API.
#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub default_branch: String,
}

/// Commit status state.
#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum CommitStatusState {
    Pending,
    Success,
    Failure,
    Error,
}

/// Request body for creating a commit status.
#[derive(Serialize)]
struct CreateStatusRequest<'a> {
    state: CommitStatusState,
    target_url: Option<&'a str>,
    description: Option<&'a str>,
    context: &'a str,
}

impl GitHubApp {
    /// Create a new GitHub App client.
    pub fn new(app_id: String, private_key_pem: &str) -> Result<Self, String> {
        let private_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
            .map_err(|e| format!("Failed to parse private key: {}", e))?;

        Ok(Self {
            app_id,
            private_key,
            client: reqwest::Client::new(),
            token_cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// Generate a JWT for authenticating as the GitHub App.
    fn generate_app_jwt(&self) -> Result<String, String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();

        let claims = AppJwtClaims {
            // GitHub recommends 60 seconds in the past to allow for clock drift
            iat: now.saturating_sub(60),
            // JWT expires in 10 minutes (max allowed by GitHub)
            exp: now + 600,
            iss: self.app_id.clone(),
        };

        let header = Header::new(Algorithm::RS256);
        encode(&header, &claims, &self.private_key)
            .map_err(|e| format!("Failed to encode JWT: {}", e))
    }

    /// Get an installation access token for a specific installation.
    /// Tokens are cached until they expire.
    pub async fn get_installation_token(&self, installation_id: i64) -> Result<String, String> {
        // Check cache first
        {
            let cache = self.token_cache.read().await;
            if let Some((token, expires_at)) = cache.get(&installation_id) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                // Use token if it has more than 5 minutes left
                if *expires_at > now + 300 {
                    return Ok(token.clone());
                }
            }
        }

        // Fetch new token
        let jwt = self.generate_app_jwt()?;

        let response = self
            .client
            .post(format!(
                "https://api.github.com/app/installations/{}/access_tokens",
                installation_id
            ))
            .header("Authorization", format!("Bearer {}", jwt))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "Rejoice-Cloud")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|e| format!("Failed to request installation token: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("GitHub API error ({}): {}", status, body));
        }

        let token_response: InstallationTokenResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        // Parse expiration time and cache the token
        // expires_at is ISO 8601 format: "2024-01-01T00:00:00Z"
        let expires_at = chrono::DateTime::parse_from_rfc3339(&token_response.expires_at)
            .map(|dt| dt.timestamp() as u64)
            .unwrap_or(0);

        {
            let mut cache = self.token_cache.write().await;
            cache.insert(installation_id, (token_response.token.clone(), expires_at));
        }

        Ok(token_response.token)
    }

    /// List all installations of this GitHub App.
    pub async fn list_installations(&self) -> Result<Vec<Installation>, String> {
        let jwt = self.generate_app_jwt()?;

        let response = self
            .client
            .get("https://api.github.com/app/installations")
            .header("Authorization", format!("Bearer {}", jwt))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "Rejoice-Cloud")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .map_err(|e| format!("Failed to list installations: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("GitHub API error ({}): {}", status, body));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Failed to parse installations: {}", e))
    }

    /// Get installation for a specific user by their GitHub username.
    pub async fn get_installation_for_user(
        &self,
        username: &str,
    ) -> Result<Option<Installation>, String> {
        let installations = self.list_installations().await?;
        Ok(installations
            .into_iter()
            .find(|i| i.account.login == username))
    }

    /// List all repositories accessible to an installation.
    /// Handles pagination to fetch all repos.
    pub async fn list_installation_repos(
        &self,
        installation_id: i64,
    ) -> Result<Vec<Repository>, String> {
        let token = self.get_installation_token(installation_id).await?;
        let mut all_repos = Vec::new();
        let mut page = 1;
        let per_page = 100; // Max allowed by GitHub

        loop {
            let response = self
                .client
                .get("https://api.github.com/installation/repositories")
                .query(&[
                    ("per_page", per_page.to_string()),
                    ("page", page.to_string()),
                ])
                .header("Authorization", format!("Bearer {}", token))
                .header("Accept", "application/vnd.github+json")
                .header("User-Agent", "Rejoice-Cloud")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await
                .map_err(|e| format!("Failed to list repos: {}", e))?;

            if !response.status().is_success() {
                let status = response.status();
                let body = response.text().await.unwrap_or_default();
                return Err(format!("GitHub API error ({}): {}", status, body));
            }

            #[derive(Deserialize)]
            struct ReposResponse {
                repositories: Vec<Repository>,
                total_count: i64,
            }

            let repos: ReposResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse repos: {}", e))?;

            let fetched_count = repos.repositories.len();
            all_repos.extend(repos.repositories);

            // Check if we've fetched all repos
            if fetched_count < per_page || all_repos.len() as i64 >= repos.total_count {
                break;
            }

            page += 1;

            // Safety limit to prevent infinite loops
            if page > 50 {
                break;
            }
        }

        // Sort by name for consistent ordering
        all_repos.sort_by(|a, b| a.full_name.to_lowercase().cmp(&b.full_name.to_lowercase()));

        Ok(all_repos)
    }

    /// Create a commit status on a repository.
    pub async fn create_commit_status(
        &self,
        installation_id: i64,
        owner: &str,
        repo: &str,
        sha: &str,
        state: CommitStatusState,
        target_url: Option<&str>,
        description: Option<&str>,
        context: &str,
    ) -> Result<(), String> {
        let token = self.get_installation_token(installation_id).await?;

        let body = CreateStatusRequest {
            state,
            target_url,
            description,
            context,
        };

        let response = self
            .client
            .post(format!(
                "https://api.github.com/repos/{}/{}/statuses/{}",
                owner, repo, sha
            ))
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "Rejoice-Cloud")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Failed to create status: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("GitHub API error ({}): {}", status, body));
        }

        Ok(())
    }

    /// Generate a git clone URL with embedded token for authenticated access.
    /// Returns a URL like: https://x-access-token:TOKEN@github.com/owner/repo.git
    pub async fn get_authenticated_clone_url(
        &self,
        installation_id: i64,
        owner: &str,
        repo: &str,
    ) -> Result<String, String> {
        let token = self.get_installation_token(installation_id).await?;
        Ok(format!(
            "https://x-access-token:{}@github.com/{}/{}.git",
            token, owner, repo
        ))
    }

    /// Get the public installation URL for users to install the app.
    pub fn get_installation_url(&self) -> String {
        format!("https://github.com/apps/rejoice-cloud-dev/installations/new")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_status_serialization() {
        let json = serde_json::to_string(&CommitStatusState::Pending).unwrap();
        assert_eq!(json, "\"pending\"");

        let json = serde_json::to_string(&CommitStatusState::Success).unwrap();
        assert_eq!(json, "\"success\"");
    }
}
