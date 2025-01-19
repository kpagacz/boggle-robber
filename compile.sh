#!/bin/bash

# check if cross is installed
if [ -x "$(cross -v)" ]; then
  echo "cross is not installed. Please install cross first."
  exit 1
fi

cd heist
cross build --release --target aarch64-unknown-linux-gnu

cd ../getaway
cross build --release --target aarch64-unknown-linux-gnu
