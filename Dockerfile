FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bullseye-slim

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/quicksys /app/quicksys

# Set the entrypoint
ENTRYPOINT ["/app/quicksys"]

# Default command
CMD ["--pretty"]