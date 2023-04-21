#!/bin/bash

docker run -it --rm \
    --network mayhem_mayhem_net \
    -e HOST=0.0.0.0 \
    -e PORT=4000 \
    -e DATABASE_USER=mayhem \
    -e DATABASE_PASS=mayhem \
    -e DATABASE_HOST=database \
    -e DATABASE_PORT=5432 \
    -e DATABASE_NAME=mayhem \
    -p 4000:4000 \
    -p 4001:4001 \
    ghcr.io/redstonewizard08/mayhem:latest \
    bash
