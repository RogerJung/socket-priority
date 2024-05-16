#!/bin/bash

parallel ::: "python3 client_streaming.py -p 8000 -i 0" \
              "python3 client_streaming.py -p 8001 -i 1" \
              "python3 client_streaming.py -p 8002 -i 2" \
              "python3 client_streaming.py -p 8003 -i 3"
