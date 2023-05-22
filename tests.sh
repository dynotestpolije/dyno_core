#!/usr/bin/env bash
# This scripts runs various CI-like checks in a convenient way.
set -eux
_FLAGS="${1:-}"

if [[ "-d" == ${_FLAGS} ]]; then 
    cargo test --release --doc --all-features -- -Zunstable-options --report-time 
else
    cargo test --release --all-targets --all-features -- -Zunstable-options --report-time
fi
