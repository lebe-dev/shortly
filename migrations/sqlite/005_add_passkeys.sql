-- Migration: 005_add_passkeys
-- Description: WebAuthn (passkey) credentials, challenge state and stable WebAuthn user handles

-- Stable WebAuthn user handle (UUID) for existing accounts.
-- Filled lazily on the first passkey registration.
ALTER TABLE users ADD COLUMN webauthn_id TEXT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_webauthn_id ON users(webauthn_id);

-- Registered passkeys. One user may have several credentials.
CREATE TABLE IF NOT EXISTS passkey_credentials (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    credential_id TEXT NOT NULL UNIQUE,
    passkey TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    last_used_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_passkey_credentials_user_id ON passkey_credentials(user_id);

-- Short-lived WebAuthn ceremony state shared between the begin and finish requests.
-- Stored in the database so that the flow survives multi-instance deployments.
CREATE TABLE IF NOT EXISTS passkey_challenges (
    id TEXT PRIMARY KEY,
    user_id INTEGER,
    operation TEXT NOT NULL,
    state TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_passkey_challenges_expires_at ON passkey_challenges(expires_at);
