FROM rust:1.96-slim AS builder
WORKDIR /app

RUN mkdir src && echo "fn main() {}" > src/main.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN touch src/main.rs 

RUN cargo build --release