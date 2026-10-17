#!/bin/bash
cd /home/user/wt/complete || exit 2
export no_proxy=$(echo "$no_proxy" | tr ',' '\n' | grep -v -e crates.io -e npmjs | paste -sd,)
export NO_PROXY=$(echo "$NO_PROXY" | tr ',' '\n' | grep -v -e crates.io -e npmjs | paste -sd,)
export CARGO_TARGET_DIR=/home/user/phase-personal/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUST_MIN_STACK=16777216 CARGO_PROFILE_TEST_DEBUG=0
echo "HEAD $(git rev-parse HEAD)"; git status --short
clean() { find "$CARGO_TARGET_DIR/debug/deps" -maxdepth 1 -type f -perm -u+x -size +20M -delete; df -h / | tail -1; }
pkgs=$(cargo metadata --no-deps --format-version 1 | python3 -c "import json,sys; print(' '.join(p['name'] for p in json.load(sys.stdin)['packages'] if p['name']!='phase-tauri'))")
for p in $pkgs; do
  echo "=== $p $(date +%T)"
  if [ "$p" = phase-engine ] || cargo tree -q -p "$p" -i phase-engine -e normal,dev >/dev/null 2>&1; then f="--features engine/proptest"; else f=""; fi
  echo "features: $f"
  cargo test --locked -p "$p" $f --no-fail-fast 2>&1 | grep -E "^test result:|FAILED|panicked at|^error|Running |warning: unused" 
  echo "=== rc=${PIPESTATUS[0]}"
  clean
done
