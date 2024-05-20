# Use the official Rust image.
# This is a multi-stage build. In the first stage, we build the binary.
FROM rust:latest as builder

# Create a new empty shell project
RUN USER=root cargo new --bin myapp
WORKDIR /final

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This trick will cache our dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now copy your source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/final_project*
RUN cargo build --release

# Final stage
# This time we use a Debian slim image to reduce the size
FROM debian:bookworm-slim

# Copy the build artifact from the builder stage.
COPY --from=builder /final/target/release/final_project /usr/local/bin/final

# Set the startup command to run your binary.
CMD ["final"]
