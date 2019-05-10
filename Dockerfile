FROM rust:1.34.1

WORKDIR /usr/src/emtm
COPY . .
RUN cargo install --path

CMD ["emtm"]

