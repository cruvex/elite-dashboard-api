FROM rust:slim AS build

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

COPY ./src ./src
COPY ./migrations ./migrations
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=build /app/target/x86_64-unknown-linux-musl/release/elite-dashboard-api /app

EXPOSE 8089

ENTRYPOINT ["/app"]