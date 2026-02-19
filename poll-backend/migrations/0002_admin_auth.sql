CREATE TABLE IF NOT EXISTS admins (
    id            TEXT PRIMARY KEY,
    name          TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS admin_sessions (
    id         TEXT PRIMARY KEY,
    admin_id   TEXT NOT NULL REFERENCES admins(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_admin_sessions_admin_id ON admin_sessions(admin_id);
CREATE INDEX IF NOT EXISTS idx_events_admin_id ON events(admin_id);
