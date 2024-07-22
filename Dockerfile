FROM rustlang/rust:nightly as builder
ENV RUSTFLAGS="-C target-cpu=skylake"

WORKDIR /bot

RUN apt-get update && apt-get install -y cmake && apt-get clean

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN api-get update && apt-get upgrade -y && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /bot/target/release/discord-tier-list-bot /user/local/bin/discord-tier-list-bot

CMD ["/usr/local/bin/discord-tier-list-bot"]
