-- 0005_create_rooms.up.sql
-- A room is a live recording/collaboration space within a project.

CREATE TABLE rooms (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name       TEXT NOT NULL,
    slug       TEXT UNIQUE NOT NULL,
    host_id    UUID REFERENCES users(id),   -- NULL = no host assigned yet
    status     TEXT NOT NULL DEFAULT 'waiting'
               CHECK (status IN ('waiting', 'live', 'ended')),
    max_peers  INT NOT NULL DEFAULT 10
               CHECK (max_peers BETWEEN 1 AND 25),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    ended_at   TIMESTAMPTZ                    -- NULL until the room ends
);

CREATE INDEX idx_rooms_project_id ON rooms (project_id);
CREATE INDEX idx_rooms_slug       ON rooms (slug);
CREATE INDEX idx_rooms_status     ON rooms (status);
