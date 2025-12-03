FROM alpine:3.23.0@sha256:51183f2cfa6320055da30872f211093f9ff1d3cf06f39a0bdb212314c5dc7375

COPY ./target/x86_64-unknown-linux-musl/release/unreferenced_files /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["unreferenced_files"]
