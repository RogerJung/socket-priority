#!/usr/bin/env bash
set -e

dev="$1"

shift || {
    echo "Usage: $0 <DEVICE>" >&2
    exit 1
}

sudo tc qdisc del dev "$dev" root 2>/dev/null || true

sudo tc qdisc replace dev "$dev" parent root handle 100 taprio \
     num_tc 4 \
     map 0 1 2 3 3 3 3 3 3 3 3 3 3 3 3 3 \
     queues 1@0 1@1 1@2 1@3 \
     base-time 0 \
     sched-entry S 01 8000000 \
     sched-entry S 03 8000000 \
     sched-entry S 05 8000000 \
     sched-entry S 09 8000000 \
     flags 0x2 2> /dev/null
