#!/bin/bash

DIR="$(realpath "$(dirname "$0")")"

if ! [ -x "$(command -v concurrently)" ]; then
    if ! [ -x "$(command -v npm)" ]; then
        echo "Error: npm is not installed." >&2
        exit 1
    fi

    npm install --global concurrently
fi

concurrently --raw "$DIR/server-dev.sh" "$DIR/client-dev.sh"
