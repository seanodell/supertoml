#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="${SUPERTOML_BIN}"

# Test import with ./ prefix from child directory
mkdir -p child
cd child
supertoml ../config.toml test
