FROM brianp/rust-builds:osx

WORKDIR /usr/src

# This is a dummy build to get the dependencies cached
COPY . .
RUN cargo fetch --target x86_64-apple-darwin
RUN rm -rf ./*

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
    tmux \
    locales;

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
      locale-gen

ENV LANG=en_US.UTF-8 \
    LANGUAGE=en_US:en \
    LC_ALL=en_US.UTF-8
