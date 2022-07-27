#!/bin/bash
#/usr/local/bin/docker-compose build
docker-compose build 
docker run -d -p 5000:5000 --restart=always --name registry registry:2
sleep 5s
docker push localhost:5000/ciel
sleep 5s
docker run ciel

