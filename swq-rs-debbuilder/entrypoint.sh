#!/bin/bash

cd /swq-rs
git pull --force

#amd64
cargo deb

find target/ -name \*.deb -exec cp {} /debs \;
