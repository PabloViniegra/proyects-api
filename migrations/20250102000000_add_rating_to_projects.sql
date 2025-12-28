-- Add rating column to projects table
ALTER TABLE projects ADD COLUMN rating REAL CHECK(rating IS NULL OR (rating >= 0.0 AND rating <= 5.0));

-- Create index on rating for filtering and sorting
CREATE INDEX IF NOT EXISTS idx_projects_rating ON projects(rating);
