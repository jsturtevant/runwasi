#!/usr/bin/env bash

set -e

CARGO_FLAGS=""
if [[ $RUNNER_OS == "Windows" ]]; then
    CARGO_FLAGS="--no-default-features"
fi

cargo build --all --verbose --features generate_doc $CARGO_FLAGS
git status --porcelain | grep README.md || exit 0

echo "README.md is not up to date. Please run 'cargo build --all --features generate_doc' and commit the changes." >&2
exit 1
