#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "SendMessage",
    "data": {
        "content": "AAA",
        "timestamp": "0",
        "sender": 2,
        "channel": 4
    }
}'
