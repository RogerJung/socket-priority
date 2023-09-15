#!/usr/bin/env bash
set -e
cargo build --release --bin server_pn

parallel -j0 <<EOF
./target/release/server --listen-addr=192.168.1.1:55553 --priority=3
./target/release/server --listen-addr=192.168.1.1:55551 --priority=1
./target/release/server --listen-addr=192.168.1.1:55552 --priority=2
EOF
