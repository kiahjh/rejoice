CREATE TABLE deployments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    git_sha TEXT NOT NULL,
    git_branch TEXT NOT NULL,
    git_message TEXT,
    pr_number INTEGER,
    status TEXT NOT NULL DEFAULT 'pending',
    fly_machine_id TEXT,
    url TEXT,
    build_logs TEXT,
    error_message TEXT,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    finished_at DATETIME
);

CREATE INDEX idx_deployments_project_id ON deployments(project_id);
CREATE INDEX idx_deployments_status ON deployments(status);
