-- 0009_create_audit_logs.up.sql
-- Append-only audit trail for all mutations in the system.

CREATE TABLE audit_logs (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID REFERENCES users(id),  -- NULL for system-initiated actions
    action      TEXT NOT NULL,              -- e.g. "user.login", "room.create", "session.delete"
    resource    TEXT NOT NULL,              -- e.g. "user", "workspace", "room"
    resource_id UUID,                       -- the ID of the affected resource
    metadata    JSONB,                      -- extra context (old values, request info, etc.)
    ip_address  INET,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Query patterns: "show me all actions by user X" and "show me all actions on resource Y"
CREATE INDEX idx_audit_logs_user_id     ON audit_logs (user_id);
CREATE INDEX idx_audit_logs_resource    ON audit_logs (resource, resource_id);
CREATE INDEX idx_audit_logs_created_at  ON audit_logs (created_at);
