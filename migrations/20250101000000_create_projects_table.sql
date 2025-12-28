-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    repository_url TEXT NOT NULL,
    language TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create index on name for faster lookups
CREATE INDEX IF NOT EXISTS idx_projects_name ON projects(name);

-- Create index on language for filtering
CREATE INDEX IF NOT EXISTS idx_projects_language ON projects(language);
