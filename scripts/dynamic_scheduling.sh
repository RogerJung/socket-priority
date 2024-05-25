#!/bin/bash

# Device to configure
set -e

dev="$1"
shift || {
    echo "Usage: $0 <DEVICE>" >&2
    exit 1
}

# Function to get current network load (this is a placeholder)
get_current_load() {
    # Placeholder for actual load monitoring logic
    echo $((RANDOM % 100))  # Random load percentage for demonstration
}

# Function to adjust scheduling based on load
adjust_schedule() {
    local load=$1
    if [ "$load" -gt 80 ]; then
        # High load: prioritize certain queues
        echo "High load detected: $load%. Adjusting schedule for high load."
        sched_entry1="S 01 3000000"
        sched_entry2="S 02 3000000"
        sched_entry3="S 04 2000000"
        sched_entry4="S 08 2000000"
    else
        # Normal load: use default settings
        echo "Normal load detected: $load%. Using default schedule."
        sched_entry1="S 01 5000000"
        sched_entry2="S 02 4000000"
        sched_entry3="S 04 3000000"
        sched_entry4="S 08 2000000"
    fi

    # Apply new schedule
    sudo tc qdisc replace dev "$dev" parent root handle 100 taprio \
        num_tc 4 \
        map 0 1 2 3 2 2 2 2 2 2 2 2 2 2 2 2 \
        queues 1@0 1@1 1@2 1@3 \
        base-time 0 \
        sched-entry $sched_entry1 \
        sched-entry $sched_entry2 \
        sched-entry $sched_entry3 \
        sched-entry $sched_entry4 \
        flags 0x2
}

# Main loop to continuously adjust the schedule
while true; do
    current_load=$(get_current_load)
    adjust_schedule "$current_load"
    sleep 5  # Adjust every 5 seconds, tune this as needed
done

