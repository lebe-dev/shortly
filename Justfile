version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`

lint:
    cargo fmt
    cargo clippy

build: lint
    cargo build

test-image-build:
    docker build --progress=plain -t app:dev .

run-backend:
    cargo run --bin server

run-frontend:
    cd frontend && yarn && npm run dev -- --port=4200

test-all:
    cd frontend && yarn test run
    cargo test --lib
    cargo test --bin server

start-dev-image:
    docker compose -f docker-compose-dev.yml up -d --build --force-recreate

stop-dev-image:
    docker compose -f docker-compose-dev.yml down

# RELEASE

build-release-image: test-all
    docker build --progress=plain --platform=linux/amd64 -t tinyops/shortly:{{ version }} .

build-chart:
    helm package helm-chart/

trivy:
    trivy image --severity HIGH,CRITICAL tinyops/shortly:{{ version }}

release: build-release-image
    docker push tinyops/shortly:{{ version }}
    helm package helm-chart/
