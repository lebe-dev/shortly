# Monitoring

## Health Check Endpoint

Shortly provides a health check endpoint for monitoring service availability and database connectivity.

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
