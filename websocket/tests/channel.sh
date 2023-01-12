#!/bin/bash

wscat -c wss://dev-websocket.kadaroja.com/ws -x \
    '{
        "action": "CreateChannel",
        "data": {
            "name": "Test Channel",
            "server_id": 1,
            "channel_type": "text"
        }
    }'
