# Builder
FROM rust:latest AS builder
RUN update-ca-certificates
WORKDIR /discord_channel_cleaner
COPY ./ .
RUN cargo build --release

# Image
FROM ubuntu:latest
WORKDIR /dcc
COPY --from=builder /discord_channel_cleaner/target/release/discord_channel_cleaner ./
CMD ["/dcc/discord_channel_cleaner"]
