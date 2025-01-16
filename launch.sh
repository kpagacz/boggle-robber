#!/bin/bash

sudo apt install -y build-essential libssl-dev pkg-config nginx

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
