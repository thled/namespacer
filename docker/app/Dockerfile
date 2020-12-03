FROM rust:1.48-alpine3.12

ENV USER=rust

RUN adduser -D rust
USER rust

RUN rustup component add rustfmt

WORKDIR /usr/src/app

