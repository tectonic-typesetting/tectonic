#!/bin/sh

image=$1

docker build -t $image docker/ -f docker/$image/Dockerfile
