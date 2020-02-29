#!/bin/sh

set -ex
umask 0077

docker build -t slacker -f Dockerfile .

CIDFILE=$(mktemp -u)
docker run -ti --cidfile "${CIDFILE}" slacker

[ ! -d build ] && mkdir -p build
docker cp "$(cat "${CIDFILE}"):/build/target/x86_64-unknown-linux-musl/release/slacker" "build/slacker"
docker rm "$(cat "${CIDFILE}")"
rm -f "${CIDFILE}"
