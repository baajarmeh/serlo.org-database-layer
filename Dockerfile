FROM rust:1.49 as build
WORKDIR /usr/src/app
COPY src src
COPY Cargo.lock .
COPY Cargo.toml .
COPY sqlx-data.json .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/src/app
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/app/target/release/serlo-org-database-layer .
CMD ["./serlo-org-database-layer"]
EXPOSE 8080
