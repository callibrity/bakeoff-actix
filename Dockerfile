FROM rust:1.64.0

WORKDIR /src
COPY . .

RUN cargo install --path .

CMD ["bakeoff-rust"]
