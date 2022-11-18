#!/usr/bin/env bash
set -x
sex -env
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PW:=password}"
DB_NAME="${POSTGRES_DC:=contacts}"
DB_PORT="${POSTGRES_PORT:=5432}"

docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
