#!/usr/bin/env bash
sudo tc qdisc replace dev enp1s0 parent root handle 100 taprio \
     num_tc 3 \
     map 0 1 2 2 2 2 2 2 2 2 2 2 2 2 2 2 \
     queues 1@0 1@1 1@2 \
     base-time 200 \
     sched-entry S 00 300000 \
     sched-entry S 02 300000 \
     sched-entry S 04 800000 \
     flags 0x1 \
     txtime-delay 200000 \
     clockid CLOCK_TAI
