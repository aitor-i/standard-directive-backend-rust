FROM rust:latest

WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
CMD ["standard-directive-n2-backend"]
