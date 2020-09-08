
docker build . --file Dockerfile --tag ghcr.io/axmouth/axmouth.dev-backend
docker-compose build
docker-compose up -d --remove-orphans