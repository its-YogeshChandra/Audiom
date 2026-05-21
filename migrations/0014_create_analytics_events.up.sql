-- 0014_create_analytics_events.up.sql
-- Analytics events for tracking downloads, plays, engagement, etc.

CREATE TABLE analytics_events (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type   TEXT NOT NULL,   -- download | play | pause | drop_off | share
    resource     TEXT NOT NULL,   -- episode | session | clip
    resource_id  UUID,
    workspace_id UUID REFERENCES workspaces(id) ON DELETE SET NULL,
    metadata     JSONB,           -- extra dimensions (country, device, browser, etc.)
    ip_address   INET,
    user_agent   TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Time-series queries: "events for workspace X in the last 7 days"
CREATE INDEX idx_analytics_workspace_created ON analytics_events (workspace_id, created_at);
CREATE INDEX idx_analytics_event_type        ON analytics_events (event_type);
CREATE INDEX idx_analytics_resource          ON analytics_events (resource, resource_id);
