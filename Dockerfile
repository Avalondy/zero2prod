FROM rust:1.85.0

# Switch to working directory `app`
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt-get update && apt-get install lld clang -y
# Copy all files from working environment to the Docker image
COPY . .
# Set sqlx to run offline
ENV SQLX_OFFLINE=true
# Build the project
RUN cargo build --release
# When `docker run` is executed, launch the binary
ENTRYPOINT [ "./target/release/zero2prod" ]