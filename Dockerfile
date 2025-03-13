# Planner stage
FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef

# Switch to working directory `app`
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y

FROM chef AS planner
# Copy all files from working environment to the Docker image
COPY . .
# Compute a lock-like file for the project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build the project dependencies only, not the application
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same, all layers will be cached
COPY . .
# Set sqlx to run offline
ENV SQLX_OFFLINE=true
# Build the project
RUN cargo build --release --bin zero2prod

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder environment to the runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime
COPY configuration configuration
# Set the environment variable to `production`
ENV APP_ENVIRONMENT=production
# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./zero2prod" ]