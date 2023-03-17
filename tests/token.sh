#!/bin/bash

curl -X POST "http://localhost:4001/api/token" -H "Content-Type: application/json" \
    -d '{
            "username": "JohnDoe123",
            "password": "JohnDoeIsCool"
        }'
