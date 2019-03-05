FROM rustlang/rust:nightly as builder

ENV UNATTENDED 1
ENV OSX_VERSION_MIN=10.7

WORKDIR /usr/src/

RUN apt-get update && \
    apt-get install -qqy --no-install-recommends \
      clang \
      gcc \
      g++ \
      zlib1g-dev \
      libmpc-dev \
      libmpfr-dev \
      libgmp-dev

RUN git clone https://github.com/tpoechtrager/osxcross.git --depth 1 /osxcross/
COPY MacOSX10.10.sdk.tar.xz /osxcross/tarballs/

RUN cd /osxcross \
    && ./build.sh -y \
    && rm tarballs/MacOSX10.10.sdk.tar.xz

ENV PATH /osxcross/target/bin:$PATH

RUN rustup target add x86_64-apple-darwin

RUN rustup --version \
    && rustc --version \
    && cargo --version
