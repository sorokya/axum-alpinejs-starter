# Node builder stage
FROM node:24-alpine3.23 AS assets-builder

WORKDIR /app

# Install dependencies
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./

RUN corepack enable \
    && pnpm config set store-dir /root/.pnpm-store

RUN --mount=type=cache,target=/root/.pnpm-store \
    pnpm install --frozen-lockfile

COPY assets ./assets

RUN pnpm run build

# Rust builder stage
FROM rust:1.92-alpine3.23 AS rust-builder

ARG TARGETARCH

RUN apk add --no-cache musl-dev

RUN case "${TARGETARCH}" in \
    amd64)  echo "x86_64-unknown-linux-musl" > /rust_target ;; \
    arm64)  echo "aarch64-unknown-linux-musl" > /rust_target ;; \
    *)      echo "Unsupported arch: ${TARGETARCH}" && exit 1 ;; \
    esac

RUN rustup target add $(cat /rust_target)

WORKDIR /usr/src

RUN USER=root cargo new axum-alpinejs-starter

WORKDIR /usr/src/axum-alpinejs-starter

COPY Cargo.toml Cargo.lock .

RUN cargo build --release

COPY src ./src
COPY templates ./templates

RUN touch src/main.rs

RUN cargo build --release

# Final stage
FROM alpine:3.23 AS runtime

COPY --from=rust-builder /usr/src/axum-alpinejs-starter/target/release/axum-alpinejs-starter /usr/local/bin/axum-alpinejs-starter

WORKDIR /app

COPY public ./public

COPY --from=assets-builder /app/public ./assets/public

EXPOSE 3000

CMD ["axum-alpinejs-starter"]
