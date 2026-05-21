-- 0003_create_workspace_members.up.sql
-- Junction table for workspace membership with RBAC roles.

CREATE TABLE workspace_members (
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role         TEXT NOT NULL DEFAULT 'viewer'
                 CHECK (role IN ('owner', 'admin', 'editor', 'viewer', 'guest')),
    joined_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

    PRIMARY KEY (workspace_id, user_id)
);

-- Quick lookup: "which workspaces does this user belong to?"
CREATE INDEX idx_workspace_members_user_id ON workspace_members (user_id);
