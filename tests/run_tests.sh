#!/usr/bin/env bash

set -e

echo "----- Running 'cargo test' -----"
cargo test
echo "--------------------------------"

echo "----- Running negative tests -----"
!(cd do_not_load_settings_from_cargo_manifest && cargo build --target-dir "../target")
echo "----------------------------------"
