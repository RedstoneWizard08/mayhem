#!/bin/bash

wscat -c "ws://localhost:4001/api/ws" -x '{
    "action": "GetChannelsInServer",
    "data": 3
}'
