-- Migration: 003_add_actor_to_audit
-- Description: Add actor_user_id to track who performed the action

-- Add new column for actor (who performed the action)
ALTER TABLE url_audit ADD COLUMN IF NOT EXISTS actor_user_id BIGINT;

-- Populate actor_user_id from existing user_id (they were the same before)
UPDATE url_audit SET actor_user_id = user_id WHERE actor_user_id IS NULL;

-- Rename user_id to target_user_id for clarity
ALTER TABLE url_audit RENAME COLUMN user_id TO target_user_id;

-- Make actor_user_id NOT NULL after populating
ALTER TABLE url_audit ALTER COLUMN actor_user_id SET NOT NULL;

-- Add foreign key constraint
ALTER TABLE url_audit
ADD CONSTRAINT fk_audit_actor FOREIGN KEY (actor_user_id)
REFERENCES users(id) ON DELETE CASCADE;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS idx_url_audit_actor_user_id ON url_audit(actor_user_id);
CREATE INDEX IF NOT EXISTS idx_url_audit_target_user_id ON url_audit(target_user_id);
CREATE INDEX IF NOT EXISTS idx_url_audit_actor_created ON url_audit(actor_user_id, created_at);
