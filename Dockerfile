FROM node:25.9.0-alpine3.23 AS frontend-build

WORKDIR /build

COPY frontend/ /build
COPY Cargo.toml /tmp/Cargo.toml

RUN VERSION=$(awk -F'"' '/^version[[:space:]]*=/{print $2; exit}' /tmp/Cargo.toml) && \
    test -n "$VERSION" && \
    sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"${VERSION}\"/" package.json && \
    yarn && \
    yarn test run && \
    yarn build

FROM rust:1.95.0-alpine3.23 AS app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk add elfutils pkgconfig perl make upx openssl-dev openssl-libs-static

COPY Cargo.toml Cargo.lock /build/
COPY .cargo/ /build/.cargo/
COPY src/ /build/src/
COPY migrations/ /build/migrations/
COPY config.yml-dist /build/config.yml-dist
COPY --from=frontend-build /build/build/ /build/static/

RUN cargo test --lib && \
    cargo test --bin server && \
    cargo build --bin server --release && \
    eu-elfcompress target/release/server && \
    strip target/release/server && \
    upx -9 --lzma target/release/server && \
    chmod +x target/release/server

FROM alpine:3.23

WORKDIR /app

RUN apk update && \
    adduser -h /app -D app && \
    chmod 700 /app && \
    chown -R app: /app

COPY --from=app-build /build/config.yml-dist /app/config.yml
COPY --from=app-build /build/target/release/server /app/shortly

RUN chown -R app: /app && chmod +x /app/shortly

USER app

CMD ["/app/shortly"]
