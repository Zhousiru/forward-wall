FROM rust:latest as builder
WORKDIR /build
COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build -r


FROM debian:stable-slim
COPY --from=builder /build/target/release/forward-wall /app/forward-wall
WORKDIR /app
EXPOSE 3000/tcp
ENTRYPOINT ["./forward-wall"]
