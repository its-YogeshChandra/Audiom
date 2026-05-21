-- 0011_create_webhook_endpoints.up.sql
-- Webhook endpoints configured per workspace.

CREATE TABLE webhook_endpoints (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    url          TEXT NOT NULL,
    secret       TEXT NOT NULL,       -- HMAC-SHA256 signing key
    events       TEXT[] NOT NULL,     -- array of event types: {"session.complete", "episode.published", ...}
    active       BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_webhook_endpoints_workspace_id ON webhook_endpoints (workspace_id);
