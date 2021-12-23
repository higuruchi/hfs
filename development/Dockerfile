FROM ubuntu:20.04

# WORKDIR /home/development
ENV DEBIAN_FRONTEND=noninteractive

RUN apt update -y && \
    apt install fuse -y && \
    apt install libfuse-dev -y && \
    apt install vim -y && \
    apt install curl -y && \
    apt install git -y && \
    apt install pkg-config -y tzdata && \
    apt install sudo -y && \
    apt install build-essential -y
    # apt install timedatectl -y

# RUN timedatectl set-timezone Asia/Tokyo

COPY ./config /etc/config

RUN adduser --home /home/development --shell /bin/bash --uid 1000 --disabled-login development && \
    echo "development:hfs_dev_pass" | chpasswd && \
    gpasswd -a development sudo

USER 1000
SHELL ["/bin/bash", "-l", "-c"]

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    source $HOME/.cargo/env