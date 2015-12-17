#!/usr/bin/env bash
# This script is pretty low tech, but it helps keep the doc version numbers
# up to date. It should be run BEFORE tagging a new release, but AFTER
# bumping the version in Cargo.toml.

set -euo pipefail

wanted=v$(cargo read-manifest | jq -r .version)

for file in Cargo.toml README.md src/lib.rs; do
    sed -i.bak -e "s|r2d2-redis/doc/[[:alnum:].]*|r2d2-redis/doc/$wanted|" "$file"
    rm "$file.bak"
done
