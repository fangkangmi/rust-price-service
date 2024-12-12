# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the subdirectory Cargo.toml files
COPY client/Cargo.toml client/
COPY http/Cargo.toml http/
COPY httpserver/Cargo.toml httpserver/
COPY server/Cargo.toml server/

# Create empty directories for the subprojects
RUN mkdir -p client/src http/src httpserver/src server/src

# Copy the source code
COPY client/src client/src
COPY http/src http/src
COPY httpserver/src httpserver/src
COPY server/src server/src
COPY src src

# Build the project
RUN cargo build -p httpserver

# Expose port 7070
EXPOSE 7070
EXPOSE 8081

# Run the application
CMD ["cargo", "run", "-p", "httpserver"]