# Starter: Rust + Svelte

![project logo](logo.png)

1. Rename app name in `Cargo.toml`
2. Rename `server` to your app name in `Dockerfile`
3. Rename `myapp` to your app name in `.gitlab-ci.yml`

## How to build

```bash
docker build --progress=plain --platform=linux/amd64 -t myapp:1.0.0 .
```

## Technical stack

- Backend: Rust + axum + sqlite (sqlx)
- Frontend: [Svelte 5](https://svelte.dev/docs/svelte/overview) + [tailwindcss](https://tailwindcss.com/docs/installation/tailwind-cli) + [shadcn](https://shadcn-svelte.com/docs/)

## Docs for LLMs

Check `docs/LLM`.

## TODO

1. .gitlab-ci.yml
