#!/bin/bash

curl -X PUT https://dev-backend.kadaroja.com/api/users -H "Content-Type: application/json" \
    -d '{
        "first_name": "John",
        "last_name": "Doe",
        "email": "john.doe@example.com",
        "username": "JohnDoe",
        "password": "JohnDoe123"        
    }'
