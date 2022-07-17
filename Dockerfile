FROM rust:latest

COPY ./json_generator .

RUN cargo build --release

CMD ["./target/release/json_generator"]
