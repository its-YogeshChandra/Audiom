-- 0008_create_transcripts.up.sql
-- Stores AI-generated transcriptions for sessions.
-- `content` is JSONB: array of { start_ms, end_ms, speaker, text } segments.

CREATE TABLE transcripts (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
    language   TEXT NOT NULL DEFAULT 'en',
    content    JSONB NOT NULL,   -- structured transcript segments
    srt_url    TEXT,             -- S3 key for .srt subtitle file
    vtt_url    TEXT,             -- S3 key for .vtt subtitle file
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_transcripts_session_id ON transcripts (session_id);
