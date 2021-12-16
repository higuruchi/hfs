FROM rust AS builder

WORKDIR /src

COPY ./ /src

RUN apt update -y && \
    apt install fuse -y && \
    apt install libfuse-dev -y

RUN cargo build

# ------------------------------------

FROM ubuntu:20.04

RUN apt update  -y && \
    apt install fuse  -y && \
    apt install libfuse-dev -y

COPY --from=builder /src/target/debug/hfs /app/hfs
COPY --from=builder /src/image.yaml /etc/image.yaml

WORKDIR /app

CMD ["/app/hfs", "--config-path", "/etc/image.yaml"]