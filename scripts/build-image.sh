#!/bin/bash

docker buildx build \
    -t ghcr.io/redstonewizard08/mayhem:latest \
    --load \
    "$(pwd)"
