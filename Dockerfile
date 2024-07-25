# Use the official Rust image as the build environment
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY ./src ./src
COPY ./data ./data

# Build the project
RUN cargo install --path .

CMD ["ro_rust_api"]

# Use the official Debian slim image as the runtime environment
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the build environment
COPY --from=builder /usr/local/cargo/bin/ro_rust_api /usr/local/bin/ro_rust_api
COPY --from=builder /usr/src/app/data /usr/src/app/data

# Expose the port the app runs on
EXPOSE 8000

# Run the application
CMD ["ro_rust_api"]
