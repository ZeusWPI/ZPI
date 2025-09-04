FROM rust:1.88-alpine AS chef

WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev
RUN cargo install cargo-chef --locked

# ------------------------------------------------------------------------
FROM chef AS planner

COPY Cargo.lock Cargo.toml ./
COPY database/Cargo.toml ./database/
RUN cargo chef prepare --recipe-path recipe.json

# ------------------------------------------------------------------------
FROM chef AS builder

# build dependencies
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# build app
COPY Cargo.lock Cargo.toml ./
COPY migrations ./migrations
COPY database ./database
COPY src ./src
RUN cargo build --release

# ------------------------------------------------------------------------
FROM alpine:latest

WORKDIR /app
ENV MAGICK_PATH=/usr/bin/magick

RUN apk add --no-cache openssl imagemagick libwebp imagemagick-jpeg

COPY --from=builder /app/target/release/zpi /usr/local/bin

CMD ["/usr/local/bin/zpi"]
