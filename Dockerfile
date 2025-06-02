
FROM rust:1-slim-bullseye AS builder
WORKDIR /code

# Print Rust version for reference
RUN rustc --version

# Download crates-io index and fetch dependency code.
# This step avoids needing to spend time on every build 
# downloading the index, which can take a long time within
# the docker context. Docker will cache it.

RUN USER=root cargo init
COPY Cargo.toml Cargo.toml
RUN cargo fetch

# copy app files
COPY src src

## Build the app
RUN cargo build --release

## Production stage
FROM debian:bullseye-slim
WORKDIR /app

# Copy the build artifacts
COPY --from=builder /code/target/release/spot-feed spot-feed

# Run as non-root user for security reasons 
RUN groupadd -r appuser && useradd -r -g appuser appuser
RUN chown -R appuser:appuser /app

# Switch to the appuser
USER appuser

# Expose the port the app runs on
EXPOSE 8080

# Run the app
CMD [ "/app/spot-feed" ]
