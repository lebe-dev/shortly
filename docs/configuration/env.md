# Configuration via environment variables

### Environment Variables

You can override any configuration value using environment variables. Environment variables have **higher priority** than values from `config.yml`.

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
cp .env.example .env

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
