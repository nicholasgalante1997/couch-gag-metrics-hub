FROM rust:1.31

RUN mkdir -p /couch-gag/metrics-hub 

WORKDIR /couch-gag/metrics-hub

COPY . .

RUN cargo build --release

CMD ["./target/release/couch-gag-metrics-hub"]