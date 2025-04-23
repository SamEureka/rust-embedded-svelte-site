# Stage 1: Build the Svelte app
FROM node:18 AS svelte-builder
WORKDIR /usr/src/app

# Copy the entire frontend directory
COPY frontend/ ./frontend/

# Install dependencies for the frontend
WORKDIR /usr/src/app/frontend
RUN npm install

# Build the Svelte app
RUN npm run build

# Stage 2: Build the Rust backend
FROM rust:latest AS rust-builder

# Install MUSL target for static linking
# RUN rustup target add x86_64-unknown-linux-musl

# Copy the built frontend from the svelte-builder stage
COPY --from=svelte-builder /usr/src/app/frontend/dist /usr/src/app/frontend/dist

WORKDIR /usr/src/app

# Copy the entire backend source code (Cargo.toml, Cargo.lock, src/)
COPY backend/ ./backend/

WORKDIR /usr/src/app/backend

# Build the Rust backend
# ENV RUSTFLAGS='-C target-feature=-crt-static'
RUN cargo build --release

FROM gcr.io/distroless/cc

WORKDIR /app
# Expose the port the Rust app will use
EXPOSE 8686

COPY --from=rust-builder /usr/src/app/backend/target/release/backend .

# Start the Rust server
CMD ["./backend"]