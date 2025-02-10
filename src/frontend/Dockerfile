

# Build stage
FROM rust:1.83 AS builder

RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    pkg-config \
    libssl-dev

RUN cargo install trunk
RUN cargo install trunk wasm-bindgen-cli --version 0.2.100
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY . .
RUN trunk build --release

# Serve stage
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf