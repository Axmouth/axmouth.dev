docker build . --file Dockerfile.user.client.builder --tag ghcr.io/axmouth/axmouth.dev-user-client-builder
docker build . --file Dockerfile.user.client.final --tag ghcr.io/axmouth/axmouth.dev-user-client
docker build . --file Dockerfile.api.builder --tag ghcr.io/axmouth/axmouth.dev-backend-builder
docker build . --file Dockerfile.api.final --tag ghcr.io/axmouth/axmouth.dev-backend
docker-compose build
docker-compose up -d --remove-orphans