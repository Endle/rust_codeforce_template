#!/usr/bin/env bash

set -e

cd ../rust-bundler-cp
cargo build
cp --force -v target/debug/rust_bundler_cp ../rust_codeforce_template
