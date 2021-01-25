FROM rust:1.40

RUN rustup default nightly

WORKDIR /usr/src/resort
COPY . .

RUN cargo install --path .

CMD ["resort"]