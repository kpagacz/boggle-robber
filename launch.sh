#!/bin/bash

sudo apt install build-essential libssl-dev pkg-config

cp nginx.conf /etc/nginx/nginx.conf
sudo nginx -s reload

cd heist
cargo build --release
cargo run --release &
echo "Launched backend"

cd ../getaway
cargo build --release
cargo run --release &
echo "Launched frontend"
