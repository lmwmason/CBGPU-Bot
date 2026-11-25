FROM rust:1-bookworm AS build
WORKDIR /src
# Build dependencies in their own layer so code-only changes don't recompile them.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && cargo clean --release -p CBGPU-Bot \
    && rm -rf src
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=build /src/target/release/CBGPU-Bot /usr/local/bin/cbgpu-bot
VOLUME /app/data
CMD ["cbgpu-bot"]
