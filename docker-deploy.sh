
docker build . --file Dockerfile --tag axmouth/axmouth.dev-backend
docker-compose build
docker-compose up -d --remove-orphans