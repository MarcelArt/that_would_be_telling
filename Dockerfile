# Use the official Rust image as build environment
FROM rust:slim AS builder

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Copy proc macro crate
COPY crud_derive ./crud_derive

# Copy .env file
# COPY .env ./.env

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

# Set working directory
WORKDIR /root/

# Copy .env file
# COPY .env ./.env

# Copy the binary from builder stage
COPY --from=builder /app/target/release/that_would_be_telling .

# Expose port (adjust as needed)
EXPOSE 8080

# Run the application
CMD ["./that_would_be_telling", "prod"]