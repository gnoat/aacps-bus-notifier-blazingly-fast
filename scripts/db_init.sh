#!/usr/bin/env bash
set -x
sex -env

if ! [ -x "$(command -v sqlx)" ]; then
    echo "Sqlx-cli not installed, installing now..."
    cargo install --version="~0.6" sqlx-cli --no-default-features --features rustls,postgres
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PW:=password}"
DB_NAME="${POSTGRES_DC:=contacts}"
DB_PORT="${POSTGRES_PORT:=5432}"
export PGPASSWORD="${DB_PASSWORD}"

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000

until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    echo "Postgres is still sleeping"
    sleep 1
done

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
