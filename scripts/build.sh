#!/bin/bash

DIR="$(realpath "$(dirname "$0")")"

"$DIR/build-client.sh"
"$DIR/build-server.sh"
