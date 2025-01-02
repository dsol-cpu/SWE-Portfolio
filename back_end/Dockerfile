FROM rust:latest AS builder

# Create appuser
ENV USER=web
ENV UID=1001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

# Copy manifests from the back_end directory
COPY back_end/Cargo.lock back_end/Cargo.toml ./

# Cache dependencies
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Copy the actual source code
COPY back_end/src src/

# Build for release
RUN touch src/main.rs && cargo build --release

# Final stage
FROM debian:bookworm-slim

# Install OpenSSL
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*

# Import from builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy the build artifact from builder
COPY --from=builder /app/target/release/back_end ./

# Use an unprivileged user
USER web:web

# Expose port
EXPOSE 8080

# Run the service
CMD ["./back_end"]