#!/bin/bash
#docker-compose build
docker-compose up --build
#docker rm registry -f
# We need registry when kubernetes has to be provided images
#docker run -d -p 5000:5000 --restart=always --name registry registry:2
#sleep 5s
#docker push localhost:5000/ciel
#sleep 5s
#docker run ciel
#docker-compose down --rmi all

