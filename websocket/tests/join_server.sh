#!/bin/bash

wscat -c wss://dev-websocket.kadaroja.com/ws -x \
    '{
        "action": "JoinServer",
        "data": {
            "user_id": 1,
            "server_id": 1
        }
    }'
