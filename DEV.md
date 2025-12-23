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
