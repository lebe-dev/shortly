# Shortly Helm Chart

Helm chart for deploying Shortly - a URL shortener service.

## Installing the Chart

```bash
helm repo add tinyops https://tinyops.ru/helm-charts/
helm repo update

helm upgrade --install -n shortly --create-namespace shortly tinyops/shortly --version 1.1.4

# with custom values
helm upgrade --install -n shortly --create-namespace shortly tinyops/shortly --version 1.1.4 -f values.yml
```

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
| `config.scheduler.cleanupExpiredUrls` | Cron expression for cleanup task | `0 0 * * *` (daily) | No |
| `config.features.createUrl.enabled` | Enable URL creation feature | `true` | No |
| `config.features.createUrl.authOnly` | Require authentication for URL creation | `true` | No |
| `config.auth.enabled` | Enable authentication | `true` | No |
| `config.auth.type` | Authentication type | `gitlab` | No |
| `config.auth.providers.gitlab.baseUrl` | GitLab instance URL | `https://gitlab.com` | No |
| `config.nginx.enabled` | Enable nginx reverse proxy sidecar | `true` | No |
| `config.nginx.image.repository` | Nginx image repository | `nginxinc/nginx-unprivileged` | No |
| `config.nginx.image.tag` | Nginx image tag | `1.29.3-alpine-otel` | No |
| `config.nginx.image.pullPolicy` | Nginx image pull policy | `IfNotPresent` | No |
| `config.nginx.port` | Nginx container port | `8080` | No |
| `config.nginx.backendPort` | Backend application port | `8081` | No |
| `config.nginx.resources` | Nginx resource requests/limits | `{}` | No |
| `config.nginx.config` | Nginx configuration (nginx.conf) | See values.yaml | No |
| `secrets.dbCnn` | Database connection string | `sqlite:///data/shortly.db?mode=rwc` | **Yes** |
| `secrets.auth.providers.gitlab.applicationId` | GitLab OAuth application ID | `""` | **Yes** |
| `secrets.auth.providers.gitlab.secret` | GitLab OAuth application secret | `""` | **Yes** |
| `secrets.annotations` | Annotations for Secret resource | `{}` | N/A |

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
| `ingress.annotations` | Ingress annotations | Includes static asset caching (nginx) |
| `ingress.hosts` | Ingress hosts configuration | See values.yaml |
| `ingress.tls` | Ingress TLS configuration | `[]` |

**Default Annotations:**

The chart includes default caching annotations for static assets (images, fonts, CSS, JS, SVG) when using nginx ingress controller:

```yaml
nginx.ingress.kubernetes.io/configuration-snippet: |
  location ~* \.(?:jpg|jpeg|gif|png|ico|js|svg|woff|woff2|ttf|css)$ {
    expires max;
    access_log off;
    add_header Cache-Control "public";
  }
```

These annotations:
- Set maximum cache expiration for static files
- Add `Cache-Control: public` header for browser and CDN caching
- Disable access logging for static assets to reduce overhead
- Only work with nginx ingress controller

You can override or remove these annotations in your custom values file if needed.

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
- **Nginx container**: Runs as sidecar, listens on port 8080, serves as reverse proxy
- **Shortly app**: Listens on port 8081 (backend)
- **Service**: Routes external traffic to nginx on port 8080
- **Communication**: Nginx proxies requests to `localhost:8081`

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
| `probes.nginx.livenessProbe.tcpSocket.port` | Port for TCP check | `8080` |
| `probes.nginx.livenessProbe.initialDelaySeconds` | Initial delay before liveness probe starts | `5` |
| `probes.nginx.livenessProbe.periodSeconds` | How often to perform the probe | `30` |
| `probes.nginx.livenessProbe.timeoutSeconds` | Timeout for probe response | `3` |
| `probes.nginx.livenessProbe.failureThreshold` | Failures before restart | `3` |
| `probes.nginx.readinessProbe.enabled` | Enable readiness probe for nginx container | `true` |
| `probes.nginx.readinessProbe.tcpSocket.port` | Port for TCP socket check | `8080` |
| `probes.nginx.readinessProbe.initialDelaySeconds` | Initial delay before readiness probe starts | `2` |
| `probes.nginx.readinessProbe.periodSeconds` | How often to perform the probe | `10` |
| `probes.nginx.readinessProbe.timeoutSeconds` | Timeout for probe response | `3` |
| `probes.nginx.readinessProbe.failureThreshold` | Failures before marking unready | `3` |
| `probes.nginx.startupProbe.enabled` | Enable startup probe for nginx container | `true` |
| `probes.nginx.startupProbe.tcpSocket.port` | Port for TCP check | `8080` |
| `probes.nginx.startupProbe.initialDelaySeconds` | Initial delay before startup probe starts | `0` |
| `probes.nginx.startupProbe.periodSeconds` | How often to perform the probe | `1` |
| `probes.nginx.startupProbe.timeoutSeconds` | Timeout for probe response | `3` |
| `probes.nginx.startupProbe.failureThreshold` | Failures before container fails to start | `10` |

**Probe Behavior:**
- **Startup Probe**: Gives the application time to initialize. Other probes don't run until startup succeeds.
- **Liveness Probe**: Restarts the container if it fails. Used to detect deadlocks or hung processes.
- **Readiness Probe**: Removes the pod from service if it fails. Used to temporarily stop traffic during high load or temporary issues.

**Shortly Container**: All probes use HTTP GET to `/api/health`, which verifies database connectivity.

**Nginx Container**: Both liveness and readiness probes use TCP socket check on port 8080 (lightweight connection verification). HTTP health checks are not used for nginx probes due to security restrictions that block external access to `/api/health`.

## Example Configurations

### Minimal Production Setup

```yaml
replicaCount: 1

image:
  tag: "0.1.0"

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
