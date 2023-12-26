FROM rust:1.74.01 as builder
WORKDIR /usr/src/scones-ie
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/scones-ie /usr/local/scones-ie
CMD ["myapp"]