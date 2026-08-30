FROM node:25-alpine AS frontend-build

WORKDIR /build

COPY frontend/ /build
COPY Cargo.toml /tmp/Cargo.toml

RUN VERSION=$(awk -F'"' '/^version[[:space:]]*=/{print $2; exit}' /tmp/Cargo.toml) && \
    test -n "$VERSION" && \
    sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"${VERSION}\"/" package.json && \
    yarn && \
    yarn test run && \
    yarn build

FROM rust:1.98.0-alpine AS app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk add elfutils pkgconfig perl make upx openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock /build/
COPY .cargo/ /build/.cargo/
COPY src/ /build/src/
COPY migrations/ /build/migrations/
COPY test-data/ /build/test-data/
COPY config.yml-dist /build/config.yml-dist
COPY --from=frontend-build /build/build/ /build/static/

RUN cargo test --lib && \
    cargo test --bin server && \
    cargo build --bin server --release && \
    eu-elfcompress target/release/server && \
    strip target/release/server && \
    upx -9 --lzma target/release/server && \
    chmod +x target/release/server

FROM alpine:3.24

WORKDIR /app

RUN apk update && \
    addgroup -g 10001 app && \
    adduser -h /app -D -u 10001 -G app app && \
    chmod 700 /app && \
    chown -R app: /app

COPY --from=app-build /build/config.yml-dist /app/config.yml
COPY --from=app-build /build/target/release/server /app/shortly

RUN chown -R app: /app && chmod +x /app/shortly

USER app

CMD ["/app/shortly"]
