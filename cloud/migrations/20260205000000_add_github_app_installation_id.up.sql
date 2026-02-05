-- Add github_app_installation_id to users table
-- This stores the GitHub App installation ID for the user's account
ALTER TABLE users ADD COLUMN github_app_installation_id INTEGER;
