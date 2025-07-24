#!/usr/bin/env bash

export PATH="$(eval echo ~$USER)/.cargo/bin:$PATH"

set -e

# Clean and restart containers
docker compose -f sql-compose.yaml down -v
docker compose -f sql-compose.yaml up -d

# Wait for PostgreSQL to be ready (using container healthcheck)
echo "Waiting for Postgres container to be healthy..."
until [ "$(docker inspect -f '{{.State.Health.Status}}' my_postgres)" == "healthy" ]; do
  sleep 1
done

echo "Postgres is ready. Running migrations..."
sqlx migrate run

echo "All services are up! ✅"
