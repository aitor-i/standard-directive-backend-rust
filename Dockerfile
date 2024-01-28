# Start by building from a Rust image
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rustapp
WORKDIR /rustapp

# Copy your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build

# Now that the dependencies are built, copy your source code
COPY ./src ./src


# Set the CMD to your binary
CMD ["cargo", "run"]
