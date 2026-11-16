set lazy

imageName := "tinyops/shortly"
version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`
gitShortSha := `git rev-parse --short HEAD`
devVersion := version + "-" + gitShortSha
chartName := `cat helm-chart/Chart.yaml | yq -r '.name'`
chartVersion := `cat helm-chart/Chart.yaml | yq -r '.version'`

cleanup:
    rm -f {{ chartName }}-*.tgz

# --- Lints ---
lint: lint-backend lint-frontend lint-chart

lint-backend:
    cargo fmt --all -- --check
    cargo clippy --all-targets --locked -- -D warnings

lint-frontend:
    cd frontend && yarn lint
    cd frontend && yarn check

lint-chart:
    helm lint helm-chart/

# --- Security ---
# Требует OSV-Scanner v2, Gitleaks v8, Semgrep CE, Trivy и собранный образ.
security: security-osv security-gitleaks security-semgrep security-trivy-config security-trivy-image

security-osv:
    osv-scanner scan source --lockfile=Cargo.lock --lockfile=frontend/yarn.lock

# История всех локальных веток и текущее содержимое файлов; секреты скрыты в выводе.
security-gitleaks:
    gitleaks git --redact --log-opts="--all" .
    gitleaks dir --redact .

security-semgrep:
    semgrep scan --oss-only --config p/security-audit --error --strict --metrics=off --exclude target --exclude frontend/node_modules --exclude frontend/build --exclude frontend/.svelte-kit .

security-trivy-config:
    trivy fs --scanners misconfig --exit-code 1 --skip-dirs target --skip-dirs frontend/node_modules --skip-dirs frontend/build --skip-dirs frontend/.svelte-kit .

# Проверяет существующий локальный образ; не скачивает его из registry.
security-trivy-image:
    trivy image --image-src docker --scanners vuln --severity HIGH,CRITICAL --exit-code 1 "{{ imageName }}:{{ version }}"

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
    docker build --progress=plain --platform=linux/amd64 -t {{ imageName }}:{{ version }} .

build-release-dev-image: test
    docker build --progress=plain --platform=linux/amd64 -t {{ imageName }}:{{ devVersion }} .

push-image:
    docker push {{ imageName }}:{{ version }}

push-dev-image:
    docker push {{ imageName }}:{{ devVersion }}

# Совместимость с прежней командой проверки образа.
trivy: security-trivy-image

release-image: build-release-image
    @just push-image

release: build-release-image
    @just push-image
    @just release-chart

release-dev: build-release-dev-image
    @just push-dev-image
