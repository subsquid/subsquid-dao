FROM rustlang/rust:nightly-bullseye-slim

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
    cargo install --force cargo-make; \
    cargo install cargo-contract --vers ^0.15 --force --locked; \
    apt-get remove -y --auto-remove \
        git \
        ; \
    rm -rf /var/lib/apt/lists/*; \
    chmod -R ugo+rwx /usr/local/cargo;