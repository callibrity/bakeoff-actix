FROM rust:1.64.0

WORKDIR /src
COPY . .

RUN cargo install --path .

RUN cargo install diesel_cli --no-default-features --features postgres

ENTRYPOINT ./docker-entrypoint.sh