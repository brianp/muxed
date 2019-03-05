FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/
RUN USER=root cargo init
COPY Cargo.toml .

# This is a dummy build to get the dependencies cached
RUN cargo build --release
COPY . .
RUN cargo build --release

FROM debian:stretch-slim
COPY --from=builder /usr/src/muxed/target/release/muxed /bin/
