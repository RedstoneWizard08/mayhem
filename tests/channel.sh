#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "CreateChannel",
    "data": {
        "name": "general",
        "server_id": 3,
        "channel_type": "text"
    }
}'
