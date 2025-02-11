

# Build stage
FROM rust:1.83 AS builder

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    pkg-config \
    libssl-dev

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo install wasm-bindgen-cli --version 0.2.100
RUN cargo binstall trunk --version 0.21.5
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .
# RUN trunk build --release --skip-version-check

# Serve stage
CMD trunk serve