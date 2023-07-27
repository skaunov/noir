#!/usr/bin/env bash

function require_command {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "Error: $1 is required but not installed." >&2
        exit 1
    fi
}
function check_installed {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "$1 is not installed. Please install it." >&2
    return 1
  fi
  return 0
}
function run_or_fail {
  "$@"
  local status=$?
  if [ $status -ne 0 ]; then
    echo "Command '$*' failed with exit code $status" >&2
    exit $status
  fi
}

require_command toml2json
require_command jq
require_command cargo
require_command wasm-bindgen
require_command wasm-opt

export pname=$(toml2json < Cargo.toml | jq -r .package.name)

run_or_fail ./buildPhaseCargoCommand.sh
run_or_fail ./installPhase.sh

