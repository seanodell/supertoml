#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="${SUPERTOML_BIN}"

# Test importing from sibling directory while running from a different location
mkdir -p child
cd child
supertoml ../dir1/config.toml test
