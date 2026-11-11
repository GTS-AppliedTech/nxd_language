#!/usr/bin/env bash

set -e

echo "===== Formatting & Building ====="
cargo fmt
cargo build
echo "===================="

echo "===================="
echo "===== Testing ====="
cargo test
echo "===================="

echo "===================="
echo "===== All checks passed ====="