-- 0012_create_webhook_deliveries.up.sql
-- Delivery log for outbound webhooks (for retry tracking).

CREATE TABLE webhook_deliveries (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    endpoint_id  UUID NOT NULL REFERENCES webhook_endpoints(id) ON DELETE CASCADE,
    event        TEXT NOT NULL,         -- e.g. "session.complete"
    payload      JSONB NOT NULL,
    status_code  INT,                   -- HTTP status from the target; NULL if not yet delivered
    attempt      INT NOT NULL DEFAULT 1,
    delivered_at TIMESTAMPTZ,           -- NULL until successfully delivered
    next_retry   TIMESTAMPTZ,           -- NULL when delivered or max retries exhausted
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_webhook_deliveries_endpoint_id ON webhook_deliveries (endpoint_id);
CREATE INDEX idx_webhook_deliveries_next_retry  ON webhook_deliveries (next_retry)
    WHERE next_retry IS NOT NULL;       -- partial index for pending retries
