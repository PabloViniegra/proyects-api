-- Create technologies table
CREATE TABLE IF NOT EXISTS technologies (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at TEXT NOT NULL
);

-- Create index on name for faster lookups
CREATE INDEX IF NOT EXISTS idx_technologies_name ON technologies(name);
