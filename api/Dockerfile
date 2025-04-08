FROM rust:1.83-slim-bookworm as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libpq-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY Cargo.toml ./
COPY Cargo.lock ./

# Copy the .sqlx directory for offline mode
COPY .sqlx ./.sqlx

# Set SQLX_OFFLINE environment variable to true
ENV SQLX_OFFLINE=true

# Create a dummy main.rs to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code
COPY . .

# Build the application with SQLx in offline mode
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev libpq-dev ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/api /app/api

# Create a .env file only if environment variables aren't provided
RUN touch .env && \
    echo "DATABASE_URL=\${DATABASE_URL:-postgres://postgres:postgres@db:5432/pbtar}" > .env && \
    echo "RUST_LOG=\${RUST_LOG:-info}" >> .env

EXPOSE 8080

CMD ["/app/api"]