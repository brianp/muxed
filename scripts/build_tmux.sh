#!/bin/bash
set -euo pipefail

TMUX_VERSION="${1:-master}"

# Detect OS
OS="$(uname -s)"
case "$OS" in
  Linux*)   OS=Linux ;;
  Darwin*)  OS=macOS ;;
  *)        echo "Unsupported OS: $OS" && exit 1 ;;
esac

echo "Installing tmux version $TMUX_VERSION"
echo "On OS: $OS"

# Where to install tmux
PREFIX="$HOME/tmux-$TMUX_VERSION"

# Idempotent check
if [ -x "$PREFIX/bin/tmux" ]; then
  echo "tmux $TMUX_VERSION already installed at $PREFIX"
  "$PREFIX/bin/tmux" -V
  exit 0
fi

# Install build deps
if [ "$OS" == "Linux" ]; then
    sudo apt-get update
    sudo apt-get remove -y tmux || true
    sudo apt-get install -y \
      automake \
      autoconf \
      pkg-config \
      build-essential \
      libevent-dev \
      libncurses5-dev
elif [ "$OS" == "macOS" ]; then
    brew update
    brew install automake autoconf pkg-config libevent ncurses
fi

# Clone + build
git clone https://github.com/tmux/tmux.git tmux-src
cd tmux-src
git checkout "$TMUX_VERSION"
sh autogen.sh
./configure --disable-utf8proc --prefix="$PREFIX" && make && make install
cd ..

# Update PATH (persist if on GitHub Actions)
if [ -n "${GITHUB_PATH:-}" ]; then
    echo "$PREFIX/bin" >> "$GITHUB_PATH"
else
    export PATH="$PREFIX/bin:$PATH"
    echo "Add this to your shell profile to persist:"
    echo "export PATH=\"$PREFIX/bin:\$PATH\""
fi

"$PREFIX/bin/tmux" -V