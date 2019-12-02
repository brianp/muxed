FROM rustlang/rust:nightly

WORKDIR /usr/src
RUN USER=root cargo init

# This is a dummy build to get the dependencies cached
COPY . .
RUN cargo fetch --target x86_64-unknown-linux-gnu
RUN rm -rf ./*

RUN apt-get update && \
      apt install -y tmux && \
      apt install -y locales

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
      locale-gen

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

RUN rustup component add rustfmt
RUN rustup component add clippy --toolchain=nightly || cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy
RUN cargo install clog-cli
