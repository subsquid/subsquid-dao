FROM rustlang/rust:nightly-bullseye-slim

ENV RUST_BACKTRACE 1

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        clang \
        git \
        curl \
        libssl-dev \ 
        llvm \ 
        libudev-dev \
        libssl-dev \
        pkg-config \        
        binaryen \
        ; \
    \
    rustup component add rust-src; \
    rustup target add wasm32-unknown-unknown; \
    rustup show; \
    cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --rev 8d91b8e --force --locked; \
    apt-get remove -y --auto-remove \
        git \
        ; \
    rm -rf /var/lib/apt/lists/*;

    # check if executable works in this container
    RUN /usr/local/cargo/bin/substrate-contracts-node --version

    EXPOSE 30333 9933 9944

    ENTRYPOINT ["/usr/local/cargo/bin/substrate-contracts-node"]