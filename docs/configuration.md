# Configuration

The application configuration is loaded from `config.yml` file with support for environment variable overrides.

### Environment Variables

You can override any configuration value using environment variables. Environment variables have **higher priority** than values from `config.yml`.

**Available variables:**

| Environment Variable | Config File Field | Description | Example |
|---------------------|-------------------|-------------|---------|
| `BIND` | `bind` | Server bind address | `0.0.0.0:8080` |
| `LOG_LEVEL` | `log-level` | Logging level | `debug`, `info`, `warn`, `error` |
| `LOG_TARGET` | `log-target` | Logging target | `console`, `file` |
| `DB_CNN` | `db-cnn` | SQLite connection string | `sqlite://./data/app.db?mode=rwc` |
| `BASE_URL` | `base-url` | Base URL for short links | `https://example.com` |
| `SHORT_URL_TTL` | `short-url.ttl` | Link TTL in hours | `168` (7 days) |
| `SCHEDULER_CLEANUP_EXPIRED_URLS` | `scheduler.cleanup-expired-urls` | Cron expression for cleanup | `0 0 0 * * *` |

**Usage example:**

```bash
# Override server port
export BIND="0.0.0.0:9090"

# Override database connection
export DB_CNN="sqlite://./data/production.db"

# Override link TTL to 30 days
export SHORT_URL_TTL=720

# Start the server
cargo run --bin server
```

**Docker example:**

```bash
docker run -e BASE_URL="https://shortly.company.com" -e SHORT_URL_TTL=720 tinyops/shortly:0.1.0
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
