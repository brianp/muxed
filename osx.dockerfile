FROM brianp/rust-builds:osx

WORKDIR /usr/src
RUN USER=root cargo init
COPY Cargo.toml .
COPY Cargo.lock .

# This is a dummy build to get the dependencies cached
RUN cargo fetch --target x86_64-apple-darwin
