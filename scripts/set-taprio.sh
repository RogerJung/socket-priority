#!/usr/bin/env bash
set -e

dev="$1"
shift || {
    echo "Usage: $0 <DEVICE>" >&2
    exit 1
}

sudo tc qdisc replace dev "$dev" parent root handle 100 taprio \
     num_tc 3 \
     map 0 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 \
     queues 1@0 1@1 1@2 \
     base-time 200 \
     sched-entry S 01 300000 \
     sched-entry S 03 300000 \
     sched-entry S 05 800000 \
     flags 0x1 \
     txtime-delay 200000 \
     clockid CLOCK_TAI
