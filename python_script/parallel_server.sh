#!/bin/bash

commands=(
    "python3 server_streaming.py -p 8000"
    "python3 server_streaming.py -p 8001"
    "python3 server_streaming.py -p 8002"
    "python3 server_streaming.py -p 8003"
)

parallel ::: "${commands[@]}"
