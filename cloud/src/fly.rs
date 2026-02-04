//! Fly.io Machines API client.
//!
//! This module provides a client for interacting with the Fly.io Machines API
//! to create apps, deploy machines, manage volumes, and more.
//!
//! API Documentation: https://fly.io/docs/machines/api/

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const FLY_API_HOSTNAME: &str = "https://api.machines.dev";

/// Fly.io API client
#[derive(Clone)]
pub struct FlyClient {
    client: Client,
    token: String,
    org_slug: String,
}

impl FlyClient {
    /// Create a new Fly.io API client.
    pub fn new(token: String, org_slug: String) -> Self {
        Self {
            client: Client::new(),
            token,
            org_slug,
        }
    }

    /// Make an authenticated request to the Fly.io API.
    async fn request<T: for<'de> Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<impl Serialize>,
    ) -> Result<T, FlyError> {
        let url = format!("{}{}", FLY_API_HOSTNAME, path);

        let mut req = self
            .client
            .request(method, &url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            req = req.json(&body);
        }

        let response = req.send().await.map_err(FlyError::Request)?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(FlyError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        // Handle empty responses (204, 202, etc.)
        let text = response.text().await.map_err(FlyError::Request)?;
        if text.is_empty() {
            // Return default/empty value - this works for () and Option<T>
            return serde_json::from_str("null").map_err(FlyError::Json);
        }

        serde_json::from_str(&text).map_err(FlyError::Json)
    }

    // =========================================================================
    // Apps API
    // =========================================================================

    /// Create a new Fly App.
    pub async fn create_app(&self, app_name: &str) -> Result<CreateAppResponse, FlyError> {
        self.request(
            reqwest::Method::POST,
            "/v1/apps",
            Some(CreateAppRequest {
                app_name: app_name.to_string(),
                org_slug: self.org_slug.clone(),
            }),
        )
        .await
    }

    /// Get details about an app.
    pub async fn get_app(&self, app_name: &str) -> Result<AppDetails, FlyError> {
        self.request::<AppDetails>(
            reqwest::Method::GET,
            &format!("/v1/apps/{}", app_name),
            None::<()>,
        )
        .await
    }

    /// Delete a Fly App.
    pub async fn delete_app(&self, app_name: &str, force: bool) -> Result<(), FlyError> {
        let path = if force {
            format!("/v1/apps/{}?force=true", app_name)
        } else {
            format!("/v1/apps/{}", app_name)
        };

        self.request::<Option<()>>(reqwest::Method::DELETE, &path, None::<()>)
            .await?;
        Ok(())
    }

    // =========================================================================
    // Machines API
    // =========================================================================

    /// Create a new Machine in an app.
    pub async fn create_machine(
        &self,
        app_name: &str,
        config: MachineConfig,
    ) -> Result<Machine, FlyError> {
        self.request(
            reqwest::Method::POST,
            &format!("/v1/apps/{}/machines", app_name),
            Some(CreateMachineRequest {
                config,
                region: Some("iad".to_string()), // Default to US East
                name: None,
            }),
        )
        .await
    }

    /// List all Machines in an app.
    pub async fn list_machines(&self, app_name: &str) -> Result<Vec<Machine>, FlyError> {
        self.request::<Vec<Machine>>(
            reqwest::Method::GET,
            &format!("/v1/apps/{}/machines", app_name),
            None::<()>,
        )
        .await
    }

    /// Get a specific Machine.
    pub async fn get_machine(&self, app_name: &str, machine_id: &str) -> Result<Machine, FlyError> {
        self.request::<Machine>(
            reqwest::Method::GET,
            &format!("/v1/apps/{}/machines/{}", app_name, machine_id),
            None::<()>,
        )
        .await
    }

    /// Update a Machine's configuration.
    pub async fn update_machine(
        &self,
        app_name: &str,
        machine_id: &str,
        config: MachineConfig,
    ) -> Result<Machine, FlyError> {
        self.request(
            reqwest::Method::POST,
            &format!("/v1/apps/{}/machines/{}", app_name, machine_id),
            Some(UpdateMachineRequest { config }),
        )
        .await
    }

    /// Start a stopped Machine.
    pub async fn start_machine(&self, app_name: &str, machine_id: &str) -> Result<(), FlyError> {
        self.request::<Option<()>>(
            reqwest::Method::POST,
            &format!("/v1/apps/{}/machines/{}/start", app_name, machine_id),
            None::<()>,
        )
        .await?;
        Ok(())
    }

    /// Stop a running Machine.
    pub async fn stop_machine(&self, app_name: &str, machine_id: &str) -> Result<(), FlyError> {
        self.request::<Option<()>>(
            reqwest::Method::POST,
            &format!("/v1/apps/{}/machines/{}/stop", app_name, machine_id),
            None::<()>,
        )
        .await?;
        Ok(())
    }

    /// Delete a Machine permanently.
    pub async fn delete_machine(&self, app_name: &str, machine_id: &str) -> Result<(), FlyError> {
        self.request::<Option<()>>(
            reqwest::Method::DELETE,
            &format!("/v1/apps/{}/machines/{}", app_name, machine_id),
            None::<()>,
        )
        .await?;
        Ok(())
    }

    /// Wait for a Machine to reach a specified state.
    pub async fn wait_for_machine(
        &self,
        app_name: &str,
        machine_id: &str,
        state: MachineState,
        timeout_secs: u32,
    ) -> Result<(), FlyError> {
        let state_str = match state {
            MachineState::Started => "started",
            MachineState::Stopped => "stopped",
            MachineState::Destroyed => "destroyed",
        };

        self.request::<Option<()>>(
            reqwest::Method::GET,
            &format!(
                "/v1/apps/{}/machines/{}/wait?state={}&timeout={}",
                app_name, machine_id, state_str, timeout_secs
            ),
            None::<()>,
        )
        .await?;
        Ok(())
    }

    // =========================================================================
    // Volumes API
    // =========================================================================

    /// Create a new volume for persistent storage.
    pub async fn create_volume(
        &self,
        app_name: &str,
        name: &str,
        size_gb: u32,
        region: &str,
    ) -> Result<Volume, FlyError> {
        self.request(
            reqwest::Method::POST,
            &format!("/v1/apps/{}/volumes", app_name),
            Some(CreateVolumeRequest {
                name: name.to_string(),
                size_gb,
                region: region.to_string(),
            }),
        )
        .await
    }

    /// List all volumes for an app.
    pub async fn list_volumes(&self, app_name: &str) -> Result<Vec<Volume>, FlyError> {
        self.request::<Vec<Volume>>(
            reqwest::Method::GET,
            &format!("/v1/apps/{}/volumes", app_name),
            None::<()>,
        )
        .await
    }

    /// Delete a volume.
    pub async fn delete_volume(&self, app_name: &str, volume_id: &str) -> Result<(), FlyError> {
        self.request::<Option<()>>(
            reqwest::Method::DELETE,
            &format!("/v1/apps/{}/volumes/{}", app_name, volume_id),
            None::<()>,
        )
        .await?;
        Ok(())
    }
}

// =============================================================================
// Request/Response Types
// =============================================================================

#[derive(Debug, Serialize)]
struct CreateAppRequest {
    app_name: String,
    org_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAppResponse {
    pub id: String,
    pub created_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct AppDetails {
    pub id: String,
    pub name: String,
    pub status: String,
    pub organization: Option<AppOrganization>,
}

#[derive(Debug, Deserialize)]
pub struct AppOrganization {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
struct CreateMachineRequest {
    config: MachineConfig,
    region: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Serialize)]
struct UpdateMachineRequest {
    config: MachineConfig,
}

/// Configuration for a Fly Machine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineConfig {
    /// Docker image to run
    pub image: String,

    /// Environment variables
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    /// Services (ports) to expose
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<MachineService>>,

    /// Guest VM configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest: Option<MachineGuest>,

    /// Mounts for persistent volumes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<MachineMount>>,

    /// Auto-stop configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_destroy: Option<bool>,

    /// Restart policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<MachineRestart>,

    /// Health checks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<HashMap<String, MachineCheck>>,
}

impl MachineConfig {
    /// Create a basic config for a Rejoice app.
    pub fn for_rejoice_app(image: &str, env: HashMap<String, String>, volume_id: Option<&str>) -> Self {
        let mut config = Self {
            image: image.to_string(),
            env: Some(env),
            services: Some(vec![MachineService {
                protocol: "tcp".to_string(),
                internal_port: 3000, // Default Rejoice port
                ports: vec![
                    MachinePort {
                        port: 80,
                        handlers: vec!["http".to_string()],
                        force_https: Some(true),
                        tls_options: None,
                    },
                    MachinePort {
                        port: 443,
                        handlers: vec!["http".to_string(), "tls".to_string()],
                        force_https: None,
                        tls_options: None,
                    },
                ],
                concurrency: Some(MachineServiceConcurrency {
                    kind: "connections".to_string(),
                    hard_limit: 250,
                    soft_limit: 200,
                }),
                autostop: Some("stop".to_string()),
                autostart: Some(true),
            }]),
            guest: Some(MachineGuest {
                cpu_kind: "shared".to_string(),
                cpus: 1,
                memory_mb: 256,
            }),
            mounts: None,
            auto_destroy: None,
            restart: Some(MachineRestart {
                policy: "always".to_string(),
                max_retries: Some(3),
            }),
            checks: Some({
                let mut checks = HashMap::new();
                checks.insert(
                    "health".to_string(),
                    MachineCheck {
                        port: Some(3000),
                        kind: "http".to_string(),
                        path: Some("/_health".to_string()),
                        interval: Some("15s".to_string()),
                        timeout: Some("5s".to_string()),
                        grace_period: Some("30s".to_string()),
                    },
                );
                checks
            }),
        };

        // Add volume mount if provided
        if let Some(vol_id) = volume_id {
            config.mounts = Some(vec![MachineMount {
                volume: vol_id.to_string(),
                path: "/data".to_string(),
            }]);
        }

        config
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineService {
    pub protocol: String,
    pub internal_port: u16,
    pub ports: Vec<MachinePort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<MachineServiceConcurrency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachinePort {
    pub port: u16,
    pub handlers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_https: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_options: Option<MachineTlsOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineTlsOptions {
    pub alpn: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineServiceConcurrency {
    #[serde(rename = "type")]
    pub kind: String,
    pub hard_limit: u32,
    pub soft_limit: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineGuest {
    pub cpu_kind: String,
    pub cpus: u32,
    pub memory_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineMount {
    pub volume: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineRestart {
    pub policy: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineCheck {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grace_period: Option<String>,
}

/// A Fly Machine instance.
#[derive(Debug, Deserialize)]
pub struct Machine {
    pub id: String,
    pub name: String,
    pub state: String,
    pub region: String,
    pub instance_id: Option<String>,
    pub private_ip: Option<String>,
    pub config: Option<MachineConfig>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

/// Machine states for wait operations.
pub enum MachineState {
    Started,
    Stopped,
    Destroyed,
}

// =============================================================================
// Volume Types
// =============================================================================

#[derive(Debug, Serialize)]
struct CreateVolumeRequest {
    name: String,
    size_gb: u32,
    region: String,
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    pub id: String,
    pub name: String,
    pub state: String,
    pub size_gb: u32,
    pub region: String,
    pub zone: Option<String>,
    pub attached_machine_id: Option<String>,
    pub created_at: Option<String>,
}

// =============================================================================
// Errors
// =============================================================================

#[derive(Debug)]
pub enum FlyError {
    Request(reqwest::Error),
    Api { status: u16, message: String },
    Json(serde_json::Error),
}

impl std::fmt::Display for FlyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlyError::Request(e) => write!(f, "Request error: {}", e),
            FlyError::Api { status, message } => {
                write!(f, "Fly API error ({}): {}", status, message)
            }
            FlyError::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for FlyError {}
