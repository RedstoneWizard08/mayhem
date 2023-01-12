#!/bin/bash

wscat -c wss://dev-websocket.kadaroja.com/ws -x \
    '{
        "action": "SendMessage",
        "data": {
            "content": "Test Message",
            "timestamp": "Wed Jan 11 2023 12:57:01 GMT-0800 (Pacific Standard Time)",
            "sender": 1,
            "channel": 2
        }
    }'
