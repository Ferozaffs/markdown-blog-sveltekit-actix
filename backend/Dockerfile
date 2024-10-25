FROM rust:latest

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./backend/dummy.rs ./backend/
COPY ./shared ./shared

WORKDIR /app/backend
#  Dummy swap to cache dependencies
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

# Swap back
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
COPY ./backend ./

RUN cargo build --release

# Expose the port that your Rust backend will run on
EXPOSE 8080

# Run your Rust backend application
CMD ["target/release/md_backend"]