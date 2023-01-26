FROM rust

WORKDIR /usr/src/server

COPY . .

EXPOSE 7845

RUN cargo install --bin server --path .

CMD ["server"]