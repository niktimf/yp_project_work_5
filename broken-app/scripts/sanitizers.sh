#!/usr/bin/env bash
set -euo pipefail

rustup component add rust-src --toolchain nightly 2>/dev/null || true

echo "=== ASan ==="
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test --test integration --target x86_64-unknown-linux-gnu test_ 2>&1

echo "=== TSan ==="
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test --test integration --target x86_64-unknown-linux-gnu -Zbuild-std test_ 2>&1