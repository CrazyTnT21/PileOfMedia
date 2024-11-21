FROM rust:alpine

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY . .
RUN cargo build -r

FROM alpine
RUN mkdir /content

WORKDIR /app

COPY --from=0 /app/target/release/my_collection /

EXPOSE 3000

CMD ["/my_collection"]

