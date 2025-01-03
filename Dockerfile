# Stage 1: Build the Rust application
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/pentheus

# Install necessary packages for building and database tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install the pentheus crate
RUN cargo install pentheus --version 0.1.1

# Stage 2: Create the final image
FROM ubuntu:oracular-20241120

# Install necessary libraries and database tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    mysql-client \
    postgresql-client \
    && rm -rf /var/lib/apt/lists/*

# Copy the installed binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/pentheus /usr/local/bin/pentheus

# Set the command to run the pentheus binary by default
CMD ["pentheus"]
