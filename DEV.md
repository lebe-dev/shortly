# Development Setup

## Nginx Configuration for Dev Mode with Hot Reload

Set port for backend to `18080` in `config.yml`.

Put `127.0.0.1 shortly.dev` in your `/etc/hosts` file.

Then create nginx config:

```nginx
server {
    listen       443 ssl;
    server_name  shortly.dev;

    ssl_certificate      /opt/homebrew/etc/nginx/tls/shortly.dev.crt;
    ssl_certificate_key  /opt/homebrew/etc/nginx/tls/shortly.dev.key;
    ssl_session_cache    shared:SSL:1m;
    ssl_session_timeout  5m;
    ssl_ciphers  HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers  on;

    # Special paths for dev server (Vite HMR and SvelteKit routes)
    location ~ ^/(login|$|_app|node_modules|@fs|@vite|@id) {
        proxy_set_header Host $host;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_pass http://localhost:4200;
    }

    # API requests always go to backend
    location /api {
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header Host $host;
        proxy_pass http://localhost:18080;
    }

    # Static files (with extensions) go to dev server
    location ~ \. {
        proxy_set_header Host $host;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_pass http://localhost:4200;
    }

    # Everything else (potential short links) goes to backend for redirect
    location / {
        proxy_set_header Host $host;
        proxy_pass http://localhost:18080;
    }
}
```

## Backend tests

Run the backend unit tests without Docker:

```bash
cargo test --locked --lib --bin server
```

PostgreSQL integration tests require a running Docker daemon accessible to
Testcontainers. On the first run Docker downloads `postgres:17.6-alpine`.

```bash
cargo test --locked --features postgres-integration --test postgres -- --test-threads=4
```

Each test starts its own disposable PostgreSQL container on a dynamically assigned
port, applies the SQL files from `migrations/postgres`, and removes the container
afterwards. No external database or application configuration is needed. The suite
covers migration replay and rollback, URL storage and cleanup, quotas, audit
filtering and pagination, users, sessions, and passkey persistence, including
concurrent challenge consumption. Passkey tests exercise database storage, not a
WebAuthn browser ceremony. Docker failures fail the tests rather than skipping them.

The `postgres-integration` feature enables the integration test target; it is
excluded from ordinary `cargo test` runs. To run all backend tests including
PostgreSQL, use `cargo test --locked --features postgres-integration`.

## Corporate link integration tests (Playwright)

```bash
cd frontend && yarn install --frozen-lockfile
cd .. && just test-integration
```

Alternatively, run `yarn test:integration` from `frontend`. Requires Node.js 22+
and the Rust toolchain. Browser installation, Docker, a frontend build, a running
application and a real GitLab account are not required. These are HTTP/API tests;
browser UI testing remains separate. The suite uses Playwright's
[APIRequestContext](https://playwright.dev/docs/api-testing).

The global setup builds the current backend with `cargo build --locked`. Each test
starts its own server, temporary SQLite database with real migrations, and local
GitLab OAuth stub; login creates real application sessions. Configuration and
process environment are isolated from the developer's `.env` and `config.yml`.
Temporary processes and databases are removed after each test, including failures.
Playwright artifacts and server logs on failures go to `target/playwright/results`.

Two local HTTP proxies represent corporate and external clients. They append
fixed documentation-only addresses to `X-Forwarded-For`; these addresses are never
bound to a network interface or contacted. All TCP connections use `127.0.0.1`
and OS-assigned ports, so tests do not depend on Wi-Fi, VPN, LAN subnets, DNS,
`/etc/hosts` or fixed available ports. Separate scenarios disable proxy trust and
verify the actual loopback TCP peer. Forwarding-chain cases also cover IPv6,
mapped addresses, spoofing, malformed headers and multiple trusted hops.

The suite checks creation defaults and overrides, access by name/case/ID,
GET/HEAD and redirects, absence of destination leaks, cache prevention,
owner/admin restrictions, denied-visit tracking, public generated links,
persistence across restart and CIDR revocation. SQLite exercises the complete
HTTP stack; PostgreSQL storage is covered by the separate Testcontainers suite
above. Run a subset with `yarn test:integration --grep 'forwarding'`.

## Lint and security checks

`just lint` runs rustfmt in check mode, Clippy (warnings are errors), frontend
Prettier/ESLint, Svelte type checking, and Helm lint. It does not run UI tests.
Install the frontend dependencies with `cd frontend && yarn install --frozen-lockfile`
first. Rust with rustfmt and Clippy, Yarn and Helm must be available in PATH.

`just security` runs the following checks in order and stops on the first failure:

- `security-osv`: OSV-Scanner v2 checks `Cargo.lock` and `frontend/yarn.lock`.
- `security-gitleaks`: Gitleaks v8 checks all local Git history and the working
  directory, with secrets redacted. Generated artifacts are excluded via
  `.gitleaks.toml`; local configuration files remain covered.
- `security-semgrep`: Semgrep CE runs the registry's `p/security-audit` rules
  with telemetry disabled. Coverage depends on the language and available rules;
  this is not a complete audit of Rust or Svelte code.
- `security-trivy-config`: Trivy checks Docker, Compose, Helm and Kubernetes
  configuration. Any reported misconfiguration fails the check.
- `security-trivy-image`: Trivy checks the local Docker image matching the
  version in `Cargo.toml`; HIGH and CRITICAL vulnerabilities fail the check.
  Build it first with `docker build -t tinyops/shortly:$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml) .`.
  `just trivy` remains an alias for this check.

Install `osv-scanner` (v2), `gitleaks` (v8), `semgrep` and `trivy` in PATH before
running security checks. On macOS: `brew install osv-scanner gitleaks semgrep trivy`.
Scanners need network access for vulnerability databases and Semgrep rules;
source scanning runs locally and does not require a Semgrep account.
Each check can also be called separately with `just <recipe>`.
Findings produce a nonzero exit code; no baseline or vulnerability suppression
is configured. OSV and Semgrep fail on findings at every severity.
