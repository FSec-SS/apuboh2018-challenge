#!/bin/bash

PORT=8081
CONTAINER_NAME=web1

rm nginx_root/flag_is_here/*
python gen_data.py
CONTAINER_ID=$(docker run -d --name $CONTAINER_NAME -p $PORT:80 -v $(pwd)/nginx_root:/usr/share/nginx/html -it nginx)
docker cp nginx.conf $CONTAINER_ID:/etc/nginx/nginx.conf
docker exec $CONTAINER_ID nginx -s reload
