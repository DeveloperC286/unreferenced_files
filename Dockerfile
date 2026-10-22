FROM alpine:3.24.3@sha256:1b7232cda4ca3dc20b5d1247bdc9bfb631f2661d15d323bdd6fccc035ed1a9e7

ARG TARGET
COPY ./target/${TARGET}/release/unreferenced_files /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["unreferenced_files"]
