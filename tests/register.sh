#!/bin/bash

curl -X PUT "http://localhost:4001/api/users" \
    -H "Content-Type: application/json" \
    -d '{
            "username": "JohnDoe123",
            "password": "JohnDoeIsCool",
            "first_name": "John",
            "last_name": "Doe",
            "email": "johndoe123@test.dev"
        }'
