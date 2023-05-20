#!/bin/bash

DIR="$(realpath "$(dirname "$0")")"

if ! [ -x "$(command -v concurrently)" ]; then
    if ! [ -x "$(command -v bun)" ]; then
        echo "Error: bun is not installed." >&2
        exit 1
    fi

    bun add -g concurrently
fi

concurrently --restart-tries -1 --raw "$DIR/server-dev.sh" "$DIR/client-dev.sh"
