docker build . --file Dockerfile.builder --tag ghcr.io/axmouth/axmouth.dev-backend-builder
docker build . --file Dockerfile.final --tag ghcr.io/axmouth/axmouth.dev-backend
docker-compose build
docker-compose up -d --remove-orphans