# syntax = docker/dockerfile:1.4

# Copied from https://fasterthanli.me/series/building-a-rust-service-with-nix/part-6
# to speed up development

################################################################################
FROM ubuntu:22.04 AS base

################################################################################
FROM base AS builder

# Install compile-time dependencies
RUN set -eux; \
		apt update; \
		apt install -y --no-install-recommends \
			curl ca-certificates gcc libc6-dev pkg-config libssl-dev libpq-dev \
			;

# Install rustup
RUN set -eux; \
		curl --location --fail \
			"https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init" \
			--output rustup-init; \
		chmod +x rustup-init; \
		./rustup-init -y --no-modify-path --default-toolchain stable; \
		rm rustup-init;

# Add rustup to path, check that it works
ENV PATH=${PATH}:/root/.cargo/bin
RUN set -eux; \
		rustup --version;

# Copy sources and build them
WORKDIR /app
COPY src src
COPY migrations migrations
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/root/.rustup \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/root/.cargo/git \
		--mount=type=cache,target=/app/target \
		set -eux; \
        rustup default stable; \
		cargo build --release; \
		cp target/release/scones .

################################################################################
FROM base AS app

# Install run-time dependencies, remove extra APT files afterwards.
# This must be done in the same `RUN` command, otherwise it doesn't help
# to reduce the image size.
RUN set -eux; \
		apt update; \
		apt install -y --no-install-recommends \
			ca-certificates \
			libpq-dev libpq5 \
			; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/

# Copy app from builder
WORKDIR /app
COPY --from=builder /app/scones .
COPY static /app/static

CMD ["/app/scones"]