FROM rust as builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config openssl libssl-dev

RUN cargo install --git https://github.com/leptos-rs/cargo-leptos cargo-leptos

COPY . .

RUN echo "my-leptos-app-$(tr -dc a-z0-9 </dev/urandom | head -c 10)" > leptos_output_name

RUN LEPTOS_OUTPUT_NAME=$(cat leptos_output_name) cargo leptos build -r -P

FROM debian:12-slim


COPY --from=builder /app/target/release/blog /app/target/release/blog
COPY --from=builder /app/target/site /app/target/site
COPY --from=builder /app/leptos_output_name /app/leptos_output_name

WORKDIR /app

ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"

EXPOSE 8080

CMD ["sh", "-c", "LEPTOS_OUTPUT_NAME=$(cat leptos_output_name) ./target/release/blog"]
