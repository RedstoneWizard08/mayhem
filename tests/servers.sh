#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "GetServersForUser",
    "data": 2
}'
