#! /bin/bash -eu

PROJ_PATH=$(readlink -f $(cd $(dirname $(readlink -f $0)) && pwd))
cd ${PROJ_PATH}/..

mkdir -p artifacts

docker-compose build
docker-compose up -d
docker cp cidr-chef:/cidr-chef ${PWD}/artifacts/cidr-chef
docker-compose down
