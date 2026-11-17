-- =================================================
-- tables.sql
-- =================================================

-- -------------------------------------------------
-- 1) events table
-- -------------------------------------------------

CREATE TABLE events (
    uuid UUID PRIMARY KEY DEFAULT get_random_uuid(),

    event_type TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    payload JSONB NOT NULL        
)

CREATE INDEX idx_events_created_at 
    ON events (created_at DESC);

CREATE INDEX idx_events_type 
    ON events (event_type);