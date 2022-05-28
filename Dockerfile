FROM rust:1.61.0

# Set working directory in container
RUN mkdir /usr/src/randomapi
WORKDIR /usr/src/randomapi

# Copy all source code file from local computer to container
COPY src src
COPY Cargo.toml .

# Build release application
RUN cargo build --release

# Expose listening port for application
EXPOSE 8080

# Run the application
CMD ["target/release/restapiwebservice"]