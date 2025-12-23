version := `cat Cargo.toml | grep version | head -1 | cut -d " " -f 3 | tr -d "\""`

test-image-build:
  docker build --progress=plain -t app:dev .

run-backend:
  cargo run --bin server

run-frontend:
  cd frontend && npm run dev -- --port=4200

test-all:
  cargo test --lib
  cargo test --bin server

build-release-image: test-all
  docker build --progress=plain --platform=linux/amd64 -t tinyops/shortly:{{version}} .

release: build-release-image
  docker push tinyops/shortly:{{version}}

build-chart:
  helm package helm-chart/
