#!/usr/bin/env bash
# This script is pretty low tech, but it helps keep the doc version numbers
# up to date. It should be run BEFORE tagging a new release, but AFTER
# bumping the version in Cargo.toml.

set -euo pipefail

wanted=$(cargo read-manifest | jq -r .version)

for file in Cargo.toml README.md src/lib.rs; do
    sed -i.bak -E \
        -e "s|version=[0-9.]+|version=$wanted|g" \
        -e "s|r2d2_redis/[0-9.]+|r2d2_redis/$wanted|g" \
        -e "s|r2d2_redis = \"[0-9.]+\"|r2d2_redis = \"$wanted\"|g" \
        "$file"
    rm "$file.bak"
done
