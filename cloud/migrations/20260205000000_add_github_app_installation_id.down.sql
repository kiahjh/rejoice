-- SQLite doesn't support DROP COLUMN directly in older versions
-- For newer SQLite (3.35.0+), this works:
ALTER TABLE users DROP COLUMN github_app_installation_id;
