#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "CreateServer",
    "data": {
        "name": "Test"
    }
}'
