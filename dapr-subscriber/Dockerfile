ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

FROM ${BASE_IMAGE} AS base

ENV USER=rust
WORKDIR /code
RUN cargo init
COPY --chown=rust:rust Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src
COPY build.rs /code/build.rs
COPY proto /code/proto

FROM base AS builder

RUN cargo build --release

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /code/target/x86_64-unknown-linux-musl/release/dapr-subscriber \
    /usr/local/bin/

EXPOSE 50010 8088

ENTRYPOINT [ "/usr/local/bin/dapr-subscriber" ]