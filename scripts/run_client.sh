#!/bin/bash
set -e

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir/.."

dir="$1"
shift || {
    echo "Usage: $0 <OUT_DIR>" >&2
    exit 1
}

mkdir "$dir"
cargo build --release --all-targets

for prio in {0..6}; do
    echo "./target/release/client -c 192.168.1.1:5555${prio} -p ${prio} -s 64 > $dir/p${prio}.std 2> $dir/p${prio}.err"
done | parallel -j0 --lb
