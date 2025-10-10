#!/bin/bash

shopt -s expand_aliases

cd "$(dirname "$0")"

alias supertoml="$(cd ../../.. && pwd)/target/debug/supertoml"

supertoml --version
