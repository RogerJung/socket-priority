#!/bin/bash

source env.sh

# Command to run
COMMAND="timeout 15 python3 client.py"
COMMAND2="python3 dynamic_taprio.py"


# Calculate the future time (e.g., 1 minute from now)
FUTURE_TIME=$(date -d "20 seconds" +"%H:%M:%S")

# Function to run the command locally
run_local_command() {
    while [[ $(date +"%H:%M:%S") != "$FUTURE_TIME" ]]; do sleep 0.1; done
    $COMMAND
}

sudo tc qdisc del dev enp5s0 root 2>/dev/null || true

while true; do
    # Run the command locally
    run_local_command
    
    # Wait a second before running the command again
    sleep 1

    
    FUTURE_TIME=$(date -d "20 seconds" +"%H:%M:%S")
done
