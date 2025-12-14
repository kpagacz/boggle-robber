#!/bin/bash

INST="52.59.249.245"
IDENTITY="~/.ssh/WjazdDoCentraliKurwa"

ssh -i ${IDENTITY} "ubuntu@${INST}" "/bin/bash -s" <<EOF
    pkill heist
    pkill getaway
    exit
EOF

# Copy the backend binary to the INST host using scp
scp -i ${IDENTITY} ./heist/target/aarch64-unknown-linux-gnu/release/heist "ubuntu@${INST}":/home/ubuntu/heist

# Copy the frontend binary to the INST host using scp
scp -i ${IDENTITY} ./getaway/target/aarch64-unknown-linux-gnu/release/getaway "ubuntu@${INST}":/home/ubuntu/getaway

# Log into the INST host using ssh and kill the running backend and frontend processes
# then run the new backend and frontend binaries
ssh -tt -i ${IDENTITY} "ubuntu@${INST}" "/bin/bash -s" <<EOF
    nohup /home/ubuntu/heist & sleep 1;
    nohup /home/ubuntu/getaway & sleep 1;
    exit
EOF

exit 0
