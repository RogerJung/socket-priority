#!/usr/bin/env bash
set -e
cargo build --release --bin server_pn

parallel -j0 <<EOF
./target/release/server_pn --listen-addr=192.168.1.1:55553 --priority=3 >p1.out
./target/release/server_pn --listen-addr=192.168.1.1:55551 --priority=1 >p2.out
./target/release/server_pn --listen-addr=192.168.1.1:55552 --priority=2 >p3.out
EOF
