FROM rust:1-stretch as builder

WORKDIR /usr/src/muxed
RUN USER=root cargo init
COPY /muxed/Cargo.toml .

# This is a dummy build to get the dependencies cached
RUN cargo build --release
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
COPY --from=builder /usr/src/muxed/target/release/muxed /bin/
