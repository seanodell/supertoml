#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="$(cd ../../.. && pwd)/target/debug/supertoml"

# Test importing from sibling directory while running from a different location
mkdir -p child
cd child
supertoml ../dir1/config.toml test
