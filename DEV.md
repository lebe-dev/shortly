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
