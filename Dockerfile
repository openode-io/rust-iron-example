FROM rust:1.23.0

WORKDIR /usr/src/myapp
COPY . .
ENV PORT=80

RUN cargo install --force
RUN cargo build --release

CMD ["myapp"]
