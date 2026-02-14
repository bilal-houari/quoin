# --- Stage 1: Get Pandoc ---
FROM pandoc/minimal:latest AS pandoc-base

# --- Stage 2: Get Typst ---
FROM ghcr.io/typst/typst:latest AS typst-base

# --- Stage 3: Build Frontend ---
FROM node:24-slim AS frontend-builder
WORKDIR /app/web
COPY web/package.json web/package-lock.json ./
RUN npm install
COPY web/ ./
RUN npm run build

# --- Stage 4: Build Backend (using rustup) ---
FROM debian:bookworm-slim AS backend-builder
RUN apt-get update && apt-get install -y \
    curl build-essential pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# Pre-build dependencies to cache layers
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src/ ./src/
# rust-embed expects web/dist/ relative to the cargo root
COPY --from=frontend-builder /app/web/dist ./web/dist
RUN cargo build --release

# --- Stage 5: Final Production Image ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Configurable Port for Docker Build
ARG PORT=3232
ENV APP_PORT=${PORT}

# Create a non-root user
RUN groupadd -r quoin && useradd -r -g quoin quoin

# We use /data as the default working directory for CLI usage
WORKDIR /data
RUN chown -R quoin:quoin /data

# Copy binaries
COPY --from=pandoc-base /usr/local/bin/pandoc /usr/local/bin/pandoc
COPY --from=typst-base /bin/typst /usr/local/bin/typst
COPY --from=backend-builder /app/target/release/quoin /usr/local/bin/quoin

USER quoin

# Expose the API port
EXPOSE ${APP_PORT}

# Use exec-form ENTRYPOINT for native signal handling (PID 1)
ENTRYPOINT ["quoin"]

# Default to server mode
CMD ["server", "--port", "3232"]
