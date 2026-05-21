-- 0002_create_workspaces.up.sql
-- Workspaces are the top-level organizational unit (like an org/team).

CREATE TABLE workspaces (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT NOT NULL,
    slug       TEXT UNIQUE NOT NULL,
    owner_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    plan       TEXT NOT NULL DEFAULT 'free',  -- free | pro | enterprise
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_workspaces_slug     ON workspaces (slug);
CREATE INDEX idx_workspaces_owner_id ON workspaces (owner_id);
