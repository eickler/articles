# Note: This Docker configuration does not work yet. The container will start but the process will crash during startup with exit code 139.

FROM rust:latest as builder
WORKDIR /usr/src

# 1a: Prepare for static linking
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y musl-tools && \
    rustup default nightly && \
    rustup target add x86_64-unknown-linux-musl

# 1b: Download and compile Rust dependencies (and store as a separate Docker layer)
RUN USER=root cargo new articles
WORKDIR /usr/src/articles
COPY Cargo.toml Cargo.lock ./
RUN cargo build --target x86_64-unknown-linux-musl # --release

# 1c: Build the exe using the actual source code
COPY src/ ./src/
COPY diesel.toml ./
RUN touch src/main.rs && cargo build --target x86_64-unknown-linux-musl # --release

# 2: Copy the exe and extra files ("static") to an empty Docker image
FROM scratch
COPY --from=builder /usr/src/articles/target/x86_64-unknown-linux-musl/debug/articles .
#COPY --from=builder /usr/src/articles/target/x86_64-unknown-linux-musl/release/articles .
USER 1000
EXPOSE 8000
CMD ["./articles"]
