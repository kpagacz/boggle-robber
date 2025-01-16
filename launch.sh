#!/bin/bash

cd heist
cargo run --release &
echo "Launched backend"

cd ../getaway
cargo run --release &
echo "Launched frontend"
