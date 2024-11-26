# Build stage
FROM rust:1.75-slim-bullseye as builder

WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install diesel CLI
RUN cargo install diesel_cli --version "2.1.1" --no-default-features --features postgres

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/web-app /app/web-app
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/src/app/migrations /app/migrations

# Create a non-root user
# RUN useradd -m appuser && chown -R appuser:appuser /app
# USER appuser

# Set environment variables
ENV RUST_LOG=info

# Run migrations and start the application
CMD ["sh", "-c", "diesel migration run && ./web-app"]