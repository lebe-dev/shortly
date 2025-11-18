FROM node:24.11.1-alpine3.22 AS frontend-build

WORKDIR /build

COPY frontend/ /build

RUN yarn && \
    yarn test run && \
    yarn build

FROM rust:1.91.1-alpine3.22 AS app-build

WORKDIR /build

RUN mkdir -p /build/static && \
    apk add nodejs npm musl-dev elfutils xz wget pkgconfig libressl-dev perl make && \
    wget https://github.com/upx/upx/releases/download/v4.0.2/upx-4.0.2-amd64_linux.tar.xz && \
    unxz upx-4.0.2-amd64_linux.tar.xz && tar xvf upx-4.0.2-amd64_linux.tar && \
    cp upx-4.0.2-amd64_linux/upx /usr/bin/upx && chmod +x /usr/bin/upx

COPY . /build
COPY --from=frontend-build /build/build/ /build/static/

RUN cargo test && \
    cargo build --bin server --release && \
    eu-elfcompress target/release/server && \
    strip target/release/server && \
    upx -9 --lzma target/release/server && \
    chmod +x target/release/server

FROM alpine:3.22.2

WORKDIR /app

RUN apk add libressl-dev && \
    adduser -h /app -D app && \
    chmod 700 /app && \
    chown -R app: /app

COPY --from=app-build /build/config.yml-dist /app/config.yml
COPY --from=app-build /build/target/release/server /app/server

RUN chown -R app: /app && chmod +x /app/server

USER app

CMD ["/app/server"]
