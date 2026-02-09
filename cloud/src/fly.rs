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
const FLY_GRAPHQL_URL: &str = "https://api.fly.io/graphql";

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

    // =========================================================================
    // IP Addresses API (GraphQL)
    // =========================================================================

    /// List allocated IP addresses for a Fly app.
    pub async fn list_ip_addresses(
        &self,
        app_name: &str,
    ) -> Result<Vec<IpAddress>, FlyError> {
        let query = r#"
            query($appName: String!) {
                app(name: $appName) {
                    ipAddresses {
                        nodes {
                            address
                            type
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "appName": app_name,
        });

        #[derive(Deserialize)]
        struct Response {
            app: AppIpResponse,
        }

        #[derive(Deserialize)]
        struct AppIpResponse {
            #[serde(rename = "ipAddresses")]
            ip_addresses: IpNodes,
        }

        #[derive(Deserialize)]
        struct IpNodes {
            nodes: Vec<IpAddress>,
        }

        let response: Response = self.graphql(query, variables).await?;
        Ok(response.app.ip_addresses.nodes)
    }

    // =========================================================================
    // Certificates API (GraphQL)
    // =========================================================================

    /// Execute a GraphQL query against the Fly.io API.
    async fn graphql<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<T, FlyError> {
        let body = serde_json::json!({
            "query": query,
            "variables": variables,
        });

        let response = self
            .client
            .post(FLY_GRAPHQL_URL)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(FlyError::Request)?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(FlyError::Api {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let response_body: serde_json::Value =
            response.json().await.map_err(FlyError::Request)?;

        // Check for GraphQL errors
        if let Some(errors) = response_body.get("errors") {
            let message = errors
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown GraphQL error");
            return Err(FlyError::Api {
                status: 200,
                message: message.to_string(),
            });
        }

        let data = response_body
            .get("data")
            .ok_or_else(|| FlyError::Api {
                status: 200,
                message: "No data in GraphQL response".to_string(),
            })?
            .clone();

        serde_json::from_value(data).map_err(FlyError::Json)
    }

    /// Add a certificate (custom domain) to a Fly app.
    ///
    /// This initiates the certificate provisioning process. The returned
    /// `CertificateInfo` contains DNS validation instructions that the user
    /// must configure with their DNS provider.
    pub async fn add_certificate(
        &self,
        app_name: &str,
        hostname: &str,
    ) -> Result<CertificateInfo, FlyError> {
        let query = r#"
            mutation($appId: ID!, $hostname: String!) {
                addCertificate(appId: $appId, hostname: $hostname) {
                    certificate {
                        configured
                        acmeDnsConfigured
                        acmeAlpnConfigured
                        isAcmeHttpConfigured
                        certificateAuthority
                        dnsProvider
                        dnsValidationInstructions
                        dnsValidationHostname
                        dnsValidationTarget
                        hostname
                        id
                        source
                        clientStatus
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "appId": app_name,
            "hostname": hostname,
        });

        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "addCertificate")]
            add_certificate: AddCertificateResponse,
        }

        #[derive(Deserialize)]
        struct AddCertificateResponse {
            certificate: CertificateInfo,
        }

        let response: Response = self.graphql(query, variables).await?;
        Ok(response.add_certificate.certificate)
    }

    /// Get certificate details for a hostname on a Fly app.
    pub async fn get_certificate(
        &self,
        app_name: &str,
        hostname: &str,
    ) -> Result<CertificateDetail, FlyError> {
        let query = r#"
            query($appName: String!, $hostname: String!) {
                app(name: $appName) {
                    certificate(hostname: $hostname) {
                        configured
                        acmeDnsConfigured
                        acmeAlpnConfigured
                        isAcmeHttpConfigured
                        certificateAuthority
                        createdAt
                        dnsProvider
                        dnsValidationInstructions
                        dnsValidationHostname
                        dnsValidationTarget
                        hostname
                        id
                        source
                        clientStatus
                        issued {
                            nodes {
                                type
                                expiresAt
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "appName": app_name,
            "hostname": hostname,
        });

        #[derive(Deserialize)]
        struct Response {
            app: AppCertResponse,
        }

        #[derive(Deserialize)]
        struct AppCertResponse {
            certificate: CertificateDetail,
        }

        let response: Response = self.graphql(query, variables).await?;
        Ok(response.app.certificate)
    }

    /// Check a certificate's status (triggers re-validation).
    ///
    /// Similar to `get_certificate` but also requests the `check` field,
    /// which triggers Fly to re-validate the certificate.
    pub async fn check_certificate(
        &self,
        app_name: &str,
        hostname: &str,
    ) -> Result<CertificateDetail, FlyError> {
        let query = r#"
            query($appName: String!, $hostname: String!) {
                app(name: $appName) {
                    certificate(hostname: $hostname) {
                        check
                        configured
                        acmeDnsConfigured
                        acmeAlpnConfigured
                        isAcmeHttpConfigured
                        certificateAuthority
                        createdAt
                        dnsProvider
                        dnsValidationInstructions
                        dnsValidationHostname
                        dnsValidationTarget
                        hostname
                        id
                        source
                        clientStatus
                        issued {
                            nodes {
                                type
                                expiresAt
                            }
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "appName": app_name,
            "hostname": hostname,
        });

        #[derive(Deserialize)]
        struct Response {
            app: AppCertResponse,
        }

        #[derive(Deserialize)]
        struct AppCertResponse {
            certificate: CertificateDetail,
        }

        let response: Response = self.graphql(query, variables).await?;
        Ok(response.app.certificate)
    }

    /// List all certificates for a Fly app.
    pub async fn list_certificates(
        &self,
        app_name: &str,
    ) -> Result<Vec<CertificateSummary>, FlyError> {
        let query = r#"
            query($appName: String!) {
                app(name: $appName) {
                    certificates {
                        nodes {
                            createdAt
                            hostname
                            clientStatus
                            id
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "appName": app_name,
        });

        #[derive(Deserialize)]
        struct Response {
            app: AppCertsResponse,
        }

        #[derive(Deserialize)]
        struct AppCertsResponse {
            certificates: CertsNodes,
        }

        #[derive(Deserialize)]
        struct CertsNodes {
            nodes: Vec<CertificateSummary>,
        }

        let response: Response = self.graphql(query, variables).await?;
        Ok(response.app.certificates.nodes)
    }

    /// Delete a certificate (custom domain) from a Fly app.
    pub async fn delete_certificate(
        &self,
        app_name: &str,
        hostname: &str,
    ) -> Result<(), FlyError> {
        let query = r#"
            mutation($appId: ID!, $hostname: String!) {
                deleteCertificate(appId: $appId, hostname: $hostname) {
                    app {
                        name
                    }
                    certificate {
                        hostname
                        id
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "appId": app_name,
            "hostname": hostname,
        });

        // We don't need the response data, just need it to succeed
        let _: serde_json::Value = self.graphql(query, variables).await?;
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
// IP Address Types (GraphQL API)
// =============================================================================

/// An allocated IP address for a Fly app.
#[derive(Debug, Clone, Deserialize)]
pub struct IpAddress {
    pub address: String,
    #[serde(rename = "type")]
    pub ip_type: String, // "v4" or "v6"
}

// =============================================================================
// Certificate Types (GraphQL API)
// =============================================================================

/// Certificate info returned when adding a new certificate.
#[derive(Debug, Clone, Deserialize)]
pub struct CertificateInfo {
    pub configured: bool,
    #[serde(rename = "acmeDnsConfigured")]
    pub acme_dns_configured: bool,
    #[serde(rename = "acmeAlpnConfigured")]
    pub acme_alpn_configured: bool,
    #[serde(rename = "isAcmeHttpConfigured")]
    pub is_acme_http_configured: bool,
    #[serde(rename = "certificateAuthority")]
    pub certificate_authority: Option<String>,
    #[serde(rename = "dnsProvider")]
    pub dns_provider: Option<String>,
    #[serde(rename = "dnsValidationInstructions")]
    pub dns_validation_instructions: Option<String>,
    #[serde(rename = "dnsValidationHostname")]
    pub dns_validation_hostname: Option<String>,
    #[serde(rename = "dnsValidationTarget")]
    pub dns_validation_target: Option<String>,
    pub hostname: String,
    pub id: String,
    pub source: Option<String>,
    #[serde(rename = "clientStatus")]
    pub client_status: Option<String>,
}

/// Detailed certificate info including issued certificates.
#[derive(Debug, Clone, Deserialize)]
pub struct CertificateDetail {
    pub configured: bool,
    #[serde(rename = "acmeDnsConfigured")]
    pub acme_dns_configured: bool,
    #[serde(rename = "acmeAlpnConfigured")]
    pub acme_alpn_configured: bool,
    #[serde(rename = "isAcmeHttpConfigured")]
    pub is_acme_http_configured: bool,
    #[serde(rename = "certificateAuthority")]
    pub certificate_authority: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "dnsProvider")]
    pub dns_provider: Option<String>,
    #[serde(rename = "dnsValidationInstructions")]
    pub dns_validation_instructions: Option<String>,
    #[serde(rename = "dnsValidationHostname")]
    pub dns_validation_hostname: Option<String>,
    #[serde(rename = "dnsValidationTarget")]
    pub dns_validation_target: Option<String>,
    pub hostname: String,
    pub id: String,
    pub source: Option<String>,
    #[serde(rename = "clientStatus")]
    pub client_status: Option<String>,
    pub issued: Option<IssuedCertificates>,
}

/// Container for issued certificate nodes.
#[derive(Debug, Clone, Deserialize)]
pub struct IssuedCertificates {
    pub nodes: Vec<IssuedCertificate>,
}

/// An individual issued certificate (RSA or ECDSA).
#[derive(Debug, Clone, Deserialize)]
pub struct IssuedCertificate {
    #[serde(rename = "type")]
    pub cert_type: String,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<String>,
}

/// Summary certificate info from listing.
#[derive(Debug, Clone, Deserialize)]
pub struct CertificateSummary {
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub hostname: String,
    #[serde(rename = "clientStatus")]
    pub client_status: Option<String>,
    pub id: String,
}

impl CertificateDetail {
    /// Whether the certificate has been fully issued and is ready.
    pub fn is_ready(&self) -> bool {
        self.client_status.as_deref() == Some("Ready")
    }

    /// Get a user-friendly status string.
    pub fn status_display(&self) -> &str {
        match self.client_status.as_deref() {
            Some("Ready") => "ready",
            Some("Awaiting configuration") => "pending",
            Some(s) if s.contains("error") || s.contains("Error") => "error",
            _ => "validating",
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_info_deserialization() {
        let json = r#"{
            "configured": true,
            "acmeDnsConfigured": false,
            "acmeAlpnConfigured": true,
            "isAcmeHttpConfigured": false,
            "certificateAuthority": "lets_encrypt",
            "dnsProvider": "cloudflare",
            "dnsValidationInstructions": "CNAME _acme-challenge.example.com => example.com.xxx.flydns.net.",
            "dnsValidationHostname": "_acme-challenge.example.com",
            "dnsValidationTarget": "example.com.xxx.flydns.net",
            "hostname": "example.com",
            "id": "cert_abc123",
            "source": "fly",
            "clientStatus": "Ready"
        }"#;

        let cert: CertificateInfo = serde_json::from_str(json).unwrap();
        assert_eq!(cert.hostname, "example.com");
        assert_eq!(cert.id, "cert_abc123");
        assert!(cert.configured);
        assert!(!cert.acme_dns_configured);
        assert_eq!(cert.client_status.as_deref(), Some("Ready"));
        assert_eq!(
            cert.dns_validation_hostname.as_deref(),
            Some("_acme-challenge.example.com")
        );
        assert_eq!(
            cert.dns_validation_target.as_deref(),
            Some("example.com.xxx.flydns.net")
        );
    }

    #[test]
    fn test_certificate_detail_deserialization_with_issued() {
        let json = r#"{
            "configured": true,
            "acmeDnsConfigured": true,
            "acmeAlpnConfigured": true,
            "isAcmeHttpConfigured": true,
            "certificateAuthority": "lets_encrypt",
            "createdAt": "2026-02-08T12:00:00Z",
            "dnsProvider": "cloudflare",
            "dnsValidationInstructions": null,
            "dnsValidationHostname": "_acme-challenge.example.com",
            "dnsValidationTarget": "example.com.xxx.flydns.net",
            "hostname": "example.com",
            "id": "cert_abc123",
            "source": "fly",
            "clientStatus": "Ready",
            "issued": {
                "nodes": [
                    {"type": "ecdsa", "expiresAt": "2026-05-08T12:00:00Z"},
                    {"type": "rsa", "expiresAt": "2026-05-08T12:00:00Z"}
                ]
            }
        }"#;

        let cert: CertificateDetail = serde_json::from_str(json).unwrap();
        assert_eq!(cert.hostname, "example.com");
        assert!(cert.is_ready());
        assert_eq!(cert.status_display(), "ready");

        let issued = cert.issued.unwrap();
        assert_eq!(issued.nodes.len(), 2);
        assert_eq!(issued.nodes[0].cert_type, "ecdsa");
        assert_eq!(issued.nodes[1].cert_type, "rsa");
    }

    #[test]
    fn test_certificate_detail_status_display() {
        let make_cert = |status: Option<&str>| CertificateDetail {
            configured: false,
            acme_dns_configured: false,
            acme_alpn_configured: false,
            is_acme_http_configured: false,
            certificate_authority: None,
            created_at: None,
            dns_provider: None,
            dns_validation_instructions: None,
            dns_validation_hostname: None,
            dns_validation_target: None,
            hostname: "example.com".to_string(),
            id: "test".to_string(),
            source: None,
            client_status: status.map(String::from),
            issued: None,
        };

        assert_eq!(make_cert(Some("Ready")).status_display(), "ready");
        assert!(make_cert(Some("Ready")).is_ready());

        assert_eq!(
            make_cert(Some("Awaiting configuration")).status_display(),
            "pending"
        );
        assert!(!make_cert(Some("Awaiting configuration")).is_ready());

        assert_eq!(
            make_cert(Some("Validation error")).status_display(),
            "error"
        );

        assert_eq!(
            make_cert(Some("Verifying DNS")).status_display(),
            "validating"
        );

        assert_eq!(make_cert(None).status_display(), "validating");
        assert!(!make_cert(None).is_ready());
    }

    #[test]
    fn test_certificate_summary_deserialization() {
        let json = r#"[
            {"createdAt": "2026-02-08T12:00:00Z", "hostname": "example.com", "clientStatus": "Ready", "id": "cert1"},
            {"createdAt": "2026-02-08T13:00:00Z", "hostname": "www.example.com", "clientStatus": "Awaiting configuration", "id": "cert2"}
        ]"#;

        let certs: Vec<CertificateSummary> = serde_json::from_str(json).unwrap();
        assert_eq!(certs.len(), 2);
        assert_eq!(certs[0].hostname, "example.com");
        assert_eq!(certs[0].client_status.as_deref(), Some("Ready"));
        assert_eq!(certs[1].hostname, "www.example.com");
    }

    #[test]
    fn test_certificate_detail_without_issued() {
        let json = r#"{
            "configured": false,
            "acmeDnsConfigured": false,
            "acmeAlpnConfigured": false,
            "isAcmeHttpConfigured": false,
            "certificateAuthority": null,
            "createdAt": "2026-02-08T12:00:00Z",
            "dnsProvider": null,
            "dnsValidationInstructions": null,
            "dnsValidationHostname": null,
            "dnsValidationTarget": null,
            "hostname": "example.com",
            "id": "cert_new",
            "source": "fly",
            "clientStatus": "Awaiting configuration",
            "issued": {"nodes": []}
        }"#;

        let cert: CertificateDetail = serde_json::from_str(json).unwrap();
        assert!(!cert.is_ready());
        assert_eq!(cert.status_display(), "pending");
        assert!(!cert.configured);
        assert!(cert.issued.unwrap().nodes.is_empty());
    }
}
