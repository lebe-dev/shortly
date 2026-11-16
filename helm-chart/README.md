# Shortly Helm Chart

Helm chart for deploying Shortly - a URL shortener service.

## Installing the Chart

```bash
helm repo add tinyops https://tinyops.ru/helm-charts/
helm repo update

helm upgrade --install -n shortly --create-namespace shortly tinyops/shortly --version 1.1.8

# with custom values
helm upgrade --install -n shortly --create-namespace shortly tinyops/shortly --version 1.1.8 -f values.yml
```

Для установки из локального исходного кода:

```bash
helm upgrade --install -n shortly --create-namespace shortly ./helm-chart -f values.yml
```

Публикация чарта: `just release-chart` (обновляет Helm-репозиторий).

## Uninstalling the Chart

```bash
helm uninstall -n shortly shortly
```

## Configuration

### Application Configuration

The following parameters configure the Shortly application itself. All values are exposed as environment variables to the container.

| Parameter | Description | Default | Sensitive |
|-----------|-------------|---------|-----------|
| `config.bind` | Server bind address and port | `0.0.0.0:8080` | No |
| `config.logLevel` | Logging level (debug, info, warn, error) | `info` | No |
| `config.logTarget` | Logging target (stdout, file) | `stdout` | No |
| `config.baseUrl` | Base URL for generated short links | `http://localhost:8080` | No |
| `config.shortUrl.ttl` | Link TTL in hours | `168` (7 days) | No |
| `config.shortUrl.maxLength` | Maximum URL length in characters | `2048` | No |
| `config.scheduler.cleanupExpiredUrls` | Cron expression for cleanup task | `0 0 * * * *` (hourly) | No |
| `config.features.createUrl.enabled` | Enable URL creation feature | `true` | No |
| `config.features.createUrl.authOnly` | Require authentication for URL creation | `true` | No |
| `config.features.createUrl.maxPerUser` | Fallback total URL quota per user | `100` | No |
| `config.features.createUrl.maxPerDay` | Fallback URL quota per user in the last 24 hours | `10` | No |
| `config.features.namedUrls.enabled` | Enable authenticated creation of named links | `false` | No |
| `config.features.namedUrls.minLength` | Minimum custom name length | `3` | No |
| `config.features.namedUrls.maxLength` | Maximum custom name length | `50` | No |
| `config.features.namedUrls.defaultAccess` | `NAMED_URL_DEFAULT_ACCESS`: режим новых именных ссылок, `public` или `internal` | `public` | No |
| `config.features.namedUrls.allowAccessOverride` | `NAMED_URL_ALLOW_ACCESS_OVERRIDE`: разрешить выбор режима в UI/API | `false` | No |
| `config.features.namedUrls.checkRateLimit` | `NAMED_URL_CHECK_RATE_LIMIT`: проверки имён и попытки создания именных ссылок на источник в минуту, отдельно на каждой реплике | `30` | No |
| `config.internalNetworkCidrs` | `INTERNAL_NETWORK_CIDRS`: разрешённые клиентские IPv4/IPv6 CIDR через запятую | `""` | No |
| `config.trustedProxyCidrs` | `TRUSTED_PROXY_CIDRS`: доверенные прокси для `X-Forwarded-For`, CIDR через запятую | `""` | No |
| `config.auth.enabled` | Enable authentication | `true` | No |
| `config.auth.note` | Note shown on the login page | `""` | No |
| `config.auth.adminUsers` | Comma-separated administrator usernames | `""` | No |
| `config.metrics.enabled` | Enable `/api/metrics` | `false` | No |
| `config.auth.type` | Authentication type | `gitlab` | No |
| `config.auth.providers.gitlab.baseUrl` | GitLab instance URL | `https://gitlab.com` | No |
| `config.auth.passkey.enabled` | Enable login with a passkey (WebAuthn) | `false` | No |
| `config.auth.passkey.rpId` | Registrable domain, without scheme or port. Required when passkeys are enabled | `""` | No |
| `config.auth.passkey.rpOrigin` | Origin the browser reports, including scheme and port. Required when passkeys are enabled | `""` | No |
| `config.auth.passkey.rpName` | Name shown by the authenticator | `Shortly` | No |
| `config.auth.passkey.challengeTtl` | Lifetime of an unfinished ceremony, in seconds | `300` | No |
| `config.nginx.enabled` | Enable nginx reverse proxy sidecar | `true` | No |
| `config.nginx.image.repository` | Nginx image repository | `nginx` | No |
| `config.nginx.image.tag` | Nginx image tag | `1.30.0-alpine3.23-slim` | No |
| `config.nginx.image.pullPolicy` | Nginx image pull policy | `IfNotPresent` | No |
| `config.nginx.port` | Nginx container port | `80` | No |
| `config.nginx.backendPort` | Backend application port | `8080` | No |
| `config.nginx.resources` | Nginx resource requests/limits | `{}` | No |
| `config.nginx.config` | Nginx configuration (nginx.conf) | See values.yaml | No |
| `secrets.dbCnn` | Database connection string | `sqlite:///data/shortly.db?mode=rwc` | **Yes** |
| `secrets.auth.providers.gitlab.applicationId` | GitLab OAuth application ID | `""` | **Yes** |
| `secrets.auth.providers.gitlab.secret` | GitLab OAuth application secret | `""` | **Yes** |
| `secrets.annotations` | Annotations for Secret resource | `{}` | N/A |

### Доступность именных ссылок

Старые и обычные сгенерированные ссылки остаются публичными. `defaultAccess` применяется только к новым именным ссылкам; пользователь может изменить его при создании, только если включён `allowAccessOverride`. Для `internal` или разрешения выбора режима необходимо заполнить `internalNetworkCidrs`, иначе приложение не запустится. Диапазоны по умолчанию пустые: укажите реальные адреса компании/VPN, включая публичные адреса выхода при необходимости.

Пример `values.yml` для цепочки Ingress → Nginx sidecar → Shortly:

```yaml
config:
  features:
    namedUrls:
      enabled: true
      defaultAccess: internal
      allowAccessOverride: false
      checkRateLimit: 30
  internalNetworkCidrs: "10.20.0.0/16,fd12:3456::/32"
  trustedProxyCidrs: "127.0.0.1/32,10.244.2.0/24"
```

CIDR в примере нужно заменить: `10.20.0.0/16` и `fd12:3456::/32` — клиентские сети, `10.244.2.0/24` — пример подсети ingress-подов. С Nginx sidecar добавьте его loopback `127.0.0.1/32`; без sidecar укажите только реальные прокси перед backend. Клиентские сети и доверенные прокси задаются отдельно: не доверяйте всей корпоративной сети или всем адресам (`0.0.0.0/0`, `::/0`) как прокси.

Backend определяет клиента по цепочке `X-Forwarded-For` справа налево, пропуская доверенные прокси. Ingress должен передавать достоверный IP клиента, а доступ к backend должен идти через заданные прокси. При недостоверном IP internal-ссылка недоступна. Снаружи она выглядит как отсутствующая; авторизация владельца или администратора не обходит проверку при переходе.

Лимит проверки имён общий с попытками создания именных ссылок и действует отдельно на каждой реплике, сбрасывается после перезапуска. Ответ при превышении — `429` с `Retry-After`. Для общего лимита нескольких реплик задайте дополнительное ограничение на ingress.

Изменение ConfigMap при `helm upgrade` перезапускает pod через `checksum/config`, применяя новые ENV. Прокси должны сохранять `Cache-Control: private, no-store` для редиректов и API получения ссылок.

Подробности: [переменные окружения](../docs/configuration/env.md#named-link-availability).

### Image Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `image.repository` | Container image repository | `tinyops/shortly` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `image.tag` | Image tag (overrides chart appVersion) | `""` |
| `imagePullSecrets` | Image pull secrets | `[]` |

### Deployment Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of replicas | `1` |
| `nameOverride` | Override chart name | `""` |
| `fullnameOverride` | Override full name | `""` |
| `podAnnotations` | Annotations for pods | `{}` |
| `podLabels` | Labels for pods | `{}` |
| `podSecurityContext` | Security context for pod | `{}` |
| `securityContext` | Security context for container | `{}` |
| `resources` | Resource requests/limits | `{}` |
| `nodeSelector` | Node selector | `{}` |
| `tolerations` | Tolerations | `[]` |
| `affinity` | Affinity rules | `{}` |

### Service Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `service.type` | Service type | `ClusterIP` |
| `service.port` | Service port | `80` |

### Service Account

| Parameter | Description | Default |
|-----------|-------------|---------|
| `serviceAccount.create` | Create service account | `true` |
| `serviceAccount.automount` | Automount service account token | `true` |
| `serviceAccount.annotations` | Service account annotations | `{}` |
| `serviceAccount.name` | Service account name | `""` |

### Ingress Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `ingress.enabled` | Enable ingress | `false` |
| `ingress.className` | Ingress class name | `""` |
| `ingress.annotations` | Ingress annotations | `{}` |
| `ingress.hosts` | Ingress hosts configuration | See values.yaml |
| `ingress.tls` | Ingress TLS configuration | `[]` |

По умолчанию ingress-аннотации пустые. Кеширование статических файлов настроено в Nginx sidecar.

### HTTPRoute Configuration (Gateway API)

| Parameter | Description | Default |
|-----------|-------------|---------|
| `httpRoute.enabled` | Enable HTTPRoute | `false` |
| `httpRoute.annotations` | HTTPRoute annotations | `{}` |
| `httpRoute.parentRefs` | Gateway references | See values.yaml |
| `httpRoute.hostnames` | Hostnames | `[chart-example.local]` |
| `httpRoute.rules` | Routing rules | See values.yaml |

### Persistence Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `persistence.enabled` | Enable persistent storage for SQLite database | `true` |
| `persistence.existingClaim` | Use existing PVC (if empty, creates new) | `""` |
| `persistence.storageClass` | Storage class name (use "-" for default) | `""` |
| `persistence.accessModes` | Access modes | `[ReadWriteOnce]` |
| `persistence.size` | Volume size | `1Gi` |
| `persistence.annotations` | PVC annotations | `{}` |
| `persistence.selector` | Selector to match existing PV | `{}` |
| `persistence.mountPath` | Mount path in container | `/data` |
| `persistence.subPath` | Subpath within volume | `""` |

**Note:** SQLite database requires persistent storage in production. The database file is stored at `/data/shortly.db` inside the container.

### Nginx Reverse Proxy

The chart includes an optional nginx reverse proxy sidecar container that provides:

- **Client-side caching** for SvelteKit static assets (`/_app/immutable/*` paths)
- **Gzip compression** at optimal level (6) for text-based content
- **Performance optimization** with appropriate cache headers

**Architecture:**

When nginx is enabled (default):
- **Nginx container**: Runs as sidecar, listens on port 80, serves as reverse proxy
- **Shortly app**: Listens on port 8080 (backend)
- **Service**: Routes external traffic to nginx on port 80
- **Communication**: Nginx proxies requests to `localhost:8080`

**Caching Strategy:**

1. **SvelteKit immutable assets** (`/_app/immutable/*`):
   - Cache-Control: `public, immutable`
   - Expires: `max` (1 year)
   - Access logging: disabled
   - Perfect for content-addressed assets that never change

2. **Other static assets** (images, fonts, etc.):
   - Cache-Control: `public`
   - Expires: 7 days
   - Access logging: disabled

3. **Dynamic content**:
   - No caching
   - Proxied directly to backend

**Gzip Compression:**

- Compression level: 6 (optimal balance between size and CPU)
- Compressed types: text/plain, text/css, text/javascript, application/json, application/javascript, SVG, fonts
- Automatically adds `Vary: Accept-Encoding` header

**Configuration:**

| Parameter | Description | Default |
|-----------|-------------|---------|
| `config.nginx.enabled` | Enable/disable nginx sidecar | `true` |
| `config.nginx.config` | Full nginx.conf content | See values.yaml |
| `config.nginx.resources` | Resource limits for nginx | `{}` |

**Disabling Nginx:**

To disable nginx and run the application directly (legacy mode):

```yaml
config:
  nginx:
    enabled: false
```

When disabled, the Shortly app listens directly on port 8080.

**Custom Nginx Configuration:**

You can override the entire nginx configuration in your values file:

```yaml
config:
  nginx:
    enabled: true
    config: |
      worker_processes auto;
      error_log /dev/stderr warn;
      # ... your custom nginx.conf ...
```

**Resource Allocation Example:**

```yaml
config:
  nginx:
    enabled: true
    resources:
      requests:
        cpu: 50m
        memory: 32Mi
      limits:
        cpu: 100m
        memory: 64Mi
```

### Additional Volumes and Volume Mounts

| Parameter | Description | Default |
|-----------|-------------|---------|
| `volumes` | Additional volumes | `[]` |
| `volumeMounts` | Additional volume mounts | `[]` |

### Probe Configuration

Control Kubernetes liveness, readiness, and startup probes for both containers.

| Parameter | Description | Default |
|-----------|-------------|---------|
| `probes.shortly.livenessProbe.enabled` | Enable liveness probe for shortly container | `true` |
| `probes.shortly.livenessProbe.httpGet.path` | Health check path | `/api/health` |
| `probes.shortly.livenessProbe.initialDelaySeconds` | Initial delay before liveness probe starts | `10` |
| `probes.shortly.livenessProbe.periodSeconds` | How often to perform the probe | `30` |
| `probes.shortly.livenessProbe.timeoutSeconds` | Timeout for probe response | `5` |
| `probes.shortly.livenessProbe.failureThreshold` | Failures before restart | `3` |
| `probes.shortly.readinessProbe.enabled` | Enable readiness probe for shortly container | `true` |
| `probes.shortly.readinessProbe.httpGet.path` | Health check path | `/api/health` |
| `probes.shortly.readinessProbe.initialDelaySeconds` | Initial delay before readiness probe starts | `5` |
| `probes.shortly.readinessProbe.periodSeconds` | How often to perform the probe | `10` |
| `probes.shortly.readinessProbe.timeoutSeconds` | Timeout for probe response | `5` |
| `probes.shortly.readinessProbe.failureThreshold` | Failures before marking unready | `3` |
| `probes.shortly.startupProbe.enabled` | Enable startup probe for shortly container | `true` |
| `probes.shortly.startupProbe.httpGet.path` | Health check path | `/api/health` |
| `probes.shortly.startupProbe.initialDelaySeconds` | Initial delay before startup probe starts | `0` |
| `probes.shortly.startupProbe.periodSeconds` | How often to perform the probe | `2` |
| `probes.shortly.startupProbe.timeoutSeconds` | Timeout for probe response | `5` |
| `probes.shortly.startupProbe.failureThreshold` | Failures before container fails to start | `15` |
| `probes.nginx.livenessProbe.enabled` | Enable liveness probe for nginx container | `true` |
| `probes.nginx.livenessProbe.tcpSocket.port` | Port for TCP check | `80` |
| `probes.nginx.livenessProbe.initialDelaySeconds` | Initial delay before liveness probe starts | `5` |
| `probes.nginx.livenessProbe.periodSeconds` | How often to perform the probe | `30` |
| `probes.nginx.livenessProbe.timeoutSeconds` | Timeout for probe response | `3` |
| `probes.nginx.livenessProbe.failureThreshold` | Failures before restart | `3` |
| `probes.nginx.readinessProbe.enabled` | Enable readiness probe for nginx container | `true` |
| `probes.nginx.readinessProbe.tcpSocket.port` | Port for TCP socket check | `80` |
| `probes.nginx.readinessProbe.initialDelaySeconds` | Initial delay before readiness probe starts | `2` |
| `probes.nginx.readinessProbe.periodSeconds` | How often to perform the probe | `10` |
| `probes.nginx.readinessProbe.timeoutSeconds` | Timeout for probe response | `3` |
| `probes.nginx.readinessProbe.failureThreshold` | Failures before marking unready | `3` |
| `probes.nginx.startupProbe.enabled` | Enable startup probe for nginx container | `true` |
| `probes.nginx.startupProbe.tcpSocket.port` | Port for TCP check | `80` |
| `probes.nginx.startupProbe.initialDelaySeconds` | Initial delay before startup probe starts | `0` |
| `probes.nginx.startupProbe.periodSeconds` | How often to perform the probe | `1` |
| `probes.nginx.startupProbe.timeoutSeconds` | Timeout for probe response | `3` |
| `probes.nginx.startupProbe.failureThreshold` | Failures before container fails to start | `10` |

**Probe Behavior:**
- **Startup Probe**: Gives the application time to initialize. Other probes don't run until startup succeeds.
- **Liveness Probe**: Restarts the container if it fails. Used to detect deadlocks or hung processes.
- **Readiness Probe**: Removes the pod from service if it fails. Used to temporarily stop traffic during high load or temporary issues.

**Shortly Container**: All probes use HTTP GET to `/api/health`, which verifies database connectivity.

**Nginx Container**: Both liveness and readiness probes use TCP socket check on port 80 (lightweight connection verification). HTTP health checks are not used for nginx probes due to security restrictions that block external access to `/api/health`.

## Example Configurations

### Minimal Production Setup

```yaml
replicaCount: 1

image:
  tag: "1.6.0"

config:
  baseUrl: "https://go.company.com"
  logLevel: "info"
  features:
    createUrl:
      authOnly: true
  auth:
    enabled: true
    providers:
      gitlab:
        baseUrl: "https://gitlab.company.com"

secrets:
  auth:
    providers:
      gitlab:
        applicationId: "your-oauth-app-id"
        secret: "your-oauth-secret"

persistence:
  enabled: true
  size: 5Gi
  storageClass: "csi-nfs"

ingress:
  enabled: true
  className: "nginx"
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
  hosts:
    - host: go.company.com
      paths:
        - path: /
          pathType: Prefix
  tls:
    - secretName: shortly-tls
      hosts:
        - go.company.com

resources:
  requests:
    cpu: 100m
    memory: 128Mi
  limits:
    cpu: 500m
    memory: 512Mi
```

### Setup with External Secrets

```yaml
config:
  baseUrl: "http://localhost:8080"
  logLevel: "debug"
  auth:
    enabled: false
  features:
    createUrl:
      authOnly: false

secrets:
  # Example: Using Vault annotations for secret injection
  annotations:
    vault.security.banzaicloud.io/vault-addr: "https://vault.company.com"
    vault.security.banzaicloud.io/vault-role: "shortly"
    vault.security.banzaicloud.io/vault-path: "kubernetes_internal"
  dbCnn: "vault:secret_v2/data/infra/shortly#DB_CNN"
  auth:
    providers:
      gitlab:
        applicationId: "vault:secret_v2/data/infra/shortly#APPLICATION_ID"
        secret: "vault:secret_v2/data/infra/shortly#SECRET"

persistence:
  enabled: false

service:
  type: NodePort
```

## License

This Helm chart is provided as-is for deploying the Shortly application.
