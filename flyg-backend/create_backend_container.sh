#!/bin/bash
#VERSION=$(cat Cargo.toml | head -n5 | grep version | awk '{ print $3 }' | sed 's/"//g')
VERSION=latest
docker build -t flying7eleven/flyg_backend:$VERSION .
