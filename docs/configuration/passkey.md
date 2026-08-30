# Passkey (WebAuthn) authentication

Passkeys let a user sign in with Touch ID, Windows Hello, a phone or a hardware key instead of going through GitLab. They are an **additional** way into an account, GitLab OAuth stays in place.

Login with a passkey works for **existing accounts only**: a passkey can be created only from an authenticated session, and a passkey that is not linked to an account is rejected at login.

## Configuration

Passkeys are configured through environment variables only. Nothing about them is stored in `config.yml`. The application loads a `.env` file from the working directory at startup, see `.env-dist` for a template.

```bash
PASSKEY_ENABLED=true
PASSKEY_RP_ID=shortly.company.com
PASSKEY_RP_ORIGIN=https://shortly.company.com
PASSKEY_RP_NAME=Shortly
PASSKEY_CHALLENGE_TTL=300
```

| Variable | Description |
|----------|-------------|
| `PASSKEY_ENABLED` | Enables the feature. Default `false` |
| `PASSKEY_RP_ID` | Registrable domain of the service, without scheme or port |
| `PASSKEY_RP_ORIGIN` | Origin the browser reports, including scheme and port |
| `PASSKEY_RP_NAME` | Name shown by the authenticator. Default `Shortly` |
| `PASSKEY_CHALLENGE_TTL` | Lifetime of an unfinished ceremony, in seconds. Default `300` |

The server refuses to start when `PASSKEY_ENABLED=true` and `PASSKEY_RP_ID` or `PASSKEY_RP_ORIGIN` is missing. When authentication itself is disabled (`AUTH_ENABLED=false`), passkeys stay switched off and a warning is written to the log.

## Requirements

- Authentication must be enabled (`AUTH_ENABLED=true`).
- Browsers allow WebAuthn only in a secure context: HTTPS, or `http://localhost` for development.
- `PASSKEY_RP_ID` must be the domain the page is served from, or its parent domain. `PASSKEY_RP_ORIGIN` must match scheme, host and port exactly.

Development example:

```bash
PASSKEY_ENABLED=true
PASSKEY_RP_ID=localhost
PASSKEY_RP_ORIGIN=http://localhost:8080
```

## How it is used

1. The user signs in with GitLab.
2. On the profile page they press "Add passkey", name it and confirm in the browser dialog.
3. On the login page a "Login with passkey" button appears. No username is asked for: the browser offers the passkeys it holds for the site.
4. A user may register several passkeys and delete any of them on the profile page.
5. An administrator can delete every passkey of a user in the admin panel, on the user card.

Registering and deleting a passkey are written to the audit log as `passkey_register` and `passkey_delete`. Logging in with a passkey is recorded as a regular `user_login` event.

## Storage

- `passkey_credentials` holds the registered credentials: the public key, the signature counter and the name given by the user.
- `passkey_challenges` holds the state of an unfinished ceremony. Rows expire after `PASSKEY_CHALLENGE_TTL` seconds and a scheduled job removes them every five minutes. The state lives in the database, so the flow survives a restart and works with several instances behind a load balancer.
- `users.webauthn_id` holds the stable WebAuthn handle of an account. It is created when the first passkey is registered.

Removing a user's passkeys does not touch the account itself, the user can still sign in with GitLab and register a new passkey.
