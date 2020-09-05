#!/bin/sh

mkdir -p $STATIC_FILE_DIR
chown -R $APP_USER:$APP_USER $STATIC_FILE_DIR
exec runuser -u $APP_USER "$@"