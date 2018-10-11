# Build image
FROM rustlang/rust:nightly as build

WORKDIR /usr/src/security-gate

# Copy manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy source tree
COPY ./src ./src

# Build for release
RUN cargo build --release

# Final image
FROM debian:stable-slim

# Copy the binaries
WORKDIR /usr/src/
COPY --from=build /usr/src/security-gate/target/release/security-gate .

# Copy static
COPY ./static ./static

# Set the startup command to run the binary
CMD ["./security-gate", "-v", "-v"]