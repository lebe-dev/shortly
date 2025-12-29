# Monitoring

Shortly provides comprehensive monitoring capabilities through health check and metrics endpoints.

## Health Check Endpoint

The health check endpoint monitors service availability and database connectivity.

### Endpoint Details

**URL:** `/api/health`  
**Method:** `GET`  
**Authentication:** Not required (public endpoint)

### Response Format

#### Healthy Response

**Status Code:** `200 OK`

```json
{
  "status": "healthy",
  "database": "ok"
}
```

#### Unhealthy Response

**Status Code:** `500 Internal Server Error`

```json
{
  "status": "unhealthy",
  "database": "error"
}
```

### Health Check Mechanism

The endpoint performs the following checks:

1. **Database Connectivity**: Executes `SELECT COUNT(*) FROM _migrations` to verify:
   - SQLite database file is accessible
   - Database connection pool is working
   - Migration system is properly initialized

If any check fails, the endpoint returns a 500 error with details logged to the application logs.

### Usage Examples

#### Using curl

```bash
# Check health status
curl http://localhost:8080/api/health

# Check with status code
curl -w "\nHTTP Status: %{http_code}\n" http://localhost:8080/api/health
```

#### Using wget

```bash
wget -q -O- http://localhost:8080/api/health
```

#### Using HTTPie

```bash
http http://localhost:8080/api/health
```

## Kubernetes Integration

The health endpoint is designed for Kubernetes liveness and readiness probes.

### Liveness Probe

Detects if the application is in a broken state and needs to be restarted:

```yaml
livenessProbe:
  httpGet:
    path: /api/health
    port: 8080
  initialDelaySeconds: 10
  periodSeconds: 30
  timeoutSeconds: 5
  failureThreshold: 3
```

### Readiness Probe

Determines if the application is ready to receive traffic:

```yaml
readinessProbe:
  httpGet:
    path: /api/health
    port: 8080
  initialDelaySeconds: 5
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3
```

### Startup Probe

Gives the application time to initialize before liveness/readiness probes start:

```yaml
startupProbe:
  httpGet:
    path: /api/health
    port: 8080
  initialDelaySeconds: 0
  periodSeconds: 2
  timeoutSeconds: 5
  failureThreshold: 15  # 30 seconds total
```

See [Helm Chart Configuration](../helm-chart/README.md#probe-configuration) for complete probe settings.

## Monitoring Best Practices

### Response Time Monitoring

Monitor the `/api/health` endpoint response time to detect performance degradation:

- **Normal:** < 50ms
- **Warning:** 50-200ms
- **Critical:** > 200ms

### Availability Monitoring

Set up external monitoring to track endpoint availability:

- **Uptime checks:** Every 1-5 minutes
- **Alert threshold:** 2+ consecutive failures
- **Timeout:** 5 seconds

### Database Health

The health endpoint indirectly monitors:

- SQLite file system availability
- Database file corruption
- Migration table integrity

If the database becomes unavailable, the endpoint will return 500 errors, triggering Kubernetes pod restarts.

## Logging

Health check failures are logged with ERROR level:

```
ERROR Health check failed - database error: SqlxError(...)
```

Successful health checks generate DEBUG level logs (not logged by default to reduce noise).

## Performance Considerations

The health check is designed to be lightweight:

- **Query:** Simple COUNT query on migrations table (typically < 10 rows)
- **Connection:** Uses existing connection pool (no new connections)
- **Overhead:** < 1ms per check on typical hardware
- **Frequency:** Default probes run ~10 times per minute per pod

With WAL mode enabled on SQLite, health checks don't block writes.

## Metrics Endpoint

Shortly exposes application metrics in Prometheus format for monitoring and observability.

### Endpoint Details

**URL:** `/api/metrics`  
**Method:** `GET`  
**Authentication:** Not required (public endpoint)  
**Format:** Prometheus text format (version 0.0.4)

### Available Metrics

#### URL Metrics

| Metric Name | Type | Description |
|------------|------|-------------|
| `shortly_urls_total` | Gauge | Total number of active URLs in the system |
| `shortly_urls_last_created_timestamp` | Gauge | Unix timestamp of the most recently created URL |
| `shortly_urls_custom_named_total` | Gauge | Number of URLs with custom names |
| `shortly_urls_expired_total` | Gauge | Number of expired URLs (where TTL has passed) |
| `shortly_urls_created_last_24h` | Gauge | URLs created in the last 24 hours |
| `shortly_urls_deleted_last_24h` | Gauge | URLs deleted in the last 24 hours |
| `shortly_urls_ttl_hours` | Histogram | Distribution of URL TTL values in hours |

**TTL Histogram Buckets:** 1h, 6h, 12h, 24h, 48h, 72h, 168h (1 week), 336h (2 weeks), 720h (1 month)

#### User Metrics

| Metric Name | Type | Description |
|------------|------|-------------|
| `shortly_users_total` | Gauge | Total number of registered users |
| `shortly_users_active_sessions` | Gauge | Number of active user sessions |
| `shortly_users_last_login_timestamp` | Gauge | Unix timestamp of the last user login |

**Note:** User metrics are only available when authentication is enabled in the configuration.

#### Audit Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `shortly_audit_events_total` | Counter | `event_type` | Total count of audit events by type |
| `shortly_audit_last_event_timestamp` | Gauge | `event_type` | Unix timestamp of last event for each type |

**Event Types:**
- `CreateUrl` - URL creation events
- `DeleteUrl` - URL deletion events
- `UserLogin` - User login events
- `UserLogout` - User logout events
- `UserQuotaUpdate` - User quota modification events

#### Database Metrics

| Metric Name | Type | Description |
|------------|------|-------------|
| `shortly_database_connection_pool_size` | Gauge | Total database connections in the pool |
| `shortly_database_connection_pool_idle` | Gauge | Idle database connections |

#### System Metrics

| Metric Name | Type | Labels | Description |
|------------|------|--------|-------------|
| `shortly_uptime_seconds` | Gauge | - | Application uptime in seconds since startup |
| `shortly_version_info` | Gauge | `version` | Application version (value is always 1) |

### Example Output

```prometheus
# HELP shortly_urls_total Total number of active URLs
# TYPE shortly_urls_total gauge
shortly_urls_total 1523

# HELP shortly_urls_last_created_timestamp Unix timestamp of last created URL
# TYPE shortly_urls_last_created_timestamp gauge
shortly_urls_last_created_timestamp 1735470123

# HELP shortly_urls_ttl_hours Distribution of URL TTL in hours
# TYPE shortly_urls_ttl_hours histogram
shortly_urls_ttl_hours_bucket{le="1"} 45
shortly_urls_ttl_hours_bucket{le="6"} 123
shortly_urls_ttl_hours_bucket{le="168"} 1211
shortly_urls_ttl_hours_bucket{le="+Inf"} 1523
shortly_urls_ttl_hours_sum 253467.5
shortly_urls_ttl_hours_count 1523

# HELP shortly_audit_events_total Total audit events by type
# TYPE shortly_audit_events_total counter
shortly_audit_events_total{event_type="CreateUrl"} 1523
shortly_audit_events_total{event_type="DeleteUrl"} 312

# HELP shortly_version_info Application version information
# TYPE shortly_version_info gauge
shortly_version_info{version="1.3.0"} 1
```

### Usage Examples

#### Using curl

```bash
# Fetch all metrics
curl http://localhost:8080/api/metrics

# Count total metrics
curl -s http://localhost:8080/api/metrics | grep "^shortly_" | wc -l

# Filter specific metric
curl -s http://localhost:8080/api/metrics | grep "shortly_urls_total"
```

#### Using wget

```bash
wget -q -O- http://localhost:8080/api/metrics
```

## Prometheus Integration

### Scrape Configuration

Add the following to your Prometheus `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'shortly'
    scrape_interval: 30s
    scrape_timeout: 10s
    metrics_path: /api/metrics
    static_configs:
      - targets: ['localhost:8080']
        labels:
          environment: 'production'
          app: 'shortly'
```

### Kubernetes ServiceMonitor

For Prometheus Operator in Kubernetes:

```yaml
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: shortly
  namespace: monitoring
spec:
  selector:
    matchLabels:
      app: shortly
  endpoints:
    - port: http
      path: /api/metrics
      interval: 30s
      scrapeTimeout: 10s
```

### Grafana Dashboard Queries

Example PromQL queries for Grafana dashboards:

```promql
# Total URLs
shortly_urls_total

# URL creation rate (per hour)
rate(shortly_audit_events_total{event_type="CreateUrl"}[1h]) * 3600

# Active users
shortly_users_total

# Database connection pool utilization
shortly_database_connection_pool_size - shortly_database_connection_pool_idle

# Uptime in days
shortly_uptime_seconds / 86400

# URLs created in last 24h
shortly_urls_created_last_24h

# 95th percentile TTL
histogram_quantile(0.95, shortly_urls_ttl_hours_bucket)
```

### Alerting Rules

Example Prometheus alerting rules:

```yaml
groups:
  - name: shortly_alerts
    interval: 30s
    rules:
      - alert: HighExpiredURLs
        expr: shortly_urls_expired_total > 1000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High number of expired URLs"
          description: "{{ $value }} URLs have expired"

      - alert: DatabasePoolExhausted
        expr: shortly_database_connection_pool_idle == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Database connection pool exhausted"
          description: "No idle database connections available"

      - alert: NoRecentURLCreation
        expr: shortly_urls_created_last_24h == 0
        for: 24h
        labels:
          severity: warning
        annotations:
          summary: "No URLs created in last 24 hours"
          description: "Service may not be receiving traffic"
```

## Metrics Implementation Details

### Data Collection

- **Real-time:** Metrics are collected from the database on each scrape request
- **No caching:** All values are freshly queried to ensure accuracy
- **Performance:** Optimized SQL queries use existing indexes
- **Expected latency:** < 50ms for typical datasets (< 10k URLs)

### Database Queries

Metrics collection executes approximately 15 SQL queries per scrape:

- 6 queries for URL metrics
- 3 queries for user metrics (when auth enabled)
- 2 queries for audit metrics (grouped)
- 1 query for TTL histogram data
- 0 queries for database pool metrics (uses sqlx Pool API)
- 0 queries for system metrics (calculated from application state)

All queries leverage existing database indexes for optimal performance.

### Scraping Frequency

**Recommended:** 15-60 seconds

- **High frequency (15s):** For production monitoring with quick alert detection
- **Standard (30s):** Balance between freshness and load
- **Low frequency (60s):** For development or low-traffic environments

### Performance Considerations

- Metrics collection adds minimal overhead (< 10ms per scrape on typical hardware)
- SQLite WAL mode prevents metrics queries from blocking writes
- Connection pool is shared with application traffic
- No memory buildup (counters are recalculated from database each time)

### Future Optimizations

If performance becomes a concern with large datasets:

1. **Caching:** Implement 10-30 second cache for metrics
2. **Background collection:** Use scheduler to pre-compute metrics
3. **Sampling:** Sample histogram data instead of loading all TTL values
4. **Materialized views:** Store pre-aggregated counts in dedicated tables
