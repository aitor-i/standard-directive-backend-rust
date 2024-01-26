# Builder stage
FROM rust:1.75 as builder

WORKDIR /usr/src/app

# Copy the source code and manifest files
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./.env ./.env

# Build the application
RUN cargo build --release

# Final base image
FROM rust:1.75

# Copy the build artifact from the builder stage
COPY --from=builder /usr/src/app/target/release/standard-directive-n2-backend .

# Set the startup command to run your binary
CMD ["./standard-directive-n2-backend"]
