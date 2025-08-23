#!/usr/bin/env bash

set -e
source .env
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
mkdir -p "$BACKUP_PATH"
pg_dump "$DATABASE_URL" | gzip > "$BACKUP_PATH/libro_$TIMESTAMP.sql.gz"
