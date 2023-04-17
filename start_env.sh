#!/bin/bash

set -e

if [ -z "$(docker images | grep rust-win)" ]; then
  docker build -t rust-win:1.68 -f Dockerfile-rust-win64 .
fi 

docker run --rm -it -v $(pwd):/app/ -v rust-target:/app/target rust-win:1.68 /bin/bash
