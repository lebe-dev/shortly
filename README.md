# Shortly

![Shortly Logo](logo.png "Shortly Logo")

URL shortener service.

## Quick start

```bash
docker run --name shortly -p 8080:8080 tinyops/shortly:0.1.0 
```

## Documentation

- Installation
  - [Docker](docs/install/docker.md)
- [Configuration](docs/configuration.md)
- [Development](DEV.md)

## RoadMap

### v0.2.0

- [ ] Locales support: en, ru, es, fr, de, jp

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
