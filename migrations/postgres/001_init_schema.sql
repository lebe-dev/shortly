-- Migration: 001_init_schema
-- Description: Initial database schema with URLs, users, and sessions

-- URLs table: stores short URL mappings
CREATE TABLE IF NOT EXISTS urls (
    id VARCHAR(255) NOT NULL,
    original_url TEXT NOT NULL,
    ttl INTEGER NOT NULL,
    created BIGINT NOT NULL,
    user_id BIGINT,
    CONSTRAINT urls_pkey UNIQUE(id)
);

CREATE INDEX IF NOT EXISTS idx_urls_user_id ON urls(user_id);

-- Users table: stores GitLab user information
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    gitlab_id BIGINT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    email TEXT,
    avatar_url TEXT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_users_gitlab_id ON users(gitlab_id);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- Sessions table: stores active user sessions
CREATE TABLE IF NOT EXISTS sessions (
    id BIGSERIAL PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    last_used_at BIGINT NOT NULL,
    expires_at BIGINT
);

CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);
