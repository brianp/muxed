FROM rustlang/rust:nightly

ENV TMUX_VERSION 3.0a
WORKDIR /usr/src

# This is a dummy build to get the dependencies cached
COPY . .
RUN cargo fetch --target x86_64-unknown-linux-gnu
RUN rm -rf ./*

RUN apt-get update && \
      apt-get install -y libevent-dev \
      locales \
      bison \
      byacc && \
      apt-get remove tmux

RUN git clone https://github.com/tmux/tmux.git /opt/tmux && \
    cd /opt/tmux && \
    git checkout $TMUX_VERSION && \
    sh autogen.sh && \
    ./configure --prefix=/opt/tmux && make && make install

ENV PATH $PATH:/opt/tmux/bin

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
      locale-gen

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8

RUN rustup component add rustfmt
RUN rustup component add clippy --toolchain=nightly || cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy
RUN cargo install clog-cli
