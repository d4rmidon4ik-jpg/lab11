#!/usr/bin/env bash
set -e

# Повышенное задание 4 — кросс-платформенная сборка через docker buildx

# Создаём builder с поддержкой мультиплатформ (если ещё нет)
docker buildx inspect lab11-builder > /dev/null 2>&1 || \
    docker buildx create --name lab11-builder --use

docker buildx use lab11-builder
docker buildx inspect --bootstrap

echo "Building go-service for linux/amd64 and linux/arm64..."
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    --tag go-service:latest \
    ./go-service

echo "Building python-service for linux/amd64 and linux/arm64..."
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    --tag python-service:latest \
    ./python-service

echo "Building rust-service for linux/amd64 and linux/arm64..."
docker buildx build \
    --platform linux/amd64,linux/arm64 \
    --tag rust-service:latest \
    ./rust-service

echo "All images built successfully for amd64 and arm64."