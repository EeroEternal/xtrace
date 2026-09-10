FROM rust:1.88-bookworm AS builder
WORKDIR /usr/src/xtrace
COPY . .
RUN cargo build --release --bin xtrace

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system xtrace \
    && useradd --system --gid xtrace --home-dir /app --create-home xtrace
WORKDIR /app
COPY --from=builder /usr/src/xtrace/target/release/xtrace /usr/local/bin/xtrace
RUN mkdir -p /app/.xtrace_data /app/.xtrace_media \
    && chown -R xtrace:xtrace /app
USER xtrace
ENV BIND_ADDR=0.0.0.0:8742
EXPOSE 8742
ENTRYPOINT ["/usr/local/bin/xtrace"]
