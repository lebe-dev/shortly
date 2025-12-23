# Shortly

![Shortly Logo](logo.png "Shortly Logo")

URL shortener service.

## Quick start

```bash
docker run --name shortly -p 8080:8080 tinyops/shortly:1.0.0 
```

## Features

- BLAZING FAST 🌝 (Svelte+Rust)
- Locales support: en, ru, es, fr, de, jp, ge
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
- [Development](DEV.md)

## RoadMap

### v0.3.0

- [ ] Security:
  - [ ] URL filters support

### v0.4.0

- [ ] Security:
  - [ ] Add captcha support

### v1.0.0

- [ ] QRCode generation
  - [ ] Backend: QRCode generation
  - [ ] Frontend: Show QRCode result

### v1.1.0

- [ ] Backend: Public API

### v1.2.0

- [ ] Custom-named links

### v1.3.0

- [ ] Account management
  - [ ] Account limits
  
### v1.4.0

- [ ] Chrome extension
  - [ ] Click: create short URL from current page
