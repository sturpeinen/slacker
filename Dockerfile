FROM rust:1.41.1

RUN apt-get -y update && \
    apt-get -y install musl musl-tools

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /build
COPY . /build/

CMD cargo build --release --target="x86_64-unknown-linux-musl"
