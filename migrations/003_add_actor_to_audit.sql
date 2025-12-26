-- Migration: 003_add_actor_to_audit
-- Description: Add actor_user_id to track who performed the action (especially for admin actions)

-- Add new column for actor (who performed the action)
ALTER TABLE url_audit ADD COLUMN actor_user_id INTEGER;

-- Populate actor_user_id from existing user_id (they were the same before)
UPDATE url_audit SET actor_user_id = user_id WHERE actor_user_id IS NULL;

-- Rename user_id to target_user_id for clarity
ALTER TABLE url_audit RENAME COLUMN user_id TO target_user_id;

-- Make actor_user_id NOT NULL after populating
-- Note: SQLite doesn't support ALTER COLUMN, so we need to recreate the table
CREATE TABLE url_audit_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    actor_user_id INTEGER NOT NULL,
    target_user_id INTEGER NOT NULL,
    url_name TEXT,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (actor_user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (target_user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Copy data from old table
INSERT INTO url_audit_new (id, event_type, actor_user_id, target_user_id, url_name, created_at)
SELECT id, event_type, actor_user_id, target_user_id, url_name, created_at FROM url_audit;

-- Drop old table and rename new one
DROP TABLE url_audit;
ALTER TABLE url_audit_new RENAME TO url_audit;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS idx_url_audit_actor_user_id ON url_audit(actor_user_id);
CREATE INDEX IF NOT EXISTS idx_url_audit_target_user_id ON url_audit(target_user_id);
CREATE INDEX IF NOT EXISTS idx_url_audit_created_at ON url_audit(created_at);
CREATE INDEX IF NOT EXISTS idx_url_audit_actor_created ON url_audit(actor_user_id, created_at);
