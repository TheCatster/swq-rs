#!/bin/bash
docker build -t swq-rs-builder . || exit

docker run -v "$(pwd)/debs:/debs" swq-rs-builder
