-- 0007_create_tracks.up.sql
-- Individual media tracks (audio/video/screen) uploaded per participant per session.

CREATE TABLE tracks (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id    UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    user_id       UUID REFERENCES users(id),  -- NULL if an anonymous guest
    kind          TEXT NOT NULL
                  CHECK (kind IN ('audio', 'video', 'screen')),
    codec         TEXT,          -- e.g. opus, aac, h264, vp9
    sample_rate   INT,           -- audio only: 44100, 48000, etc.
    channels      INT,           -- audio only: 1 (mono) or 2 (stereo)
    raw_url       TEXT,          -- S3 key for the raw upload
    processed_url TEXT,          -- S3 key for the processed output
    size_bytes    BIGINT,
    duration_ms   BIGINT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tracks_session_id ON tracks (session_id);
CREATE INDEX idx_tracks_user_id    ON tracks (user_id);
