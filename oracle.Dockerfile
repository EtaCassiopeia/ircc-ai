# syntax=docker/dockerfile:1

# Build Stage
FROM --platform=$TARGETPLATFORM rust:1.71-slim-bullseye AS builder

ARG TARGETPLATFORM

# Install g++ and other build essentials for compiling openssl/tls dependencies
RUN apt update
RUN apt install -y build-essential

# Install openssl / native tls dependencies
RUN apt-get update
RUN apt-get install -y \
  pkg-config \
  libssl-dev

# Clean up apt artifacts
RUN rm -rf /var/lib/apt/lists/*

# Build the oracle binary
WORKDIR /ircc-ai-build
COPY . .
RUN cargo build --release --bin oracle --features oracle

# Runtime Stage
# Prepare the runtime image
FROM debian:bullseye-slim

# Copy the built binary and required files
COPY --from=builder /ircc-ai-build/target/release/oracle /usr/local/bin/oracle
COPY --from=builder /ircc-ai-build/target/release/libonnxruntime.so /usr/local/lib/libonnxruntime.so
COPY model /model

# Set the library path for libonnxruntime.so
ENV ORT_DYLIB_PATH=/usr/local/lib/libonnxruntime.so

# Set the entrypoint to your oracle binary
ENTRYPOINT ["/usr/local/bin/oracle"]

# Provide a default value for the --path argument
CMD ["--path", "/content"]
