#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="${SUPERTOML_BIN}"

supertoml --version
