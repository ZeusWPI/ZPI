FROM rust:1.88-alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /usr/src/zpi

COPY Cargo.toml Cargo.lock ./

# cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src .cargo/

COPY ./static ./static
COPY ./src ./src

# make cargo detect new files
RUN touch ./src/main.rs
RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache openssl

COPY --from=builder /usr/src/zpi/target/release/zpi /usr/local/bin/

CMD ["/usr/local/bin/zpi"]
