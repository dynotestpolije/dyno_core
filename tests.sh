#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux

cargo test --all-targets --all-features
cargo test --doc --all-features
