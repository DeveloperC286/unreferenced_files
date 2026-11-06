FROM alpine:3.24.4@sha256:93b41ef40b1677fce8fcc64ff4af09daddfacf5040afde618bc46c1e2d1cc7d4

ARG TARGET
COPY ./target/${TARGET}/release/unreferenced_files /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["unreferenced_files"]
