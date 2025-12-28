-- Create project_technologies pivot table (many-to-many)
CREATE TABLE IF NOT EXISTS project_technologies (
    project_id TEXT NOT NULL,
    technology_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    PRIMARY KEY (project_id, technology_id),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (technology_id) REFERENCES technologies(id) ON DELETE CASCADE
);

-- Create indexes for faster lookups
CREATE INDEX IF NOT EXISTS idx_project_technologies_project ON project_technologies(project_id);
CREATE INDEX IF NOT EXISTS idx_project_technologies_technology ON project_technologies(technology_id);
