FROM brianp/rust-builds:osx

WORKDIR /usr/src
RUN USER=root cargo init
COPY Cargo.toml .
COPY Cargo.lock .

# This is a dummy build to get the dependencies cached
RUN cargo fetch --target x86_64-apple-darwin

RUN apt-get update && \
      apt install -y tmux && \
      apt install -y locales

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
      locale-gen

ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8
