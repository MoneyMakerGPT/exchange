#!/bin/bash
docker compose -f docker-compose-core.yml down
docker compose -f docker-compose.yml down

docker compose -f docker-compose.yml up -d --build
docker compose -f docker-compose-core.yml up --build
