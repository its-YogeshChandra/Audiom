-- 0010_create_episodes.up.sql
-- Published podcast episodes, optionally linked to a session.

CREATE TABLE episodes (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id   UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    session_id   UUID REFERENCES sessions(id),  -- NULL if manually uploaded
    title        TEXT NOT NULL,
    description  TEXT,
    audio_url    TEXT,             -- S3 key for final audio
    artwork_url  TEXT,             -- episode-specific artwork (overrides project artwork)
    duration_ms  BIGINT,
    season       INT,
    number       INT,             -- episode number
    status       TEXT NOT NULL DEFAULT 'draft'
                 CHECK (status IN ('draft', 'published', 'scheduled', 'archived')),
    published_at TIMESTAMPTZ,     -- NULL until published
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_episodes_project_id ON episodes (project_id);
CREATE INDEX idx_episodes_status     ON episodes (status);
