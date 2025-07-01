#!/usr/bin/env bash

export PATH="$(eval echo ~$USER)/.cargo/bin:$PATH"

# Drop the database
docker compose -f sql-compose.yaml down --volumes
docker compose -f sql-compose.yaml up -d
sqlx migrate run