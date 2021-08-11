#!/usr/bin/env bash

set -e

# As of 2021-08-06, Codeforces.com is using rust 1.49
rustup default 1.49.0

if [[ ! -f Cargo.toml ]]; then
  echo "Not a cargo project. Aborting"
  exit 1
fi

# rust_bundler_cp
rust_bundler_cp . > /dev/shm/output.rs
rustc /dev/shm/output.rs -o /dev/shm/output
/dev/shm/output