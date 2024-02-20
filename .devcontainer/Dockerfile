FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

RUN curl -L https://fly.io/install.sh | FLYCTL_INSTALL=/usr/local sh

RUN cargo install just
# Setup diesel cli
RUN cargo install diesel_cli
RUN apt install -y libpq-dev
