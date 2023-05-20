#!/bin/bash

cargo watch -x "run --bin server" -i "$(pwd)/src" -i "$(pwd)/apps/client"
