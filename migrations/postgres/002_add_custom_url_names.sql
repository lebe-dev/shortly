-- Migration: 002_add_custom_url_names
-- Description: Add support for custom URL names, user limits, and audit trail

-- Add custom_name column to urls table
ALTER TABLE urls ADD COLUMN IF NOT EXISTS custom_name TEXT;

-- Create unique index on custom_name (case-insensitive using LOWER)
CREATE UNIQUE INDEX IF NOT EXISTS idx_urls_custom_name_lower
ON urls(LOWER(custom_name))
WHERE custom_name IS NOT NULL;

-- Add user limit columns to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS max_urls_per_user INTEGER DEFAULT 100;
ALTER TABLE users ADD COLUMN IF NOT EXISTS max_urls_per_day INTEGER DEFAULT 10;

-- Create audit log table
CREATE TABLE IF NOT EXISTS url_audit (
    id BIGSERIAL PRIMARY KEY,
    event_type TEXT NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    url_name TEXT,
    created_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_url_audit_user_id ON url_audit(user_id);
CREATE INDEX IF NOT EXISTS idx_url_audit_created_at ON url_audit(created_at);
CREATE INDEX IF NOT EXISTS idx_url_audit_user_created ON url_audit(user_id, created_at);
