#!/usr/bin/env bash
set -euo pipefail

GDB_VERSION="15.1"
GDB_SHA256="38254eacd4572134bca9c5a5aa4d4ca564cbbd30c369d881f733fb6b903354f2"

# Install build dependencies via Homebrew
brew install pkg-config python@3.11 gettext gmp mpfr libmpc isl

# Download source
cd /tmp
curl -LO "https://ftp.gnu.org/gnu/gdb/gdb-${GDB_VERSION}.tar.xz"

# Verify checksum
echo "${GDB_SHA256}  gdb-${GDB_VERSION}.tar.xz" | shasum -a 256 -c

# Extract
tar -xf "gdb-${GDB_VERSION}.tar.xz"
cd "gdb-${GDB_VERSION}"

# Configure
./configure \
  --enable-targets=all \
  --prefix=/usr/local

# Build
make -j"$(sysctl -n hw.ncpu)"

# Install
sudo make install

# Create convenience symlink
if [ -f /usr/local/bin/gdb ]; then
    sudo ln -sf /usr/local/bin/gdb /usr/local/bin/gdb-multiarch
fi

echo
echo "Installed:"
gdb --version | head -n 1