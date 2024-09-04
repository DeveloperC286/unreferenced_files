#!/usr/bin/env sh

set -o errexit
set -o xtrace

cd "unreferenced_files/end-to-end-tests/"
behave
