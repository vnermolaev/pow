FROM rust

WORKDIR /usr/src/client

COPY . .

RUN cargo install --bin client --path .

CMD ["client"]