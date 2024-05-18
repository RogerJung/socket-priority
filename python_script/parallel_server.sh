#!/bin/bash

if [[ $# -lt 2 || "$1" != "-o" ]]; then
    echo "Usage: $0 -o <output>"
    exit 1
fi

output="$2"
shift 2

# Remote Driving
# 8000 -> 10000
# 8001 -> 30000
# 8002 -> 250000
# 8003 -> 500000

commands=(
    "python3 server_streaming.py -p 8000 -s 10000 > ${output}/0.txt"
    "python3 server_streaming.py -p 8001 -s 10000 > ${output}/1.txt"
    "python3 server_streaming.py -p 8002 -s 10000 > ${output}/2.txt"
    "python3 server_streaming.py -p 8003 -s 10000 > ${output}/3.txt"
)

parallel ::: "${commands[@]}"
