FROM rust:1.83

WORKDIR /app

COPY Cargo.toml ./

COPY . .

RUN cargo build

EXPOSE 6000

CMD ["cargo", "run"]