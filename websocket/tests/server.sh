#!/bin/bash

wscat -c wss://dev-websocket.kadaroja.com/ws -x \
    '{
        "action": "CreateServer",
        "data": {
            "name": "Test Server"
        }
    }'
