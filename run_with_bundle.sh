#!/usr/bin/env bash

set -e


echo "This script is used for troubleshooting"

./compile_bleeding_edge_bundler.sh
# As of 2021-08-06, Codeforces.com is using rust 1.49
#rustup default 1.49.0

if [[ ! -f Cargo.toml ]]; then
  echo "Not a cargo project. Aborting"
  exit 1
fi

# rust_bundler_cp
RUST_LOG=debug RUST_BACKTRACE=full ./rust_bundler_cp -i . -b rust_codeforce_template > /dev/shm/output.rs
rustc /dev/shm/output.rs -o /dev/shm/output
/dev/shm/output