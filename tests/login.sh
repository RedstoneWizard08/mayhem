#!/bin/bash

curl -X POST "http://localhost:4001/api/users" -H "Content-Type: application/json" -d '{ "username": "JohnDoe123", "password": "JohnDoeIsCool" }'
