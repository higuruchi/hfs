FROM rust AS builder
# FROM ubuntu AS builder

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs --profile default | sh

# COPY ./Cargo.toml /src/Cargo.toml

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

CMD ["/app/hfs" "--config-path", "/etc/image.yaml"]