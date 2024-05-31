FROM rust:1.78 AS builder

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim AS runner
ENV DEBIAN_FRONTEND=noninteractive
RUN apt update && apt install -y --no-install-recommends \
    libssl-dev \
    ca-certificates
COPY --from=builder /usr/local/cargo/bin/ddns-sync /usr/local/bin/ddns-sync

CMD ["ddns-sync"]