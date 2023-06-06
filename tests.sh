#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux
_FLAGS="${1:-}"

if [[ "-d" == ${_FLAGS} ]]; then 
    cargo test --release --doc --all-features
elif [[ "-t" == ${_FLAGS} ]]; then 
    cargo test --release --all-features "${@:2}"
else
    cargo test --release --all-targets --all-features
fi
