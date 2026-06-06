FROM rust:bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev

RUN cargo install cargo-leptos --version 0.3.4 --locked

RUN rustup toolchain install nightly-2024-08-10 \
    && rustup target add --toolchain nightly-2024-08-10 wasm32-unknown-unknown

ENV RUSTUP_TOOLCHAIN="nightly-2024-08-10"

COPY . .

RUN echo "my-leptos-app-$(tr -dc a-z0-9 </dev/urandom | head -c 10)" > leptos_output_name

RUN LC_ALL=C \
    LEPTOS_TAILWIND_VERSION=v3.4.17 \
    LEPTOS_OUTPUT_NAME=$(cat leptos_output_name) \
    cargo leptos build --frontend-only

RUN LEPTOS_OUTPUT_NAME=$(cat leptos_output_name) cargo build --release --features ssr

FROM debian:12-slim


COPY --from=builder /app/target/release/blog /app/target/release/blog
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/leptos_output_name /app/leptos_output_name

WORKDIR /app

ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="target/site"

EXPOSE 8080

CMD ["sh", "-c", "LEPTOS_OUTPUT_NAME=$(cat leptos_output_name) ./target/release/blog"]
