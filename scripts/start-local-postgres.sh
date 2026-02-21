#!/usr/bin/env bash
set -euo pipefail

IMAGE="${IMAGE:-postgres:16}"
CONTAINER_NAME="${CONTAINER_NAME:-good-times-postgres}"
VOLUME_NAME="${VOLUME_NAME:-good_times_pg_data}"
PG_PORT="${PG_PORT:-5432}"

POSTGRES_DB="${POSTGRES_DB:-poll}"
POSTGRES_USER="${POSTGRES_USER:-poll}"
POSTGRES_PASSWORD="${POSTGRES_PASSWORD:-poll}"

if docker ps -a --format '{{.Names}}' | grep -qx "${CONTAINER_NAME}"; then
  docker start "${CONTAINER_NAME}" >/dev/null
else
  docker volume create "${VOLUME_NAME}" >/dev/null
  docker run -d \
    --name "${CONTAINER_NAME}" \
    -e POSTGRES_DB="${POSTGRES_DB}" \
    -e POSTGRES_USER="${POSTGRES_USER}" \
    -e POSTGRES_PASSWORD="${POSTGRES_PASSWORD}" \
    -p "${PG_PORT}:5432" \
    -v "${VOLUME_NAME}:/var/lib/postgresql/data" \
    "${IMAGE}" >/dev/null
fi

cat <<EOF
Postgres is running.
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${PG_PORT}/${POSTGRES_DB}
EOF

ENV_FILE="poll-backend/.env"
cat > "${ENV_FILE}" <<EOF
DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${PG_PORT}/${POSTGRES_DB}
EOF

echo "Wrote ${ENV_FILE}"
