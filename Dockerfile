# Build and Runtime Stage with the Same Base Image
FROM rust:1.75-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
RUN cargo build --release

# Use the same base image for runtime
FROM rust:1.75-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/openchess-rs /app/openchess-rs
EXPOSE 8080
CMD ["./openchess-rs"]
