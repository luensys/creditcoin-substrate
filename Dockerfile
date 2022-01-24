FROM ubuntu:latest AS builder
ENV DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-c"]
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    git \
    build-essential \
    clang \
    libclang-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN source ~/.cargo/env && rustup default stable && rustup update nightly && rustup update stable && rustup target add wasm32-unknown-unknown --toolchain nightly
WORKDIR /creditcoin-node
COPY . /creditcoin-node
RUN source ~/.cargo/env && cargo build --release

FROM ubuntu:latest
EXPOSE 30333/tcp
EXPOSE 30333/udp
EXPOSE 9944
EXPOSE 9933
COPY --from=builder /creditcoin-node/target/release/creditcoin-node /bin/creditcoin-node
COPY --from=builder /creditcoin-node/testnetSpec.json /chainspec/testnetSpec.json