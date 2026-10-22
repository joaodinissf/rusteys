#!/usr/bin/env bash
# Build and run the rusteys overlay in release mode. Any arguments are passed to rusteys.
set -euo pipefail
cd "$(dirname "$0")"
exec cargo run --release -- "$@"
