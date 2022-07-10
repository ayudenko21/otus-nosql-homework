FROM rust:latest

COPY ./json_generator .

RUN cargo build --release --bin redis_writer

CMD ["./target/release/redis_writer"]
