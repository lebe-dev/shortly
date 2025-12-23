# Nginx Configuration for Shortly

This document provides production-ready nginx configurations for securing the Shortly URL shortener service with TLS and IP rate limiting.

## Table of Contents

- [Regular nginx Configuration](#regular-nginx-configuration)
- [nginx Ingress Controller Configuration](#nginx-ingress-controller-configuration)
- [Rate Limiting Tuning](#rate-limiting-tuning)

---

## Regular nginx Configuration

This configuration is suitable for traditional nginx deployments (bare metal, VMs, or Docker).

### Complete Configuration Example

```nginx
# Rate limiting zones
# General zone: 10 requests per second per IP
limit_req_zone $binary_remote_addr zone=general:10m rate=10r/s;

# API zone: 5 requests per second per IP (more restrictive)
limit_req_zone $binary_remote_addr zone=api:10m rate=5r/s;

# Strict zone for URL creation: 2 requests per second per IP
limit_req_zone $binary_remote_addr zone=create_url:10m rate=2r/s;

# Connection limiting: max 10 concurrent connections per IP
limit_conn_zone $binary_remote_addr zone=addr:10m;

upstream shortly_backend {
    server 127.0.0.1:31100;  # Docker setup (port 31100:8080)
    # server 127.0.0.1:8080; # Direct run (uncomment if not using Docker)

    keepalive 32;
    keepalive_timeout 60s;
}

server {
    listen 80;
    server_name shortly.example.com;

    # Redirect all HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name shortly.example.com;

    # TLS Configuration
    ssl_certificate /etc/ssl/certs/shortly.example.com.crt;
    ssl_certificate_key /etc/ssl/private/shortly.example.com.key;

    # Modern TLS configuration (TLS 1.2 and 1.3 only)
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384';
    ssl_prefer_server_ciphers off;

    # SSL session cache
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    ssl_session_tickets off;

    # OCSP stapling
    ssl_stapling on;
    ssl_stapling_verify on;
    resolver 8.8.8.8 8.8.4.4 valid=300s;
    resolver_timeout 5s;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self'; frame-ancestors 'self';" always;

    # Connection limiting
    limit_conn addr 10;

    # General rate limiting (default for all locations)
    limit_req zone=general burst=20 nodelay;

    # Logging
    access_log /var/log/nginx/shortly_access.log;
    error_log /var/log/nginx/shortly_error.log;

    # Maximum request body size
    client_max_body_size 10k;
    client_body_buffer_size 10k;

    # Timeouts
    client_body_timeout 10s;
    client_header_timeout 10s;
    send_timeout 10s;

    # API endpoints with stricter rate limiting
    location /api/ {
        # Override general rate limit with API-specific limit
        limit_req zone=api burst=10 nodelay;

        proxy_pass http://shortly_backend;
        proxy_http_version 1.1;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;

        # Connection reuse
        proxy_set_header Connection "";

        # Timeouts
        proxy_connect_timeout 5s;
        proxy_send_timeout 10s;
        proxy_read_timeout 10s;

        # No caching for API responses
        add_header Cache-Control "no-store, no-cache, must-revalidate, proxy-revalidate" always;
        add_header Pragma "no-cache" always;
    }

    # URL creation endpoint - most restrictive rate limit
    location = /api/url {
        limit_req zone=create_url burst=5 nodelay;

        proxy_pass http://shortly_backend;
        proxy_http_version 1.1;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        proxy_set_header Connection "";

        proxy_connect_timeout 5s;
        proxy_send_timeout 10s;
        proxy_read_timeout 10s;

        add_header Cache-Control "no-store, no-cache, must-revalidate" always;
    }

    # Static assets with caching
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        proxy_pass http://shortly_backend;

        # Cache static assets for 1 year
        add_header Cache-Control "public, max-age=31536000, immutable" always;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Longer timeout for static assets
        proxy_read_timeout 30s;
    }

    # Root and all other routes (SPA fallback)
    location / {
        proxy_pass http://shortly_backend;
        proxy_http_version 1.1;

        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        proxy_set_header Connection "";

        proxy_connect_timeout 5s;
        proxy_send_timeout 10s;
        proxy_read_timeout 10s;

        # Cache HTML for 5 minutes
        add_header Cache-Control "public, max-age=300" always;
    }

    # Deny access to hidden files
    location ~ /\. {
        deny all;
        access_log off;
        log_not_found off;
    }
}
```

### Configuration Notes

1. **TLS Certificates**: Replace paths in `ssl_certificate` and `ssl_certificate_key` with your actual certificate paths
2. **Server Name**: Replace `shortly.example.com` with your actual domain
3. **Backend Port**: Use `31100` for Docker setup (default) or `8080` for direct run
4. **Rate Limits**: Adjust values in `limit_req_zone` directives based on your traffic patterns

### Testing the Configuration

```bash
# Test nginx configuration
sudo nginx -t

# Reload nginx
sudo systemctl reload nginx

# Test rate limiting
for i in {1..20}; do curl -I https://shortly.example.com/api/version; done
```

---

## nginx Ingress Controller Configuration

This configuration is for Kubernetes deployments using nginx ingress controller.

### Complete Ingress Configuration

```yaml
# Shortly Service
apiVersion: v1
kind: Service
metadata:
  name: shortly-service
  namespace: default
spec:
  selector:
    app: shortly
  ports:
    - protocol: TCP
      port: 8080
      targetPort: 8080
  type: ClusterIP

---
# Ingress Resource with TLS and Rate Limiting
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: shortly-ingress
  namespace: default
  annotations:
    # TLS configuration
    cert-manager.io/cluster-issuer: "letsencrypt-prod"  # Use cert-manager for automatic TLS

    # Force HTTPS
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"
    nginx.ingress.kubernetes.io/ssl-protocols: "TLSv1.2 TLSv1.3"
    nginx.ingress.kubernetes.io/ssl-ciphers: "ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305"

    # Rate limiting - general (10 requests per second per IP)
    nginx.ingress.kubernetes.io/limit-rps: "10"
    nginx.ingress.kubernetes.io/limit-burst-multiplier: "2"

    # Connection limiting
    nginx.ingress.kubernetes.io/limit-connections: "10"

    # Security headers
    nginx.ingress.kubernetes.io/configuration-snippet: |
      more_set_headers "X-Frame-Options: SAMEORIGIN";
      more_set_headers "X-Content-Type-Options: nosniff";
      more_set_headers "X-XSS-Protection: 1; mode=block";
      more_set_headers "Referrer-Policy: strict-origin-when-cross-origin";
      more_set_headers "Content-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self'; frame-ancestors 'self';";
      more_set_headers "Strict-Transport-Security: max-age=31536000; includeSubDomains; preload";

    # Request body size limit
    nginx.ingress.kubernetes.io/proxy-body-size: "10k"

    # Timeouts
    nginx.ingress.kubernetes.io/proxy-connect-timeout: "5"
    nginx.ingress.kubernetes.io/proxy-send-timeout: "10"
    nginx.ingress.kubernetes.io/proxy-read-timeout: "10"

    # Enable CORS if needed (uncomment if required)
    # nginx.ingress.kubernetes.io/enable-cors: "true"
    # nginx.ingress.kubernetes.io/cors-allow-origin: "https://shortly.example.com"

spec:
  ingressClassName: nginx
  tls:
    - hosts:
        - shortly.example.com
      secretName: shortly-tls  # cert-manager will create this secret
  rules:
    - host: shortly.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: shortly-service
                port:
                  number: 8080

---
# ConfigMap for custom nginx configuration (optional advanced settings)
apiVersion: v1
kind: ConfigMap
metadata:
  name: shortly-nginx-configuration
  namespace: default
data:
  # Custom nginx settings
  limit-req-status-code: "429"
  limit-conn-status-code: "429"

  # Rate limiting zones
  http-snippet: |
    # API-specific rate limiting zone (5 req/s)
    limit_req_zone $binary_remote_addr zone=api_limit:10m rate=5r/s;

    # URL creation zone (2 req/s)
    limit_req_zone $binary_remote_addr zone=create_url_limit:10m rate=2r/s;

---
# Additional Ingress for stricter API rate limiting
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: shortly-api-ingress
  namespace: default
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"

    # Stricter rate limiting for API endpoints (5 req/s)
    nginx.ingress.kubernetes.io/limit-rps: "5"
    nginx.ingress.kubernetes.io/limit-burst-multiplier: "2"

    # Higher priority to match before general ingress
    nginx.ingress.kubernetes.io/priority: "100"

    # No caching for API responses
    nginx.ingress.kubernetes.io/configuration-snippet: |
      more_set_headers "Cache-Control: no-store, no-cache, must-revalidate, proxy-revalidate";
      more_set_headers "Pragma: no-cache";

spec:
  ingressClassName: nginx
  tls:
    - hosts:
        - shortly.example.com
      secretName: shortly-tls
  rules:
    - host: shortly.example.com
      http:
        paths:
          - path: /api
            pathType: Prefix
            backend:
              service:
                name: shortly-service
                port:
                  number: 8080

---
# Ingress for URL creation endpoint with strictest rate limiting
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: shortly-create-url-ingress
  namespace: default
  annotations:
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"

    # Most restrictive rate limiting (2 req/s)
    nginx.ingress.kubernetes.io/limit-rps: "2"
    nginx.ingress.kubernetes.io/limit-burst-multiplier: "2"

    # Highest priority
    nginx.ingress.kubernetes.io/priority: "200"

    nginx.ingress.kubernetes.io/configuration-snippet: |
      more_set_headers "Cache-Control: no-store, no-cache, must-revalidate";

spec:
  ingressClassName: nginx
  tls:
    - hosts:
        - shortly.example.com
      secretName: shortly-tls
  rules:
    - host: shortly.example.com
      http:
        paths:
          - path: /api/url
            pathType: Exact
            backend:
              service:
                name: shortly-service
                port:
                  number: 8080
```

### cert-manager ClusterIssuer Setup

If using cert-manager for automatic TLS certificates:

```yaml
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@example.com  # Replace with your email
    privateKeySecretRef:
      name: letsencrypt-prod-key
    solvers:
      - http01:
          ingress:
            class: nginx
```

### Deploying to Kubernetes

```bash
# Install cert-manager (if not already installed)
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.13.0/cert-manager.yaml

# Create ClusterIssuer
kubectl apply -f clusterissuer.yaml

# Deploy Shortly ingress configuration
kubectl apply -f shortly-ingress.yaml

# Verify ingress
kubectl get ingress -n default
kubectl describe ingress shortly-ingress -n default

# Check certificate status
kubectl get certificate -n default
kubectl describe certificate shortly-tls -n default

# Test rate limiting
for i in {1..20}; do curl -I https://shortly.example.com/api/version; done
```

---

## Rate Limiting Tuning

### Understanding Rate Limit Parameters

#### Regular nginx

- **`rate`**: Requests per second (e.g., `rate=10r/s`) or per minute (e.g., `rate=100r/m`)
- **`burst`**: Maximum number of requests that can exceed the rate limit temporarily
- **`nodelay`**: Process excess requests immediately (within burst) instead of delaying them
- **`zone size`**: Memory allocated for tracking IPs (10m ≈ 160,000 IP addresses)

#### nginx Ingress Controller

- **`limit-rps`**: Requests per second per IP
- **`limit-burst-multiplier`**: Multiplier for burst size (burst = rps × multiplier)
- **`limit-connections`**: Maximum concurrent connections per IP

### Recommended Rate Limits by Use Case

| Endpoint Type | Regular nginx | Ingress Controller | Reasoning |
|--------------|---------------|-------------------|-----------|
| URL Creation | `2r/s`, burst 5 | `limit-rps: "2"` | Most expensive operation, prevent abuse |
| API Endpoints | `5r/s`, burst 10 | `limit-rps: "5"` | Balance between usability and protection |
| Static Assets | `10r/s`, burst 20 | `limit-rps: "10"` | Higher limit for normal browsing |
| URL Retrieval | `10r/s`, burst 20 | `limit-rps: "10"` | Main functionality, needs good UX |

### Testing Rate Limits

```bash
# Test general rate limit
ab -n 100 -c 10 https://shortly.example.com/

# Test API rate limit
ab -n 50 -c 5 -m POST -H "Content-Type: application/json" \
   -p post_data.json https://shortly.example.com/api/url

# Monitor nginx logs
tail -f /var/log/nginx/shortly_error.log | grep "limiting requests"

# For Kubernetes
kubectl logs -n ingress-nginx -l app.kubernetes.io/name=ingress-nginx -f | grep "limiting"
```

### Adjusting Rate Limits

**When to increase limits:**
- Legitimate users are getting rate limited (check logs for 429 errors)
- Multiple users behind same IP (corporate NAT, VPN)
- High traffic legitimate use cases

**When to decrease limits:**
- Under DDoS or abuse
- High server load
- Excessive API calls from specific IPs

### Monitoring

```bash
# Regular nginx - watch for rate limit violations
tail -f /var/log/nginx/shortly_error.log | grep -E "limiting|refused"

# Check current connections
ss -tn | grep :443 | wc -l

# For Kubernetes - check ingress controller metrics
kubectl top pods -n ingress-nginx
kubectl logs -n ingress-nginx deployment/ingress-nginx-controller | grep -i limit
```

### Whitelist Trusted IPs (Optional)

For regular nginx, you can whitelist specific IPs:

```nginx
geo $limit {
    default 1;
    10.0.0.0/8 0;      # Internal network
    192.168.0.0/16 0;  # Private network
    1.2.3.4 0;         # Trusted IP
}

map $limit $limit_key {
    0 "";
    1 $binary_remote_addr;
}

limit_req_zone $limit_key zone=general:10m rate=10r/s;
```

For nginx Ingress Controller, use whitelist-source-range:

```yaml
nginx.ingress.kubernetes.io/whitelist-source-range: "10.0.0.0/8,192.168.0.0/16,1.2.3.4/32"
```

---

## Security Checklist

- [ ] TLS 1.2 and 1.3 enabled, older protocols disabled
- [ ] Strong cipher suites configured
- [ ] HSTS header with `preload` enabled
- [ ] Rate limiting configured for all endpoints
- [ ] Stricter rate limiting for URL creation endpoint
- [ ] Security headers (CSP, X-Frame-Options, etc.) configured
- [ ] Client request body size limited
- [ ] Timeouts configured appropriately
- [ ] OCSP stapling enabled (regular nginx)
- [ ] Automated certificate renewal (cert-manager for K8s)
- [ ] Regular monitoring of rate limit violations
- [ ] Log rotation configured

---

## Additional Resources

- [nginx Rate Limiting Documentation](https://nginx.org/en/docs/http/ngx_http_limit_req_module.html)
- [nginx Ingress Controller Annotations](https://kubernetes.github.io/ingress-nginx/user-guide/nginx-configuration/annotations/)
- [cert-manager Documentation](https://cert-manager.io/docs/)
- [Mozilla SSL Configuration Generator](https://ssl-config.mozilla.org/)
