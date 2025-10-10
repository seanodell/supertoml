#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="$(cd ../../.. && pwd)/target/debug/supertoml"

# This should replicate the error: running from child directory with parent file that imports from parent
mkdir -p child
cd child
supertoml ../config.toml test
