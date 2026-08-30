# Shortly

![Shortly Logo](logo.png "Shortly Logo")

URL shortener service.

## Quick start

```bash
docker run --name shortly -p 8080:8080 tinyops/shortly:1.5.0 
```

## Features

- BLAZING FAST 🌝 (Svelte+Rust)
- **Custom named URLs** - create memorable short links like `/onboarding` (requires authentication)
- **OAuth Authentication** - GitLab OAuth support with session management
- **Rate limiting** - per-user total and daily URL limits, set by an administrator
- Database support: SQLite, PostgreSQL
- Locales support: en, ru, es, fr, de, jp, ge, zh, he
- Dark theme support
- Low resource usage:
  ```bash
  CONTAINER ID   NAME                          CPU %     MEM USAGE / LIMIT     MEM %     NET I/O           BLOCK I/O         PIDS
  c63264ba615e   shortly                       0.03%     4.98MiB / 1.921GiB    0.25%     207kB / 1.35MB    106kB / 295kB     3
  ```

## Documentation

- Installation
  - [Docker](docs/install/docker.md)
- [Configuration](docs/configuration/configuration.md)
  - [Add locale](docs/locale.md)
- [Monitoring](docs/monitoring.md)
- [Development](DEV.md)

## RoadMap

- Backend: Public API
- Chrome extension: click: create short URL from current page
- Security: URL filters support
- Security: Add captcha support
- QRCode generation
