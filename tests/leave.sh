#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "LeaveServer",
    "data": {
        "user_id": 2,
        "server_id": 3
    }
}'
