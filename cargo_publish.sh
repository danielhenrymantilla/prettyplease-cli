#!/bin/sh

set -euxo pipefail

for i in $(seq 10)
do
    cargo publish && exit 0
    sleep 5
done
cargo publish
