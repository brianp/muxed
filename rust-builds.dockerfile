FROM rustlang/rust:nightly

ENV UNATTENDED 1
ENV OSX_VERSION_MIN 10.15
ENV PKG_FILE MacOSX10.15.sdk.tar.xz
ENV PKG_CONFIG_ALLOW_CROSS 1

WORKDIR /usr/src/

RUN apt-get update && \
    apt-get install -qqy --no-install-recommends \
      clang \
      cmake \
      g++ \
      gcc \
      libgmp-dev \
      libmpc-dev \
      libmpfr-dev \
      zlib1g-dev

RUN git clone https://github.com/tpoechtrager/osxcross.git --depth 1 /osxcross/
COPY $PKG_FILE /osxcross/tarballs/

RUN cd /osxcross \
    && ./build.sh -y \
    && rm tarballs/$PKG_FILE

ENV PATH /osxcross/target/bin:$PATH

RUN rustup target add x86_64-apple-darwin

RUN rustup component add rustfmt
RUN rustup component add clippy --toolchain=nightly || cargo install --git https://github.com/rust-lang/rust-clippy/ --force clippy

RUN rustup --version \
    && rustc --version \
    && cargo --version
