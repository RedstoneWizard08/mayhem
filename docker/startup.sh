#!/bin/bash
# shellcheck disable=2129

M_HOST="${HOST:-0.0.0.0}"
M_PORT="${PORT:-4000}"

DB_USER="${DATABASE_USER:-mayhem}"
DB_PASS="${DATABASE_PASSWORD:-mayhem}"

DB_HOST="${DATABASE_HOST:-127.0.0.1}"
DB_PORT="${DATABASE_PORT:-5432}"

DB_NAME="${DATABASE:-mayhem}"

echo "host = \"$M_HOST\"" > /app/Mayhem.toml
echo "port = $M_PORT" >> /app/Mayhem.toml

echo "[database]" >> /app/Mayhem.toml
echo "protocol = \"postgresql\"" >> /app/Mayhem.toml
echo "host = \"$DB_HOST\"" >> /app/Mayhem.toml
echo "port = $DB_PORT" >> /app/Mayhem.toml
echo "user = \"$DB_USER\"" >> /app/Mayhem.toml
echo "pass = \"$DB_PASS\"" >> /app/Mayhem.toml
echo "database = \"$DB_NAME\"" >> /app/Mayhem.toml
echo "min_connections = 64" >> /app/Mayhem.toml
echo "max_connections = 1024" >> /app/Mayhem.toml
echo "connect_timeout = 5" >> /app/Mayhem.toml
echo "idle_timeout = 120" >> /app/Mayhem.toml

echo "[redis]" >> /app/Mayhem.toml
echo "host = \"127.0.0.1\"" >> /app/Mayhem.toml
echo "port = 6379" >> /app/Mayhem.toml

/app/mayhem
