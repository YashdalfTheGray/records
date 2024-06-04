# Stage 1: Build the frontend
FROM node:lts AS frontbuild

WORKDIR /app/front
COPY front/package.json front/package-lock.json ./
RUN npm install
COPY front/ ./

RUN npm run build


# Stage 2: Build the backend
FROM rust:latest AS backbuild

WORKDIR /app/back
RUN apt-get update && apt-get install -y libssl-dev pkg-config
COPY back/Cargo.toml back/Cargo.lock ./
COPY back/ ./

RUN cargo build --release

# Final stage: Serve the application
FROM debian:buster-slim

WORKDIR /app
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=frontbuild /app/front/dist /app/dist
COPY --from=backbuild /app/back/target/release/record_keeper .

EXPOSE 8000

CMD ["./back"]
