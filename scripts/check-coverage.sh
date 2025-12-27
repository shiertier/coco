#!/usr/bin/env bash
set -euo pipefail

cargo llvm-cov -p coco-core --lib --fail-under-lines 80
cargo llvm-cov -p coco-protocol --lib --fail-under-lines 90
