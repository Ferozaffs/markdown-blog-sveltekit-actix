# Use the official Rust image as the base image
FROM rust:latest

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock dummy.rs ./

#  Dummy swap to cache dependencies
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

# Swap back
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY . .
RUN cargo build --release

# Expose the port that your Rust backend will run on
EXPOSE 8080

# Run your Rust backend application
CMD ["target/release/md_backend"]