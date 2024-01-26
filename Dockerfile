
# Use a Rust base image
FROM rust:1.65 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rust_rest_api
WORKDIR /rust_rest_api

# Copy your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/rust_rest_api*
RUN cargo build --release

# Final base image
FROM debian:buster-slim

# Copy the build artifact from the build stage
COPY --from=builder /rust_rest_api/target/release/rust_rest_api .

# Set the startup command to run your binary
CMD ["./rust_rest_api"]
