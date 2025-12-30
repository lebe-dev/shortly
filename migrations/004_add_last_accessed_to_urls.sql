-- Migration: 004_add_last_accessed_to_urls
-- Description: Add last_accessed timestamp to track URL access events

ALTER TABLE urls ADD COLUMN last_accessed INTEGER;

-- Create index for efficient MAX() queries in metrics collection
CREATE INDEX IF NOT EXISTS idx_urls_last_accessed ON urls(last_accessed);

-- Initialize last_accessed to created timestamp for existing URLs
UPDATE urls SET last_accessed = created WHERE last_accessed IS NULL;
