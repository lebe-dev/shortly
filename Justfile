version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
chartName := `cat helm-chart/Chart.yaml | yq -r '.name'`
chartVersion := `cat helm-chart/Chart.yaml | yq -r '.version'`

cleanup:
    rm -f {{ chartName }}-*.tgz

lint:
    cargo fmt
    cargo clippy
    cd frontend && yarn eslint .

bump-frontend-deps:
    cd frontend && rm -rf node_modules yarn.lock && yarn install

bump-backend-deps:
    cargo update

bump-deps: bump-frontend-deps && bump-backend-deps

build: lint
    cargo build

test-image-build:
    docker build --progress=plain -t app:dev .

run-backend:
    cargo run --bin server

run-frontend:
    cd frontend && yarn && npm run dev -- --port=4200

test:
    cd frontend && yarn test run
    cargo test --lib
    cargo test --bin server

start-dev-image:
    docker compose -f docker-compose-dev.yml up -d --build --force-recreate

stop-dev-image:
    docker compose -f docker-compose-dev.yml down

# HELM CHART
test-chart:
    helm template helm-chart/

build-chart: test-chart
    helm package helm-chart/ --app-version {{ version }}

release-chart: build-chart
    rm -rf helm-repo
    git clone git@github.com:tinyops-ru/tinyops-ru.github.io.git helm-repo
    bash -euo pipefail -c '\
        cd helm-repo && \
        cp ../{{ chartName }}-{{ chartVersion }}.tgz helm-charts/ && \
        helm repo index helm-charts/ && \
        if [ -z "$(git status --porcelain)" ]; then \
            echo "Chart {{ chartName }}-{{ chartVersion }} already published, skipping." && \
            exit 0; \
        fi && \
        git add helm-charts/ && \
        git commit -m "Add helm chart: {{ chartName }}-{{ chartVersion }}" && \
        git push'
    rm -rf helm-repo

# RELEASE

build-release-image: test
    docker build --progress=plain --platform=linux/amd64 -t tinyops/shortly:{{ version }} .

trivy:
    trivy image --severity HIGH,CRITICAL tinyops/shortly:{{ version }}

release: build-release-image
    docker push tinyops/shortly:{{ version }}
    @just release-chart
