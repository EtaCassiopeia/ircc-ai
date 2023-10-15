# Build Stage
# BUILDPLATFORM can also be used which matches the current machine which may result a faster build.
# TARGETPLATFORM is part of a set of automatically defined (global scope) build arguments that you can use.
# See https://docs.docker.com/engine/reference/builder/?_gl=1*oef9n9*_ga*MTM2MTQyMDUuMTY5NzA3NjMzMA..*_ga_XJWPQMJYHQ*MTY5NzMxMzgyOC41LjEuMTY5NzMxNDUzMy42MC4wLjA.#automatic-platform-args-in-the-global-scope for more information.
FROM --platform=$TARGETPLATFORM rust:1.73.0-bullseye AS base

ARG TARGETPLATFORM

# Install g++ and other build essentials for compiling openssl/tls dependencies
RUN apt update
RUN apt install -y build-essential

# Install openssl / native tls dependencies
RUN apt-get update
RUN apt-get install -y \
  pkg-config \
  librust-openssl-dev \
  libssl-dev

# Clean up apt artifacts
RUN rm -rf /var/lib/apt/lists/*

# Make sure you have the target setup

WORKDIR /app

# Building the binary
FROM base as builder

# Copying actual source code will overwrite the dummy lib.rs and restore the original Cargo.toml
COPY . /app/

RUN cargo build --release  --bin bot --no-default-features --features bot

# Runtime Stage: Creating a minimal image
FROM --platform=$TARGETPLATFORM debian:bullseye-slim

# Update the CA certificate bundle
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    update-ca-certificates

# Install openssl / native tls dependencies

RUN apt-get update && apt-get install -y libssl1.1

COPY --from=builder /app/target/release/bot /usr/local/bin/bot

# Setting the entrypoint to your binary
CMD ["/usr/local/bin/bot"]