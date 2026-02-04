CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    github_repo TEXT NOT NULL,
    github_installation_id INTEGER,
    fly_app_name TEXT UNIQUE,
    fly_volume_id TEXT,
    custom_domain TEXT,
    preview_db_strategy TEXT DEFAULT 'copy',
    seed_file TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, name)
);
