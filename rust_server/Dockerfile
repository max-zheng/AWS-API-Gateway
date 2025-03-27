# Build stage
FROM rust:1.85

# Set the working directory inside the container
WORKDIR /rust_server

COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src

# Build the application in release mode
RUN cargo install --path .

EXPOSE 3000

CMD ["rust_server"]