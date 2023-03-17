#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "SendMessage",
    "data": {
        "content": "AAA",
        "timestamp": "0",
        "sender": 1,
        "channel": 4
    }
}'
