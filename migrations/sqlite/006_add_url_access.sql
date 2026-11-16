-- Existing links retain public access.
ALTER TABLE urls ADD COLUMN access TEXT NOT NULL DEFAULT 'public'
    CHECK (access IN ('public', 'internal'));
