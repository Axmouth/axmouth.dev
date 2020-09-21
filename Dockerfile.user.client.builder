FROM node:14 as builder

COPY ./ng-client-user/package*.json ./

RUN npm set progress=false && npm config set depth 0 && npm cache clean --force

## Storing node modules on a separate layer will prevent unnecessary npm installs at each build
RUN npm i && mkdir /ng-app && cp -R ./node_modules ./ng-app

WORKDIR /ng-app

COPY ./ng-client-user .

## Build the angular app in production mode and store the artifacts in dist folder
#RUN $(npm bin)/ng build --prod --build-optimizer
RUN npm run build:ssr