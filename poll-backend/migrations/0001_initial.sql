CREATE TABLE IF NOT EXISTS events (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL,
    description TEXT,
    created_at  TEXT NOT NULL,
    admin_id    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS time_slots (
    id         TEXT PRIMARY KEY,
    event_id   TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    starts_at  TEXT NOT NULL,
    ends_at    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS participants (
    id         TEXT PRIMARY KEY,
    event_id   TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name       TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS votes (
    participant_id TEXT NOT NULL REFERENCES participants(id) ON DELETE CASCADE,
    time_slot_id   TEXT NOT NULL REFERENCES time_slots(id) ON DELETE CASCADE,
    available      INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (participant_id, time_slot_id)
);
