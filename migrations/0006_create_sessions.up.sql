-- 0006_create_sessions.up.sql
-- A session is a single recording run within a room.

CREATE TABLE sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    room_id     UUID NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    status      TEXT NOT NULL DEFAULT 'recording'
                CHECK (status IN ('recording', 'processing', 'completed', 'failed')),
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at    TIMESTAMPTZ,
    duration_ms BIGINT       -- computed after session ends
);

CREATE INDEX idx_sessions_room_id ON sessions (room_id);
CREATE INDEX idx_sessions_status  ON sessions (status);
