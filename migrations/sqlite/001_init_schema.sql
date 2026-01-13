-- Migration: 001_init_schema
-- Description: Initial database schema with URLs, users, and sessions

-- URLs table: stores short URL mappings
CREATE TABLE IF NOT EXISTS urls (
    id varchar,
    original_url text,
    ttl integer,
    created integer,
    user_id INTEGER,
    UNIQUE(id)
);

CREATE INDEX IF NOT EXISTS idx_urls_user_id ON urls(user_id);

-- Users table: stores GitLab user information
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    gitlab_id INTEGER NOT NULL UNIQUE,
    username TEXT NOT NULL,
    email TEXT,
    avatar_url TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_users_gitlab_id ON users(gitlab_id);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- Sessions table: stores active user sessions
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    last_used_at INTEGER NOT NULL,
    expires_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token);
CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at);
