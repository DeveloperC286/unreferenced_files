FROM alpine:3.24.1@sha256:28bd5fe8b56d1bd048e5babf5b10710ebe0bae67db86916198a6eec434943f8b

ARG TARGET
COPY ./target/${TARGET}/release/unreferenced_files /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["unreferenced_files"]
