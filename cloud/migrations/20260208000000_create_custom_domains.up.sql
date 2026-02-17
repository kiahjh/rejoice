-- Custom domains for projects
CREATE TABLE custom_domains (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    hostname TEXT NOT NULL UNIQUE,
    fly_cert_id TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    dns_validation_hostname TEXT,
    dns_validation_target TEXT,
    configured BOOLEAN DEFAULT FALSE,
    acme_dns_configured BOOLEAN DEFAULT FALSE,
    certificate_authority TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    checked_at DATETIME
);

CREATE INDEX idx_custom_domains_project_id ON custom_domains(project_id);
