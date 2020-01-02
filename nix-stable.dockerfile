FROM rust:latest

ENV TMUX_VERSION 3.0a
WORKDIR /usr/src

# This is a dummy build to get the dependencies cached
COPY . .
RUN cargo fetch --target x86_64-unknown-linux-gnu; \
    cargo build; \
    rm -rf ./*

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    automake \
    bison \
    byacc \
    git \
    libevent-dev \
    libncurses-dev \
    locales \
    pkg-config; \
    apt-get remove tmux

RUN git clone --depth=1 --branch $TMUX_VERSION https://github.com/tmux/tmux.git /opt/tmux && \
    cd /opt/tmux && \
    git checkout $TMUX_VERSION && \
    sh autogen.sh && \
    ./configure --prefix=/opt/tmux && make && make install

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
      locale-gen

ENV PATH=$PATH:/opt/tmux/bin \
    LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8

RUN apt-get remove \
    automake \
    bison \
    byacc \
    git \
    libevent-dev \
    libncurses-dev \
    locales \
    pkg-config; \
    rm -rf /var/lib/apt/lists/*;

RUN rustup component add rustfmt; \
    cargo install clog-cli
# RUN rustup component add clippy --toolchain=nightly || cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy
