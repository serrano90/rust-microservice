FROM rust:1.40

RUN rustup default nightly

WORKDIR /usr/src/customer
COPY . .

RUN cargo install --path .

CMD ["customer"]