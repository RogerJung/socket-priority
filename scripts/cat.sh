#!/usr/bin/env bash


script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$script_dir/.."

dir="$1"
shift


for i in {0..6}
do
echo p$i
cat $dir/p${i}.std
cat $dir/p${i}.err
done
