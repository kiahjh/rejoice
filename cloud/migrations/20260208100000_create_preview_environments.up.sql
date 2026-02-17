-- Preview environments: tracks active preview deployments for PRs.
-- Each open PR with a preview gets a separate Fly app and optionally a volume.
CREATE TABLE preview_environments (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    pr_number INTEGER NOT NULL,
    pr_branch TEXT NOT NULL,
    fly_app_name TEXT NOT NULL UNIQUE,
    fly_volume_id TEXT,
    status TEXT NOT NULL DEFAULT 'active',  -- 'active', 'destroying', 'destroyed'
    url TEXT,
    github_comment_id INTEGER,  -- ID of the PR comment we posted (for updates)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    destroyed_at DATETIME,
    UNIQUE(project_id, pr_number)
);

CREATE INDEX idx_preview_environments_project_id ON preview_environments(project_id);
CREATE INDEX idx_preview_environments_status ON preview_environments(status);
