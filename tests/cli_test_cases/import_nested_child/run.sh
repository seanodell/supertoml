#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="${SUPERTOML_BIN}"

# Test from deeply nested child directory
mkdir -p child/grandchild
cd child/grandchild
supertoml ../../config.toml test
