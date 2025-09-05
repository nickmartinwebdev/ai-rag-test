# Build stage
FROM rust:1.82 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates for HTTPS requests
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/ai-rag-test /app/ai-rag-test

# Change ownership to app user
RUN chown appuser:appuser /app/ai-rag-test

USER appuser

# Expose port (adjust as needed)
EXPOSE 8080

# Run the binary
CMD ["./ai-rag-test"]
