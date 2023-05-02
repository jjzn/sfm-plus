#!/bin/bash

set -euo pipefail

fail=0

run_case() {
    file="${1:-}"

    echo "running case '$file'"

    set +e
    tput dim
    cat <<EOF | python3
import json
import info
import sys
with open('test/$file.json') as f:
    want = json.dumps(json.loads(f.read()))
    got = json.dumps(info.retrieve_info('test/$file.jpg'))
    if want == got:
        sys.exit(0)
    else:
        print(f'failed comparison:\n\twanted: {want}\n\tgot:    {got}')
        sys.exit(1)
EOF

    status=$?
    set -e
    tput sgr0

    if [ $status -eq 0 ]; then
        echo "==> '$file' test `tput setaf 2`OK`tput sgr0`"
    else
        echo "==> '$file' test `tput setaf 1`FAIL`tput sgr0`"
        fail=$(($fail + 1))
    fi

    echo
}

run_all() {
    total=$#

    i=0
    for file in "$@"; do
        i=$(($i + 1))

        echo -n "[$i/$total] "
        run_case "$file"
    done

    res="TOTAL: $total, FAIL: $fail, OK: $(($total - $fail))"
    res_len=`echo $res | wc -c`
    printf '=%.0s' `seq -s' ' $res_len`
    echo
    echo $res

    exit $fail
}

if [ $# -gt 0 ]; then
    run_all "$@"
else
    run_all `basename -s .jpg test/*.jpg`
fi
