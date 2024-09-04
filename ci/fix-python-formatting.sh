#!/usr/bin/env sh

set -o errexit
set -o xtrace

find "unreferenced_files/end-to-end-tests/features/" -type f | grep "[.]py$" | xargs -I {} autopep8 --in-place --aggressive --aggressive --max-line-length 120 "{}"
