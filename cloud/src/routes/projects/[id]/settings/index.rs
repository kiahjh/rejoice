use crate::components::{self as ui, icon, BadgeVariant, ButtonSize, ButtonVariant};
use crate::crypto;
use crate::AppState;
use rejoice::db::{query_as, FromRow};
use rejoice::{html, Req, Res};

#[derive(FromRow)]
struct Project {
    id: String,
    name: String,
    fly_app_name: Option<String>,
}

#[derive(FromRow)]
struct EnvVar {
    id: String,
    key: String,
    encrypted_value: Vec<u8>,
    is_preview_only: bool,
}

#[derive(FromRow)]
struct CustomDomain {
    id: String,
    hostname: String,
    status: String,
    dns_validation_hostname: Option<String>,
    dns_validation_target: Option<String>,
    #[allow(dead_code)]
    configured: bool,
    acme_dns_configured: bool,
}

pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res {
    let github_id: i64 = match req.cookies.get("session").and_then(|s| s.parse().ok()) {
        Some(id) => id,
        None => return res.redirect("/"),
    };

    // Verify project belongs to user
    let project = query_as::<_, Project>(
        r#"
        SELECT p.id, p.name, p.fly_app_name
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

    // Fetch env vars
    let env_vars = query_as::<_, EnvVar>(
        r#"
        SELECT id, key, encrypted_value, is_preview_only
        FROM env_vars
        WHERE project_id = ?
        ORDER BY key ASC
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // Fetch custom domains
    let domains = query_as::<_, CustomDomain>(
        r#"
        SELECT id, hostname, status, dns_validation_hostname,
               dns_validation_target, configured, acme_dns_configured
        FROM custom_domains
        WHERE project_id = ?
        ORDER BY created_at ASC
        "#,
    )
    .bind(&id)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let has_fly_app = project.fly_app_name.is_some();
    let fly_app_name = project.fly_app_name.clone().unwrap_or_default();

    // Fetch IP addresses for DNS instructions (only if deployed and has pending domains)
    let app_ips = if has_fly_app && domains.iter().any(|d| d.status != "ready") {
        match state.fly.list_ip_addresses(&fly_app_name).await {
            Ok(ips) => ips,
            Err(e) => {
                eprintln!("Failed to fetch IPs for {}: {}", fly_app_name, e);
                vec![]
            }
        }
    } else {
        vec![]
    };

    let ipv4 = app_ips
        .iter()
        .find(|ip| ip.ip_type == "v4")
        .map(|ip| ip.address.clone());
    let ipv6 = app_ips
        .iter()
        .find(|ip| ip.ip_type == "v6")
        .map(|ip| ip.address.clone());

    res.html(html! {
        div class="max-w-4xl mx-auto px-6 py-10" {
            // Back link
            (ui::back_link(&format!("/projects/{}", id), &project.name))

            // Header
            div class="mt-6 mb-10" {
                div class="flex items-center gap-3" {
                    div class="w-10 h-10 rounded-xl bg-[var(--bg-surface)] flex items-center justify-center text-[var(--text-muted)]" {
                        (icon::settings(20))
                    }
                    div {
                        h1 class="text-2xl font-semibold text-[var(--text-primary)] tracking-tight" { "Settings" }
                        p class="text-sm text-[var(--text-muted)]" { "Configure your project settings" }
                    }
                }
            }

            // Main content
            div class="space-y-8" {
                // Custom Domains Section
                (ui::card(html! {
                    (ui::card_header(
                        "Custom Domains",
                        Some("Add your own domain to serve your app. SSL certificates are automatically provisioned.")
                    ))

                    @if !has_fly_app {
                        // Can't add domains until deployed
                        div class="py-6 text-center" {
                            div class="flex justify-center text-[var(--text-faint)] mb-3" {
                                (icon::globe(32))
                            }
                            p class="text-sm text-[var(--text-muted)]" { "Deploy your project first to add custom domains." }
                            p class="text-xs text-[var(--text-faint)] mt-1" {
                                "Custom domains require an active deployment on Fly.io."
                            }
                        }
                    } @else {
                        // Add new domain form
                        form
                            method="POST"
                            action=(format!("/projects/{}/settings/domains", id))
                            class="mb-6"
                        {
                            div class="flex gap-3 items-end" {
                                div class="flex-1" {
                                    label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5" { "Domain" }
                                    (ui::input("hostname", "example.com"))
                                }
                                div {
                                    label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5 invisible" { "Add" }
                                    (ui::button_submit("Add domain", ButtonVariant::Primary, ButtonSize::Medium))
                                }
                            }
                        }

                        (ui::card_divider())

                        // Existing domains list
                        @if domains.is_empty() {
                            div class="py-8 text-center" {
                                div class="flex justify-center text-[var(--text-faint)] mb-3" {
                                    (icon::globe(32))
                                }
                                p class="text-sm text-[var(--text-muted)]" { "No custom domains configured" }
                                p class="text-xs text-[var(--text-faint)] mt-1" { "Add a domain above to get started." }
                            }
                        } @else {
                            div class="space-y-4" {
                                @for domain in &domains {
                                    (domain_row(&project.id, domain, &fly_app_name, &ipv4, &ipv6))
                                }
                            }
                        }
                    }
                }))

                // Environment Variables Section
                (ui::card(html! {
                    (ui::card_header(
                        "Environment Variables",
                        Some("Encrypted at rest and injected at deploy time. Changes require a new deployment.")
                    ))

                    // Add new env var form
                    form
                        method="POST"
                        action=(format!("/projects/{}/settings/env", id))
                        class="mb-6"
                    {
                        div class="grid grid-cols-[1fr_1fr_auto_auto] gap-3 items-end" {
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5" { "Key" }
                                input
                                    type="text"
                                    name="key"
                                    placeholder="VARIABLE_NAME"
                                    autocomplete="off"
                                    class="w-full h-10 px-3.5 text-sm font-mono uppercase \
                                           bg-[var(--bg-base)] border border-[var(--border-default)] rounded-lg \
                                           text-[var(--text-primary)] placeholder-[var(--text-faint)] \
                                           outline-none transition-all duration-150 \
                                           hover:border-[var(--border-strong)] \
                                           focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent-subtle)]";
                            }
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5" { "Value" }
                                (ui::input_password("value", "Secret value"))
                            }
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5 invisible" { "Preview" }
                                div class="h-10 flex items-center" {
                                    (ui::checkbox("preview_only", "Preview only", false))
                                }
                            }
                            div {
                                label class="block text-xs font-medium text-[var(--text-muted)] mb-1.5 invisible" { "Add" }
                                (ui::button_submit("Add", ButtonVariant::Primary, ButtonSize::Medium))
                            }
                        }
                    }

                    (ui::card_divider())

                    // Existing env vars list
                    @if env_vars.is_empty() {
                        div class="py-8 text-center" {
                            div class="flex justify-center text-[var(--text-faint)] mb-3" {
                                (icon::key(32))
                            }
                            p class="text-sm text-[var(--text-muted)]" { "No environment variables configured" }
                            p class="text-xs text-[var(--text-faint)] mt-1" { "Add variables above to get started." }
                        }
                    } @else {
                        div class="space-y-1" {
                            // Header row
                            div class="grid grid-cols-[1fr_1fr_auto_auto] gap-3 px-3 py-2 text-xs font-medium text-[var(--text-faint)] uppercase tracking-wider" {
                                span { "Key" }
                                span { "Value" }
                                span { "Scope" }
                                span { "" }
                            }
                            
                            @for env_var in &env_vars {
                                (env_var_row(&project.id, env_var, &state.encryption_key))
                            }
                        }
                    }
                }))

                // Danger Zone
                (ui::card(html! {
                    div class="flex items-start gap-3" {
                        div class="flex-shrink-0 w-8 h-8 rounded-lg bg-red-500/10 flex items-center justify-center text-red-400" {
                            (icon::trash(16))
                        }
                        div class="flex-1" {
                            h2 class="text-sm font-medium text-red-400" { "Danger Zone" }
                            p class="text-xs text-[var(--text-muted)] mt-1" { 
                                "Permanently delete this project and all its deployments. This action cannot be undone."
                            }
                        }
                        form method="POST" action=(format!("/projects/{}/delete", id)) {
                            (ui::button_submit("Delete project", ButtonVariant::Danger, ButtonSize::Small))
                        }
                    }
                }))
            }
        }
    })
}

fn domain_row(
    project_id: &str,
    domain: &CustomDomain,
    fly_app_name: &str,
    ipv4: &Option<String>,
    ipv6: &Option<String>,
) -> rejoice::Markup {
    let status_badge = match domain.status.as_str() {
        "ready" => ui::badge("Active", BadgeVariant::Success),
        "validating" => ui::badge("Validating", BadgeVariant::Warning),
        "error" => ui::badge("Error", BadgeVariant::Error),
        _ => ui::badge("Pending", BadgeVariant::Default),
    };

    let needs_dns_setup = domain.status != "ready";

    // Determine if this is an apex domain (no subdomain, e.g. "example.com")
    // vs a subdomain (e.g. "www.example.com", "app.example.com")
    let is_apex = domain.hostname.matches('.').count() == 1;

    // The DNS "Name" field value: "@" for apex, or the subdomain part for subdomains
    let dns_name = if is_apex {
        "@".to_string()
    } else {
        // e.g. "www.example.com" -> "www", "app.sub.example.com" -> "app.sub"
        let tld_parts: Vec<&str> = domain.hostname.rsplitn(3, '.').collect();
        if tld_parts.len() == 3 {
            tld_parts[2].to_string()
        } else {
            domain.hostname.clone()
        }
    };

    let cname_target = format!("{}.fly.dev", fly_app_name);

    html! {
        div class="rounded-lg border border-[var(--border-subtle)] p-4 space-y-3" {
            // Domain header row
            div class="flex items-center justify-between" {
                div class="flex items-center gap-3" {
                    // Status dot
                    @if domain.status == "ready" {
                        span class="block w-2 h-2 rounded-full bg-emerald-500" {}
                    } @else {
                        span class="block w-2 h-2 rounded-full bg-amber-500 animate-pulse" {}
                    }
                    // Hostname
                    code class="text-sm font-mono text-[var(--text-primary)] font-medium" { (&domain.hostname) }
                    (status_badge)
                }
                // Actions
                div class="flex items-center gap-1" {
                    // Check status button
                    form method="POST" action=(format!("/projects/{}/settings/domains/{}/check", project_id, domain.id)) {
                        button
                            type="submit"
                            class="p-1.5 rounded-md text-[var(--text-faint)] hover:text-[var(--text-secondary)] hover:bg-[var(--bg-surface)] transition-colors"
                            title="Re-check certificate status"
                        {
                            (icon::refresh(14))
                        }
                    }
                    // Delete button
                    form method="POST" action=(format!("/projects/{}/settings/domains/{}/delete", project_id, domain.id)) {
                        button
                            type="submit"
                            class="p-1.5 rounded-md text-[var(--text-faint)] hover:text-red-400 hover:bg-red-500/10 transition-colors"
                            title="Remove domain"
                        {
                            (icon::trash(14))
                        }
                    }
                }
            }

            // DNS configuration instructions (shown when not ready)
            @if needs_dns_setup {
                div class="mt-3 p-4 rounded-lg bg-[var(--bg-base)] border border-[var(--border-subtle)]" {
                    p class="text-xs font-medium text-[var(--text-primary)] mb-3" {
                        "Add these DNS records at your domain provider:"
                    }

                    // Recommended option based on domain type
                    @if is_apex {
                        // Apex domain: recommend A/AAAA records
                        p class="text-[11px] text-[var(--text-muted)] mb-2" {
                            "Since this is a root domain, use A and AAAA records:"
                        }

                        (dns_record_table(&[
                            ("A", &dns_name, ipv4.as_deref().unwrap_or("(no IPv4 allocated)")),
                            ("AAAA", &dns_name, ipv6.as_deref().unwrap_or("(no IPv6 allocated)")),
                        ]))
                    } @else {
                        // Subdomain: recommend CNAME
                        p class="text-[11px] text-[var(--text-muted)] mb-2" {
                            "Since this is a subdomain, use a CNAME record:"
                        }

                        (dns_record_table(&[
                            ("CNAME", &dns_name, &cname_target),
                        ]))
                    }

                    // ACME DNS challenge (if needed and available)
                    @if !domain.acme_dns_configured {
                        @if let (Some(val_hostname), Some(val_target)) = (&domain.dns_validation_hostname, &domain.dns_validation_target) {
                            div class="mt-3 pt-3 border-t border-[var(--border-subtle)]" {
                                p class="text-[11px] text-[var(--text-muted)] mb-2" {
                                    "Optional: add this record to issue the SSL certificate before pointing traffic:"
                                }
                                (dns_record_table(&[
                                    ("CNAME", val_hostname, val_target),
                                ]))
                            }
                        }
                    }

                    div class="mt-3 pt-3 border-t border-[var(--border-subtle)]" {
                        p class="text-[11px] text-[var(--text-muted)]" {
                            "After adding these records, click "
                            span class="inline-flex align-middle" { (icon::refresh(12)) }
                            " to re-check. SSL certificates are issued automatically and may take a few minutes."
                        }
                    }
                }
            }

            // Show success info when ready
            @if domain.status == "ready" {
                div class="text-xs text-emerald-400/80" {
                    "SSL certificate active. Your domain is serving traffic."
                }
            }
        }
    }
}

/// Renders a clean DNS record table with click-to-copy values.
fn dns_record_table(records: &[(&str, &str, &str)]) -> rejoice::Markup {
    html! {
        div class="rounded-md border border-[var(--border-subtle)] overflow-hidden" {
            // Header
            div class="grid grid-cols-[60px_1fr_1fr] text-[10px] font-medium text-[var(--text-faint)] uppercase tracking-wider bg-[var(--bg-surface)] px-3 py-1.5" {
                span { "Type" }
                span { "Name" }
                span { "Value" }
            }
            // Rows
            @for (record_type, name, value) in records {
                div class="grid grid-cols-[60px_1fr_1fr] items-center px-3 py-2 border-t border-[var(--border-subtle)]" {
                    span class="text-xs font-mono font-medium text-[var(--text-secondary)]" { (record_type) }
                    (copyable_cell(name))
                    (copyable_cell(value))
                }
            }
        }
    }
}

/// Renders a clickable cell that copies its text to the clipboard on click.
fn copyable_cell(text: &str) -> rejoice::Markup {
    // The onclick handler: copy text, then swap to "Copied!" briefly
    let onclick = format!(
        "navigator.clipboard.writeText(this.dataset.copy).then(()=>{{let o=this.querySelector('.copy-icon'),c=this.querySelector('.check-icon');o.style.display='none';c.style.display='block';this.classList.add('text-emerald-400');setTimeout(()=>{{o.style.display='block';c.style.display='none';this.classList.remove('text-emerald-400')}},1500)}})",
    );

    html! {
        button
            type="button"
            data-copy=(text)
            onclick=(onclick)
            class="group/copy inline-flex items-center gap-1.5 text-left text-xs font-mono text-[var(--accent-light)] \
                   hover:text-[var(--text-primary)] cursor-pointer transition-colors duration-150 \
                   rounded px-1 -mx-1 py-0.5 hover:bg-[var(--bg-surface)] w-fit"
            title="Click to copy"
        {
            code class="break-all" { (text) }
            span class="flex-shrink-0 text-[var(--text-faint)] opacity-0 group-hover/copy:opacity-100 transition-opacity" {
                // Copy icon (shown by default)
                span class="copy-icon" style="display:block" {
                    (icon::copy(12))
                }
                // Check icon (shown after copy)
                span class="check-icon" style="display:none" {
                    (icon::check_circle(12))
                }
            }
        }
    }
}

fn env_var_row(project_id: &str, env_var: &EnvVar, key: &[u8; 32]) -> rejoice::Markup {
    // Decrypt value for display (masked)
    let decrypted = crypto::decrypt(&env_var.encrypted_value, key).unwrap_or_default();
    let masked = mask_value(&decrypted);

    html! {
        div class="group grid grid-cols-[1fr_1fr_auto_auto] gap-3 items-center px-3 py-2.5 rounded-lg hover:bg-[var(--bg-surface)] transition-colors" {
            // Key name
            code class="text-sm font-mono text-[var(--text-primary)] truncate" { (&env_var.key) }

            // Masked value
            code class="text-sm font-mono text-[var(--text-faint)] truncate" { (masked) }

            // Preview only badge or All badge
            @if env_var.is_preview_only {
                (ui::badge("Preview", BadgeVariant::Accent))
            } @else {
                (ui::badge("All", BadgeVariant::Default))
            }

            // Delete button
            form
                method="POST"
                action=(format!("/projects/{}/settings/env/{}/delete", project_id, env_var.id))
            {
                button
                    type="submit"
                    class="p-1.5 rounded-md text-[var(--text-faint)] hover:text-red-400 hover:bg-red-500/10 transition-colors opacity-0 group-hover:opacity-100"
                    title="Delete"
                {
                    (icon::trash(14))
                }
            }
        }
    }
}

fn mask_value(value: &str) -> String {
    if value.len() <= 4 {
        "*".repeat(value.len().max(4))
    } else {
        format!("{}...", "*".repeat(4))
    }
}
