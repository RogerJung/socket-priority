#!/bin/bash

source env.sh

# Command to run
COMMAND="timeout 15 python3 server.py"

COUNTER=1

# Calculate the future time (e.g., 1 minute from now)
FUTURE_TIME=$(date -d "20 seconds" +"%H:%M:%S")

# Function to run the command locally
run_local_command() {
    while [[ $(date +"%H:%M:%S") != "$FUTURE_TIME" ]]; do sleep 0.1; done
    
    # Log file name
    LOG_FILE="exp${COUNTER}.txt"
    
    $COMMAND | tee $LOG_FILE

    # Increment log counter
    ((COUNTER++))
}

while true; do
    # Run the command locally
    run_local_command
    
    # Wait a second before running the command again
    sleep 1
    FUTURE_TIME=$(date -d "20 seconds" +"%H:%M:%S")
done