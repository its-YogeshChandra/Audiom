-- 0004_create_projects.up.sql
-- Projects live inside workspaces; a project is a podcast/show.

CREATE TABLE projects (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name         TEXT NOT NULL,
    description  TEXT,
    artwork_url  TEXT,
    rss_slug     TEXT UNIQUE,  -- NULL if RSS feed not enabled
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_projects_workspace_id ON projects (workspace_id);
