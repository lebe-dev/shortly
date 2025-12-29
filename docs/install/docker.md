# Install Shortly with Docker Compose

This guide walks you through deploying Shortly URL shortener using Docker Compose with nginx as a reverse proxy, featuring IP-based rate limiting and security hardening.

## Overview

The deployment architecture consists of:
- **nginx** (Alpine): Reverse proxy with TLS termination, rate limiting, and security headers
- **Shortly app**: URL shortener application (Rust + SvelteKit)
- **SQLite database**: Persistent storage (file-based)
- **Docker network**: Isolated bridge network for service communication

Only nginx exposes ports 80/443 to the host. All traffic is proxied to the Shortly app with rate limiting and security controls applied.

## Prerequisites

- Docker 20.10+ and Docker Compose 2.0+
- Domain name pointing to your server
- SSL/TLS certificate and private key for your domain
- Linux server with:
  - 1GB+ RAM
  - 10GB+ disk space
  - Open ports: 80, 443

## Quick Start

```bash
# Create installation directory
mkdir -p /opt/shortly
cd /opt/shortly

# Create directory structure
mkdir -p nginx/conf.d nginx/ssl data logs/nginx logs/app

# Download example configurations
# (Copy files from examples/ directory in the Shortly repository)
curl -o docker-compose.yml https://raw.githubusercontent.com/yourrepo/shortly/main/examples/docker-compose.nginx.yml
curl -o nginx/nginx.conf https://raw.githubusercontent.com/yourrepo/shortly/main/examples/nginx/nginx.conf
curl -o nginx/conf.d/shortly.conf https://raw.githubusercontent.com/yourrepo/shortly/main/examples/nginx/conf.d/shortly.conf
curl -o config.yml https://raw.githubusercontent.com/yourrepo/shortly/main/examples/config.yml

# Configure your domain and SSL certificates (see sections below)
# Edit config.yml and nginx/conf.d/shortly.conf

# Start services
docker compose up -d

# Verify installation
curl https://your-domain.com/api/version
```

## Directory Structure

After setup, your installation should look like this:

```
/opt/shortly/
├── docker-compose.yml          # Docker Compose configuration
├── config.yml                  # Shortly application config
├── nginx/
│   ├── nginx.conf             # Main nginx configuration
│   ├── conf.d/
│   │   └── shortly.conf       # Virtual host configuration
│   └── ssl/
│       ├── shortly.example.com.crt    # SSL certificate
│       ├── shortly.example.com.key    # SSL private key
│       └── chain.pem          # Certificate chain (optional)
├── data/
│   └── app.db                 # SQLite database (auto-created)
├── logs/
│   ├── nginx/
│   │   ├── access.log
│   │   ├── error.log
│   │   ├── shortly_access.log
│   │   └── shortly_error.log
│   └── app/
│       └── app.log
```

## Configuration

### 1. Application Configuration

Edit `config.yml` and update the `base-url` to match your domain.

#### Environment Variables (Alternative Configuration Method)

You can override any configuration value using environment variables, which have **higher priority** than `config.yml` values.

**Method 1: Using .env file (Recommended)**

```bash
# Copy the example file
cp .env.example .env

# Edit .env with your values
nano .env
```

Example `.env` file:
```bash
# Override base URL for production
BASE_URL=https://shortly.example.com

# Increase link TTL to 30 days
SHORT_URL_TTL=720

# Set logging level
LOG_LEVEL=info
```

The `docker-compose.yml` automatically loads variables from `.env` file.

**Method 2: Direct environment variables in docker-compose.yml**

Uncomment and set values in the `environment` section of `docker-compose.yml`:
```yaml
environment:
  - BASE_URL=https://shortly.example.com
  - SHORT_URL_TTL=720
  - LOG_LEVEL=info
```

**Available environment variables:**

| Variable | Description | Default |
|----------|-------------|---------|
| `BIND` | Server bind address | `0.0.0.0:8080` |
| `LOG_LEVEL` | Logging level | `info` |
| `LOG_TARGET` | Logging target | `file` |
| `DB_CNN` | Database connection | `sqlite://./data/app.db?mode=rwc` |
| `BASE_URL` | Base URL for short links | From config.yml |
| `SHORT_URL_TTL` | Link TTL in hours | `168` |
| `SCHEDULER_CLEANUP_EXPIRED_URLS` | Cleanup cron | `0 0 0 * * *` |

### 2. nginx Virtual Host Configuration

Edit `nginx/conf.d/shortly.conf`:

**Update server name:**
```nginx
server_name shortly.example.com;  # Change to your domain
```

**Update SSL certificate paths:**
```nginx
ssl_certificate /etc/nginx/ssl/shortly.example.com.crt;
ssl_certificate_key /etc/nginx/ssl/shortly.example.com.key;
```

**Rate limiting zones** (already configured):
- General endpoints: 10 req/s, burst 20
- API endpoints: 5 req/s, burst 10
- URL creation (`/api/url`): 2 req/s, burst 5
- Connection limit: 10 concurrent per IP

### 3. Docker Compose Configuration

The `docker-compose.yml` is pre-configured and typically doesn't need changes. Key settings:

```yaml
services:
  nginx:
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/ssl:/etc/nginx/ssl:ro  # SSL certificates

  app:
    expose:
      - "8080"  # Not exposed to host, only to nginx
```

## Starting Services

### Initial Startup

```bash
cd /opt/shortly

# Pull latest image
docker compose pull

# Start services in detached mode
docker compose up -d

# View logs
docker compose logs -f

# Check service status
docker compose ps
```

Expected output:
```
NAME              IMAGE                    STATUS        PORTS
shortly-nginx     nginx:alpine            Up 10 seconds  0.0.0.0:80->80/tcp, 0.0.0.0:443->443/tcp
shortly-app       tinyops/shortly:0.1.0   Up 12 seconds  8080/tcp
```

### Stopping Services

```bash
# Stop services (containers remain)
docker compose stop

# Stop and remove containers
docker compose down

# Stop and remove containers + volumes (deletes database!)
docker compose down -v
```

### Restarting After Configuration Changes

```bash
# Restart nginx only (for nginx config changes)
docker compose restart nginx

# Restart app only (for config.yml changes)
docker compose restart app

# Reload nginx config without downtime
docker compose exec nginx nginx -s reload
```

### 3. Test Rate Limiting

```bash
# Test URL creation rate limit (2 req/s)
for i in {1..10}; do
  curl -X POST https://shortly.example.com/api/url \
    -H "Content-Type: application/json" \
    -d '{"url":"https://example.com/test'$i'"}' \
    -w "\nStatus: %{http_code}\n"
done

# After 5 requests, you should see HTTP 429 (Too Many Requests)
```

### 4. Test SSL/TLS Configuration

```bash
# Check SSL certificate
openssl s_client -connect shortly.example.com:443 -servername shortly.example.com < /dev/null

# Test SSL with detailed protocol info
curl -Ivs https://shortly.example.com 2>&1 | grep -E "SSL|TLS"
```

### 5. Verify Security Headers

```bash
curl -I https://shortly.example.com

# Should include:
# Strict-Transport-Security: max-age=63072000; includeSubDomains; preload
# X-Frame-Options: SAMEORIGIN
# X-Content-Type-Options: nosniff
# Content-Security-Policy: ...
```

## Security Notes

### Current Security Features

- **IP-based rate limiting** (3 zones with different limits)
- **Connection limiting** (10 concurrent per IP)
- **Security headers** (HSTS, CSP, X-Frame-Options, etc.)
- **TLS 1.2+ only** with modern cipher suites
- **Request size limiting** (1MB max)
- **Non-root container** execution
- **Isolated Docker network**
- **CVE scanning** (clean as of 2025-11-18)
