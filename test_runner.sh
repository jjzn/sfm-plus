#!/bin/bash

set -euo pipefail

run_case() {
    file="${1:-}"

    echo "running case '$file'"
    cat <<EOF | python3
import json
import info
with open('test/$file.json') as f:
    want = json.dumps(json.loads(f.read()))
    got = json.dumps(info.retrieve_info('test/$file.jpg'))
    assert want == got, f'failed comparison:\n\twanted: {want}\n\tgot: {got}'
EOF
}

for file in `basename -s .jpg test/*.jpg`; do
    run_case "$file"
done
