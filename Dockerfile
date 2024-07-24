FROM rust:1.74.1

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

WORKDIR /app
COPY . .

EXPOSE 8080

RUN rustup default nightly
RUN cargo install cargo-watch

CMD ["sh", "-c", "cargo watch -x run > cargo.log 2>&1"]

