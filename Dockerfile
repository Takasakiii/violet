FROM rust:1.52.1 as builder

COPY . /builder/
WORKDIR /builder/

RUN cargo build --release

FROM ubuntu:20.04
RUN apt update && apt install -y openssl libssl-dev curl
COPY --from=builder /builder/target/release/violet /app/violet
WORKDIR /app
RUN chmod +x violet

EXPOSE 3000

ENTRYPOINT [ "./violet" ]
