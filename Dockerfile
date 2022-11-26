FROM rust:1.65.0

RUN mkdir -p /couch-gag/metrics-hub 

WORKDIR /couch-gag/metrics-hub

COPY ./src/ ./src/
COPY ./.env .
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./README.md .

RUN cargo build --release

EXPOSE 7878

CMD ["./target/release/couch-gag-metrics-hub"]