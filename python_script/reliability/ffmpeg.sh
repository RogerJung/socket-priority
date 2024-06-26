#!/bin/bash

ffmpeg -re -i <video_path> -f mpegts tcp://${CLIENT_IP}:8003?connect