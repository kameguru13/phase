#!/bin/bash
cd /home/user/wt/complete || exit 2
export no_proxy=$(echo "$no_proxy" | tr ',' '\n' | grep -v -e crates.io -e npmjs | paste -sd,)
export NO_PROXY=$(echo "$NO_PROXY" | tr ',' '\n' | grep -v -e crates.io -e npmjs | paste -sd,)
export CARGO_TARGET_DIR=/home/user/phase-personal/target CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUST_MIN_STACK=16777216
step() { echo "=== $1 $(date +%T)"; shift; "$@"; echo "=== rc=$?"; }
echo "HEAD $(git rev-parse HEAD)"; git status --short
step fmt cargo fmt --all --check
step gateA ./scripts/check-parser-combinators.sh
step authorities ./scripts/check-engine-authorities.sh addbc884ee251794a64f2e1da0487cd329f8ce90
step testcarddata ./scripts/check-test-card-data-load.sh addbc884ee251794a64f2e1da0487cd329f8ce90
step prelowered ./scripts/check-prelowered-ratchet.sh
step skilldoc ./scripts/check-skill-doc.sh
step cranchors ./scripts/check-cr-citation-anchors.sh
step frames ./scripts/check-resolution-frame-boundaries.sh
step drawcensus python3 scripts/draw_replacement_census.py --corpus --check
step costcensus python3 scripts/cost_walk_carrier_census.py --check
step bindings ./scripts/check-interaction-bindings.sh --check
step clippy cargo clippy --locked --workspace --exclude phase-tauri --all-targets --features engine/proptest -- -D warnings
CARGO_PROFILE_TEST_DEBUG=0 step tests cargo test --locked --workspace --exclude phase-tauri --features engine/proptest
