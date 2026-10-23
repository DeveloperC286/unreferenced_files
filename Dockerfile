FROM alpine:3.24.3@sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a

ARG TARGET
COPY ./target/${TARGET}/release/unreferenced_files /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["unreferenced_files"]
