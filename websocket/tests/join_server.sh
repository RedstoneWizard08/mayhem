#!/bin/bash

wscat -c wss://dev-websocket.kadaroja.com/ws -x \
    '{
        "action": "JoinServer",
        "data": {
            "user_id": q,
            "server_id": 0
        }
    }'
