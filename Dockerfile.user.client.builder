FROM node:14 as builder

RUN mkdir -p /app
WORKDIR /app
COPY ./ng-client-user/package*.json /app/

# RUN npm set progress=false && npm config set depth 0 && npm cache clean --force

## Storing node modules on a separate layer will prevent unnecessary npm installs at each build
RUN npm i

COPY ./ng-client-user .

## Build the angular app in production mode and store the artifacts in dist folder
# RUN npm run prerender
RUN npm run build:ssr