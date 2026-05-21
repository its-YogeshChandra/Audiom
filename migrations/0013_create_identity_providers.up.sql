-- 0013_create_identity_providers.up.sql
-- SSO identity providers (SAML/OIDC) configured per workspace.

CREATE TABLE identity_providers (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    workspace_id UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    provider     TEXT NOT NULL
                 CHECK (provider IN ('saml', 'oidc')),
    issuer_url   TEXT NOT NULL,
    metadata     JSONB NOT NULL,   -- SP/IdP config, certificates, client_id/secret, etc.
    active       BOOLEAN NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_identity_providers_workspace_id ON identity_providers (workspace_id);
