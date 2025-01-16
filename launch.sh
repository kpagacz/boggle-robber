#!/bin/bash

cp nginx.conf /etc/nginx/nginx.conf
sudo nginx -s reload

cd heist
cargo run --release &
echo "Launched backend"

cd ../getaway
cargo run --release &
echo "Launched frontend"
