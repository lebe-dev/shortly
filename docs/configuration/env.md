# Configuration via environment variables

### Environment Variables

You can override any configuration value using environment variables. Environment variables have **higher priority** than values from `config.yml`.

The application also reads a `.env` file from the working directory at startup, so every variable below may be placed there instead of the shell environment. Use `.env-dist` as a template. Variables already present in the environment are not overwritten by the file.

**Available variables:**

| Environment Variable | Config File Field | Description | Example | Sensitive |
|---------------------|-------------------|-------------|---------|-----------|
| `BIND` | `bind` | Server bind address | `0.0.0.0:8080` | No |
| `LOG_LEVEL` | `log-level` | Logging level | `debug`, `info`, `warn`, `error` | No |
| `LOG_TARGET` | `log-target` | Logging target | `console`, `file` | No |
| `DB_CNN` | `db-cnn` | SQLite connection string | `sqlite://./data/shortly.db?mode=rwc` | **Yes** |
| `BASE_URL` | `base-url` | Base URL for short links | `https://example.com` | No |
| `SHORT_URL_TTL` | `short-url.ttl` | Link TTL in hours | `168` (7 days) | No |
| `SHORT_URL_MAX_LENGTH` | `short-url.max-length` | Maximum URL length in characters | `2048` | No |
| `SCHEDULER_CLEANUP_EXPIRED_URLS` | `scheduler.cleanup-expired-urls` | Cron expression for cleanup | `0 0 * * *` | No |
| `FEATURES_CREATE_URL_ENABLED` | `features.create-url.enabled` | Enable URL creation feature | `true`, `false` | No |
| `FEATURES_CREATE_URL_AUTH_ONLY` | `features.create-url.auth-only` | Require authentication for URL creation | `true`, `false` | No |
| `FEATURES_CREATE_URL_MAX_PER_USER` | `features.create-url.max-per-user` | Fallback total URL limit per user | `100` | No |
| `FEATURES_CREATE_URL_MAX_PER_DAY` | `features.create-url.max-per-day` | Fallback daily URL limit per user | `10` | No |
| `AUTH_ENABLED` | `auth.enabled` | Enable authentication | `true`, `false` | No |
| `AUTH_TYPE` | `auth.type` | Authentication type | `gitlab` | No |
| `AUTH_PROVIDERS_GITLAB_BASE_URL` | `auth.providers.gitlab.base-url` | GitLab instance URL | `https://gitlab.com` | No |
| `AUTH_PROVIDERS_GITLAB_APPLICATION_ID` | `auth.providers.gitlab.application-id` | GitLab OAuth application ID | `your-app-id` | **Yes** |
| `AUTH_PROVIDERS_GITLAB_SECRET` | `auth.providers.gitlab.secret` | GitLab OAuth application secret | `your-secret` | **Yes** |
| `AUTH_NOTE` | `auth.note` | Note shown on the login page | `VPN required` | No |
| `AUTH_ADMIN_USERS` | `auth.admin-users` | Comma separated list of administrators | `alice,bob` | No |
| `METRICS_ENABLED` | `metrics.enabled` | Enable the metrics endpoint | `true`, `false` | No |

### Named link availability

These settings come from the environment only, including `.env`; they have no `config.yml` equivalents.

| Variable | Default | Description |
|----------|---------|-------------|
| `NAMED_URL_DEFAULT_ACCESS` | `public` | Access mode of new named links: `public` or `internal` |
| `NAMED_URL_ALLOW_ACCESS_OVERRIDE` | `false` | Allow the creator to choose a different mode in UI/API |
| `INTERNAL_NETWORK_CIDRS` | empty | Comma-separated client networks, IPv4/IPv6 CIDR |
| `TRUSTED_PROXY_CIDRS` | empty | Comma-separated proxy addresses/subnets trusted to send `X-Forwarded-For` |
| `NAMED_URL_CHECK_RATE_LIMIT` | `30` | Name probes per source per one-minute window, including named-link creation attempts |

Existing links keep public access. Changing the creation default does not change stored links. Generated links remain public. The backend rejects attempts to override the default when override is disabled, and returns the stored `access` in the creation response. The UI shows an internal-network notice after creating an `internal` link.

Set explicit company/VPN networks before selecting the `internal` default or enabling override; otherwise startup fails. Invalid CIDRs, access modes, flags and non-positive rate limits also fail startup. There are no automatically allowed private, loopback or link-local networks. Include public company/VPN egress addresses if these are the addresses seen by the proxy. Removing a network removes its access to existing internal links after restart; an empty network list denies all internal links.

Both direct redirects and `GET /api/url/{id}` enforce the stored mode, including lookups by an internal ID or case-insensitive custom name. External requests receive the same response as a missing link, without a destination or restriction flag. URL lookup is public independently of creation authentication settings. Owner/admin sessions do not bypass network restrictions when following a link; authenticated management views retain their existing permissions. Denied visits do not update `last_accessed`. Lookup responses carry `Cache-Control: private, no-store`; reverse proxies/CDNs must honor it and must not cache redirects or API lookups independently of the client network.

Name availability may reveal that a name is occupied, as may a creation conflict. Both operations share a rate limit, returning HTTP 429 with `Retry-After` and code `name_check_rate_limited`. IPv6 addresses are grouped by /64; unknown client addresses share one bucket. The limiter is bounded and local to each application instance, resets on restart, and allows up to the configured limit per replica. For a cluster-wide limit, also apply an ingress limit or use a single replica.

#### Client IP behind Nginx

The backend trusts forwarding headers only when its TCP peer is in `TRUSTED_PROXY_CIDRS`. It walks `X-Forwarded-For` from right to left, skips known proxies, and uses the first untrusted address. Missing, malformed, overlong or entirely trusted chains cannot authorize an internal link. Without a trusted proxy, the TCP peer is used and client-supplied headers are ignored. IPv4-mapped IPv6 client addresses are normalized to IPv4.

For a local Nginx forwarding directly to Shortly, a starting configuration is:

```dotenv
NAMED_URL_DEFAULT_ACCESS=internal
INTERNAL_NETWORK_CIDRS=10.20.0.0/16,fd12:3456::/32
TRUSTED_PROXY_CIDRS=127.0.0.1/32,::1/128
```

The client networks above are examples; replace them with your own. At the internet-facing Nginx, overwrite the incoming header with the actual client address:

```nginx
proxy_set_header X-Forwarded-For $remote_addr;
```

If Nginx itself sits behind another proxy, configure its real-IP module with only that proxy's trusted addresses. With ingress-nginx, configure the actual upstream load balancers through `proxy-real-ip-cidr` and enable forwarding only for a trusted upstream topology. Include the ingress pod IPs/subnets in the backend's `TRUSTED_PROXY_CIDRS`. If the chart's Nginx sidecar is enabled, include its loopback address as well and preserve the verified forwarding chain. Avoid trusting whole company/client ranges as proxies: they would be skipped during client identification. Restrict direct backend access to the intended proxy path.

See [Nginx real-IP configuration](https://nginx.org/en/docs/http/ngx_http_realip_module.html) and [ingress-nginx client IP handling](https://kubernetes.github.io/ingress-nginx/user-guide/miscellaneous/).

### Passkey variables

Passkey (WebAuthn) login is configured through the environment only, `config.yml` carries nothing about it.

| Environment Variable | Description | Example | Required |
|---------------------|-------------|---------|----------|
| `PASSKEY_ENABLED` | Enable login with a passkey | `true`, `false` | No, default `false` |
| `PASSKEY_RP_ID` | Relying party ID: the registrable domain, without scheme or port | `shortly.company.com` | Yes, when enabled |
| `PASSKEY_RP_ORIGIN` | Origin the browser reports, including scheme and port | `https://shortly.company.com` | Yes, when enabled |
| `PASSKEY_RP_NAME` | Name shown by the authenticator | `Shortly` | No, default `Shortly` |
| `PASSKEY_CHALLENGE_TTL` | Lifetime of an unfinished ceremony, in seconds | `300` | No, default `300` |

**Notes:**

- Passkeys require authentication to be enabled (`AUTH_ENABLED=true`), they are a second way into an account, not a replacement for GitLab.
- Login with a passkey works for **existing accounts only**. A user signs in with GitLab first and adds a passkey on the profile page; an administrator can remove every passkey of a user from the admin panel.
- Browsers only allow WebAuthn in a secure context: HTTPS, or `http://localhost` during development. `PASSKEY_RP_ID` must match the domain the service is served from, and `PASSKEY_RP_ORIGIN` must match scheme, host and port exactly.

**URL limits:** `FEATURES_CREATE_URL_MAX_PER_USER` and `FEATURES_CREATE_URL_MAX_PER_DAY` are only a fallback. Per-user quotas assigned by an administrator (`max_urls_per_user` / `max_urls_per_day` in the `users` table, editable in the admin panel) always win. New users get the database defaults: 100 total and 10 per day.

**Security Note:** Variables marked as "Sensitive" contain credentials and should be stored securely (e.g., in Kubernetes Secrets, Docker secrets, or encrypted environment files).

**Docker example:**

```bash
docker run -e BASE_URL="https://go.company.com" -e SHORT_URL_TTL=720 tinyops/shortly:1.0.0
```

## Docker Compose Configuration

When using Docker Compose, you have multiple options for setting environment variables.

### Option 1: Using .env file (Recommended)

Create a `.env` file in your project directory:

```bash
# Copy the example file
cp .env-dist .env

# Edit with your values
nano .env
```

Example `.env`:
```bash
BASE_URL=https://shortly.company.com
SHORT_URL_TTL=720
LOG_LEVEL=info
```

Docker Compose will automatically load these variables. No changes to `docker-compose.yml` needed.

### Option 2: Direct in docker-compose.yml

Edit the `environment` section in `docker-compose.yml`:

```yaml
services:
  app:
    environment:
      - BASE_URL=https://shortly.company.com
      - SHORT_URL_TTL=720
      - LOG_LEVEL=info
```

### Option 3: Export from shell

```bash
export BASE_URL="https://shortly.company.com"
export SHORT_URL_TTL=720
docker-compose up -d
```

**Note:** Variables defined in `.env` file or exported from shell will be automatically passed to the container via the `${VAR:-}` syntax in `docker-compose.yml`.
