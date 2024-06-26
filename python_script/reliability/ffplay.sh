#!/usr/bin/bash

ffplay -fflags nobuffer -flags low_delay -framedrop tcp://${CLIENT_IP}:8080
